use image::{io::Reader as ImageReader, ImageBuffer, Luma, Rgba};
use imageproc::drawing::{draw_line_segment_mut, Canvas};
use na::DMatrix;

#[allow(clippy::float_cmp)]

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

#[inline]
fn create_lines(x: u32, y: u32) -> Vec<(usize, f64)> {
    let mut vec = Vec::new();

    for i in 0..180 {
        let tetha = i as f64;
        let x = x as f64;
        let y = y as f64;

        let scale = std::f64::consts::PI / 180.0;
        let rho = x * (scale * tetha).cos() + y * (scale * tetha).sin();
        vec.push((i as usize, rho));
    }
    vec
}

#[inline]
fn calculate_max_rho_value(width: u32, height: u32) -> f32 {
    ((width as f32).hypot(height as f32)).ceil()
}

#[inline]
fn scale_rho(rho: f64, max_rho_value: f32) -> u32 {
    ((rho * 0.5).round() + 0.5 * max_rho_value as f64).round() as u32
}

fn save_houghspace(hough_space: &DMatrix<u32>, filename: &str) {
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

    image_buf.save(filename).unwrap();
}

fn transform_to_image_space(
    hough_space: &DMatrix<u32>,
    threshold: u32,
    max_rho_value: f32,
) -> Vec<(f32, f32)> {
    let mut vec = Vec::new();

    let width = hough_space.nrows();
    let height = hough_space.ncols();

    for rho_scaled in 0..height {
        for tetha in 0..width {
            if hough_space[(tetha, rho_scaled)] >= threshold {
                println!(
                    "value {} in hough_space will be transformed back",
                    hough_space[(tetha, rho_scaled)]
                );
                let rho = (rho_scaled as f32 - 0.5 * max_rho_value) * 2.0;

                vec.push((tetha as f32, rho))
            }
        }
    }

    vec
}

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

fn main() {
    // load the image and convert it to grayscaley
    let mut image = ImageReader::open("data/nicht-parallelogramm.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let image2 = image.to_luma8();

    let hough_space = hough_transform(&image2, 250);
    save_houghspace(&hough_space, "data/space.jpeg");

    let max_rho = calculate_max_rho_value(image.width(), image.height());
    let lines = transform_to_image_space(&hough_space, 63, max_rho);

    for line in lines {
        draw_line_in_image(
            &mut image,
            line.0,
            line.1,
            Rgba([255_u8, 0_u8, 0_u8, 255_u8]),
        );
    }

    image.save("data/detected.jpeg").unwrap();
    println!("Loaded");
}
