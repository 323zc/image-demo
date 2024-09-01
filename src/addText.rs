use image::{open, GenericImageView, ImageReader, Rgba};
use imageproc::drawing::draw_text;
use ab_glyph::{FontVec, PxScale};
fn main() {
    let image = open("test_data/pikachu.jpg").unwrap();
    // let image = ImageReader::open("test_data/pikachu.jpg")
    //     .unwrap()
    //     .decode()
    //     .unwrap();
    let (dimension_x, dimension_y) = image.dimensions();
    let font_vec: Vec<u8> = Vec::from(include_bytes!("Matemasie-Regular.ttf") as &[u8]);
    let font = FontVec::try_from_vec(font_vec).unwrap();

    let scale = PxScale::from(36.0);
    let text = "💖💖Pikachu💖💖";
    // 创建一个与原始图像大小相同的新空图像
    let new_image = draw_text(
        &image,
        Rgba([255, 192, 203, 255]),
        (dimension_x / 2).try_into().unwrap(),
        (dimension_y / 8).try_into().unwrap(),
        scale,
        &font,
        &text,
    );
    new_image.save("test_data/pikachu_new.png").unwrap();
}
