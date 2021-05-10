# Hough-transform-rs

This is a demonstration of a Hough-Transformation written for a coursework in the class
Computer-Graphics and Image Processing. The programm takes an image as input and calculates
the corresponding Hough-Space. The Hough-Space will be visualized and saved in form of a
greyscale image. Afterwards the Hough-Space is filtered via a threshold value provided by
the user and the corresponding lines are being transformed back into the original image space
and drawn into the final image with an overlay. The final image is then also safed.

## Installation

This demo requires a working Rust compiler. Installation instructions can be found [here](https://www.rust-lang.org/tools/install).

```
$ git clone https://github.com/1c3t3a/hough-transform-rs.git
$ cd hough-transform-rs
$ cargo build --release
```

## Usage

By running the command `cargo run -- --help` inside the repository folder you can get an overview of all the commands available:

```
USAGE:
    hough-transform-rs --image-path <image-path> --hough-space-target <hough-space-target> --converted-target <converted-target> --edge-threshold <edge-threshold> --hough-space-threshold <hough-space-threshold>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --converted-target <converted-target>
            Sets the path where the image of the input including an overlay with the lines is safed

    -e, --edge-threshold <edge-threshold>
            Sets the threshold greyvalue for when a pixel is part of an edge

        --hough-space-target <hough-space-target>
            Sets the path where an image of the Hough-Space is safed

        --hough-space-threshold <hough-space-threshold>
            Sets the threshold for filtering the Hough-Space

    -i, --image-path <image-path>                          
            Sets the input image path
```
