# Example: ONNX-EfficientTransformer

This project is a simple project with minimal code showing how to use tract to
process an image with EfficientTransformer.

## Input image

We will use a portrait of Grace Hopper in uniform (included in the repository).

```
grace_hopper.jpg: JPEG image data, JFIF standard 1.02, resolution (DPI), density 96x96, segment length 16, baseline, precision 8, 517x606, components 3
```

## Try it

`cargo run` should print a lot of things, and ultimately: `result: Some((11.4773035, 654))`.

This is actually good. It is the rank (654) and a confidence indicator (11.4773035)
of the inferred label.

```
$ cat -n imagenet_slim_labels.txt | grep -C 3 654
   651  megalith
   652  microphone
   653  microwave
   654  military uniform
   655  milk can
   656  minibus
   657  miniskirt
```
