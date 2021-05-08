use image::{io::Reader as ImageReader, ImageBuffer, Luma};

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

fn hough_transform(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    threshold: u8,
) -> Result<na::DMatrix<u32>, ()> {

    let hough_space = na::Matrix::zeros();
    
    let _ = image
        .enumerate_pixels()
        .take_while(|pixel| pixel.2[0] > threshold)
        .flat_map(|pixel| create_lines(pixel.0, pixel.1, pixel.2[0]))
        .for_each(|(rho, theta)| hough_space[rho][theta]+=1);

    Ok(hough_space)
}

fn main() {
    // load the image and convert it to grayscaley
    let image = ImageReader::open("test.jpg").unwrap().decode().unwrap();
    let image = image.to_luma8();

    hough_transform(&image, 5).unwrap();

    println!("Loaded");
}
