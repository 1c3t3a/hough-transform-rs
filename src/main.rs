use image::{ImageBuffer, Luma, io::Reader as ImageReader};

fn hough_transform(image: &ImageBuffer<Luma<u8>, Vec<u8>>, threshold: u32) -> Result<(), ()> {
    
    Ok(())
}

fn main() {
    // load the image -> convert it to grayscale and safe it (for now, later we want to dynamically change it)
    let image = ImageReader::open("test.jpg").unwrap().decode().unwrap();
    let image = image.to_luma8();
    
    hough_transform(&image, 5).unwrap();
    
    println!("Loaded");
}
