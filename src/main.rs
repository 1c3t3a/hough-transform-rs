use image::{ImageBuffer, Luma, Rgba, io::Reader as ImageReader};
use imageproc::drawing::{
    draw_line_segment_mut, Canvas,
};
use na::DMatrix;

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

            hough_space[(theta as usize, scaled_rho as usize)] += 1
        });

    hough_space
}

#[inline]
fn create_lines(x: u32, y: u32) -> Vec<(u32, f64)> {
    let mut vec = Vec::new();

    for i in 0..180 {
        let tetha = i as f64;
        let x = x as f64;
        let y = y as f64;

        let scale = std::f64::consts::PI / 180.0;
        let rho = x * (scale * tetha).cos() + y * (scale * tetha).sin();
        vec.push((i as u32, rho));
    }
    vec
}

#[inline]
fn calculate_max_rho_value(width: u32, height: u32) -> f64 {
    ((width as f64).hypot(height as f64)).ceil()
}

#[inline]
fn scale_rho(rho: f64, max_rho_value: f64) -> u32 {
    ((rho * 0.5).round() + 0.5 * max_rho_value as f64) as u32
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

fn draw_line_in_image<C>(image: &mut C, m: u32, b: u32, color: C::Pixel)
where
    C: Canvas,
    C::Pixel: 'static
{
    let y_one = (1f32, (m + b) as f32);
    let y_end = (image.width() as f32, (m * 180 + b) as f32);

    draw_line_segment_mut(image, y_one, y_end, color);
}

fn main() {
    // load the image and convert it to grayscaley
    let mut image = ImageReader::open("data/test2.JPG")
        .unwrap()
        .decode()
        .unwrap();
    let image2 = image.to_luma8();

    let hough_space = hough_transform(&image2, 250);
    save_houghspace(&hough_space, "data/space.jpeg");

    draw_line_in_image(&mut image, 0, 40, Rgba([255_u8, 0_u8, 0_u8, 255_u8]));

    image.save("data/detected.jpeg").unwrap();
    println!("Loaded");
}
