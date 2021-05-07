use arrayfire::{load_image, save_image, sobel, sqrt, Array};
use image::io::Reader as ImageReader;

fn main() {
    // load the image -> convert it to grayscale and safe it (for now, later we want to dynamically change it)
    let image = ImageReader::open("test.jpg").unwrap().decode().unwrap();
    let image = image.to_luma_alpha8();
    image.save("grey.jpg").unwrap();

    // load the image into an arrayfire::Array and compute the sobel
    let array: Array<u8> = load_image("grey.jpg".to_owned(), false);
    // sobel returns dx and dy result
    let sobel = sobel(&array, 3);

    // save them for demonstration
    save_image("dx.jpg".to_owned(), &sobel.0);
    save_image("dy.jpg".to_owned(), &sobel.1);

    // cast them to f32 -> needed for later computations
    let dx: Array<f32> = sobel.0.cast();
    let dy: Array<f32> = sobel.1.cast();
   
   // the goal is to compute the sobel absolute value by calculating
   // abs = sqrt(dx^2 + dy^2)
   
    // compute dx^2
    let dx2 = dx.clone() * dx;
    save_image("dx2.jpg".to_owned(), &dx2);

    // compute dy^2
    let dy2 = dy.clone() * dy;
    save_image("dy2.jpg".to_owned(), &dy2);

    // now calculate the sqrt, but make sure that now value is 0 by adding 0.001 to every pixel
    let values = vec![0.001f32; dx2.elements()];
    let arr = Array::new(&values, dx2.dims());
    let sum = (dx2 + dy2) + arr;
    save_image("sum.jpg".to_owned(), &sum);
    let sqrt = sqrt(&sum);

    // save the image
    save_image("sobel.jpg".to_owned(), &sqrt);
    println!("Loaded");
}
