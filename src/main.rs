use image::{open, GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use imageproc;
use ab_glyph::{FontVec, PxScale};
fn main() {
    let mut image = open("test_data/pikachu.jpg").unwrap();
    let (dimension_x, dimension_y) = image.dimensions();
    // 来加载字体
    let font_vec: Vec<u8> = Vec::from(include_bytes!("Matemasie-Regular.ttf") as &[u8]);
    let font = FontVec::try_from_vec(font_vec).unwrap();

    let scale = PxScale::from(36.0);
    let text = "💖💖Pikachu💖💖";
    // 创建一个与原始图像大小相同的新空图像
    let mut text_image:image::ImageBuffer<Rgba<u8>,Vec<u8>>=image::ImageBuffer::new(dimension_x,dimension_y);
    // draw_text：在新图像副本上绘制彩色文字。
    // draw_text_mut：直接在原图像上绘制彩色文字。
     draw_text_mut(
        &mut text_image,
        Rgba([255, 192, 203, 255]),
        (dimension_x / 2).try_into().unwrap(),
        (dimension_y / 8).try_into().unwrap(),
        scale,
        &font,
        &text,
    );
    text_image=imageproc::geometric_transformations::rotate_about_center(&text_image,0.3,imageproc::geometric_transformations::Interpolation::Nearest, Rgba([0,0,0,0]));
    // 将文字图像叠加到目标图像上。
    image::imageops::overlay(&mut image,&text_image,0,0);
    image.save("test_data/new_pikachu_rotate.png").unwrap();
}
