use image::{io::Reader as ImageReader, ImageBuffer, Luma};
use na::DMatrix;

fn create_lines(x: u32, y: u32, greyvalue: u8) -> Vec<(i32, i32)> {
    vec![(50, 50)]
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
    let width = hough_space.nrows();
    let height = hough_space.ncols();

    let mut image_buf = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let grey_val = hough_space[(x, y)] as f32 / 255.0;
            let pixel = image::Luma([grey_val as u8]);
            image_buf[(x as u32, (height - y - 1) as u32)] = pixel;
        }
    }

    image_buf.save(filename).unwrap();
}

fn main() {
    // load the image and convert it to grayscaley
    let image = ImageReader::open("test.jpg").unwrap().decode().unwrap();
    let image = image.to_luma8();

    let hough_space = hough_transform(&image, 5);
    save_houghspace(&hough_space, "space.jpg");

    println!("Loaded");
}
