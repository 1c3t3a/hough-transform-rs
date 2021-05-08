use image::{io::Reader as ImageReader, ImageBuffer, Luma};
use na::DMatrix;

//tetha, rho
fn create_lines(x: u32, y: u32, greyvalue: u8) -> Vec<(u32, u32)> {
    let mut vec = Vec::new();

    for i in 0..180 {
        let tetha = i as f32;
        let x = x as f32;
        let y = y as f32;

        let rho = x*tetha.cos() + y*tetha.sin();
        let rho = rho as u32;
        vec.push((i as u32, rho));
    }
    return vec;
}

fn hough_transform(image: &ImageBuffer<Luma<u8>, Vec<u8>>, threshold: u8) -> DMatrix<u32> {
    let mut hough_space = DMatrix::<u32>::zeros(image.height() as usize, image.width() as usize);

    let _ = image
        .enumerate_pixels()
        .take_while(|pixel| pixel.2[0] > threshold)
        .flat_map(|pixel| create_lines(pixel.0, pixel.1, pixel.2[0]))
        .for_each(|(rho, theta)| hough_space[(rho as usize, theta as usize)] += 1);

    hough_space
}

fn save_houghspace(hough_space: &DMatrix<u32>, filename: &str) {
    let max_value = hough_space.max();

    println!("Max value in Hough-Space is: {}", max_value);
    let width = hough_space.nrows();
    let height = hough_space.ncols();

    let mut image_buf = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let grey_val = na::min(
                ((hough_space[(x, y)] as f64) * 255.0 / (max_value as f64)).round() as u32,
                255,
            ) as u8;
            let pixel = image::Luma([grey_val as u8]);
            image_buf[(x as u32, (height - y - 1) as u32)] = pixel;
        }
    }

    image_buf.save(filename).unwrap();
}

fn main() {
    // load the image and convert it to grayscaley
    let image = ImageReader::open("data/test.jpg").unwrap().decode().unwrap();
    let image = image.to_luma8();

    let hough_space = hough_transform(&image, 5);
    save_houghspace(&hough_space, "data/space.jpeg");

    println!("Loaded");
}
