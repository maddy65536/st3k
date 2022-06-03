use image;

fn main() {
    println!("Hello, world!");
    let img: image::RgbImage = image::ImageBuffer::new(512,512);

    img.save("test.png");
}
