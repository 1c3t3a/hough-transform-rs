//! This is a demonstration of a Hough-Transformation written for a coursework in the class
//! Computer-Graphics and Image Processing. The programm takes an image as input and calculates
//! the corresponding Hough-Space. The Hough-Space will be visualized and saved in form of a
//! greyscale image. Afterwards the Hough-Space is filtered via a threshold value provided by
//! the user and the corresponding lines are being transformed back into the original image space
//! and drawn into the final image with an overlay. The final image is then also safed.
//!

use clap::{AppSettings, Clap};

use image::{io::Reader as ImageReader, ImageBuffer, Luma, Rgba};
use imageproc::drawing::{draw_line_segment_mut, Canvas};
use na::DMatrix;

#[allow(clippy::clippy::float_cmp)]

/// Calculates the hough transformation for a given `image::GreyImage`. The Hough space is returned
/// in the form of a `na::DMatrix` where the columns stand for the rho value (scaled) and the rows
/// stand for the angle theta, ranging from 0-180 degree. A value for a given row and column index
/// represents how many lines "voted" for this combination of rho and theta.
fn hough_transform(image: &ImageBuffer<Luma<u8>, Vec<u8>>, threshold: u8) -> DMatrix<u32> {
    let max_rho = calculate_max_rho_value(image.width(), image.height());
    // allocate the hough space, the x axis is 180 (degree) wide, the y axis ranges from zero
    // to the maximal rho value (calculated by `calculate_max_rho_value`)
    let mut hough_space = DMatrix::<u32>::zeros(180, max_rho.round() as usize);

    image
        .enumerate_pixels()
        .filter(|pixel| pixel.2[0] >= threshold)
        .flat_map(|pixel| create_lines(pixel.0, pixel.1))
        .for_each(|(theta, rho)| {
            let scaled_rho = scale_rho(rho, max_rho);

            hough_space[(theta, scaled_rho as usize)] += 1
        });

    hough_space
}

/// Generates a parametric family of lines for a given point. This involves generating
/// the parameters rho and theta for any possible line throgh the given point in the image.
/// All possible combinations are the returned in the form of a vector.
#[inline]
fn create_lines(x: u32, y: u32) -> Vec<(usize, f64)> {
    (0..180)
        .map(|theta| {
            let x = x as f64;
            let y = y as f64;

            let theta_rad = theta as f64 * std::f64::consts::PI / 180.0;
            let rho = x * theta_rad.cos() + y * theta_rad.sin();
            (theta as usize, rho)
        })
        .collect()
}

/// Calculates the maximum value for rho which is the length of the diagonal line through
/// the image.
#[inline]
fn calculate_max_rho_value(width: u32, height: u32) -> f32 {
    ((width as f32).hypot(height as f32)).ceil()
}

/// Scale rho from an f64 to a u32 value, as the hough space is only indexable via an unsigned
/// integer value.
#[inline]
fn scale_rho(rho: f64, max_rho_value: f32) -> u32 {
    ((rho * 0.5).round() + 0.5 * max_rho_value as f64).round() as u32
}

/// Save the hough space to an image, this involves calculating a greyvalue for every point in the hough
/// space. The grey value is normalized via the maximal value in the houg space.
fn save_houghspace(hough_space: &DMatrix<u32>, filename: &str) -> Result<(), image::ImageError> {
    let max_value = hough_space.max();

    println!("Max value in Hough-Space is: {}", max_value);
    let width = hough_space.nrows();
    let height = hough_space.ncols();

    let mut image_buf = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            if hough_space[(x, y)] == max_value {
                println!("Theta: {}, Rho: {}", x, y);
            }
            let grey_val = na::min(
                ((hough_space[(x, y)] as f64) * 255.0 / (max_value as f64)).round() as u32,
                255,
            ) as u8;
            let pixel = image::Luma([grey_val]);
            image_buf[(x as u32, (height - y - 1) as u32)] = pixel;
        }
    }

    image_buf.save(filename)
}

/// Transforms the Hough-Space back to the image space by calculating rho and theta for every point
/// in it. Rho and theta could then be used to draw a line into the original image.
fn transform_to_image_space(
    hough_space: &DMatrix<u32>,
    threshold: u32,
    max_rho_value: f32,
) -> Vec<(f32, f32)> {
    let mut vec = Vec::new();

    let width = hough_space.nrows();
    let height = hough_space.ncols();

    for rho_scaled in 0..height {
        for theta in 0..width {
            if hough_space[(theta, rho_scaled)] >= threshold {
                println!(
                    "value {} in hough_space will be transformed back",
                    hough_space[(theta, rho_scaled)]
                );
                let rho = (rho_scaled as f32 - 0.5 * max_rho_value) * 2.0;

                vec.push((theta as f32, rho))
            }
        }
    }

    vec
}

/// Draws a line into the given image for a the given parameters rho and theta (polar coordinates).
/// Attention: This mutates the imaga and therfore takes a mutable reference of it.
fn draw_line_in_image<C>(image: &mut C, theta: f32, rho: f32, color: C::Pixel)
where
    C: Canvas,
    C::Pixel: 'static,
{
    let image_width = image.width() as f32;
    let theta_rad = theta * std::f32::consts::PI / 180.0;
    let y_one: (f32, f32);
    let y_end: (f32, f32);
    // special case that line is parrallel to the y-axis
    if theta == 0.0 || theta == 180.0f32 {
        y_one = (rho.abs(), 1.0);
        y_end = (rho.abs(), image_width);
    } else if theta == 90.0 {
        y_one = (0.0, rho.abs());
        y_end = (image_width, rho.abs());
    } else {
        y_one = (1.0, (rho - theta_rad.cos()) / theta_rad.sin());
        y_end = (
            image_width,
            (rho - image_width * theta_rad.cos()) / theta_rad.sin(),
        );
    }

    draw_line_segment_mut(image, y_one, y_end, color);
}

/// A demonstration of a hough transformation written in the rust programming language.
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Bastian Kersting <bastian@cmbt.de>, Tobias Karius <tobias.karius@yahoo.de>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the input image path.
    #[clap(long, short)]
    image_path: String,
    /// Sets the path where an image of the Hough-Space is safed.
    #[clap(long)]
    hough_space_target: String,
    /// Sets the path where the image of the input including an overlay with the lines is safed.
    #[clap(long, short)]
    converted_target: String,
    /// Sets the threshold for filtering the Hough-Space.
    #[clap(long)]
    hough_space_threshold: u32,
}

fn main() {
    // parse the cmd arguments
    let opts: Opts = Opts::parse();

    // load the image and convert it to grayscaley
    let mut image = ImageReader::open(opts.image_path.clone())
        .expect("Error while loading the image")
        .decode()
        .expect("Error while decoding the image");
    let grey_img = image.to_luma8();

    // calculate the hough space for the given image and save it's representation into a file
    // A pixel is analyzed when it's greyvalue is higher than 250
    let hough_space = hough_transform(&grey_img, 250);
    save_houghspace(&hough_space, &opts.hough_space_target).expect("Couldn't save Hough-Space");

    // transform the detected lines back to the image space (using the threshold)
    let max_rho = calculate_max_rho_value(image.width(), image.height());
    let lines = transform_to_image_space(&hough_space, opts.hough_space_threshold, max_rho);

    // draw the lines into the original image and save it
    for line in lines {
        draw_line_in_image(
            &mut image,
            line.0,
            line.1,
            Rgba([255_u8, 0_u8, 0_u8, 255_u8]),
        );
    }

    // finally save the modified image
    image
        .save(opts.converted_target)
        .expect("failed to save result image");
    println!("Done");
}
