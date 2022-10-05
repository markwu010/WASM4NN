use clap::{Arg, Parser};
use tract_onnx::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Location of the input image
    #[clap(parse(from_os_str), short, long)]
    image_path: std::path::PathBuf,

    /// Location of the input onnx model
    #[clap(parse(from_os_str), short, long)]
    model_path: std::path::PathBuf,
}

const IMAGE_HEIGHT: u32 = 224;
const IMAGE_WIDTH: u32 = 224;

fn main() -> TractResult<()> {
    let args = Args::parse();
    let model = tract_onnx::onnx()
        // load the model
        .model_for_path(&args.model_path)?
        // specify input type and shape
        .with_input_fact(0, f32::fact(&[1, 3, 224, 224]).into())?
        // optimize the model
        .into_optimized()?
        // make the model runnable and fix its inputs and outputs
        .into_runnable()?;

    // open image, resize it and make a Tensor out of it
    let image = image::open(&args.image_path).unwrap().to_rgb8();
    let resized = image::imageops::resize(
        &image,
        IMAGE_HEIGHT,
        IMAGE_WIDTH,
        ::image::imageops::FilterType::Triangle,
    );
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();

    // run the model on the input
    let result = model.run(tvec!(image))?;

    // find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(2..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("This is the top-1 result: {:?}", best);

    Ok(())
}
