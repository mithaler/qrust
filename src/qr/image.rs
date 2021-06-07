use crate::qr::pattern::QRCode;
use crate::qr::Error;
use image::{Rgb, RgbImage};
use std::path::Path;

const PIXELS_PER_MODULE: u32 = 4;
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);

fn modules_to_buffer(code: &QRCode) -> RgbImage {
    let side_length = PIXELS_PER_MODULE * code.version.modules_per_side() as u32;
    let mut img = RgbImage::new(side_length, side_length);
    for (x, row) in code.rows.iter().enumerate() {
        for (y, module) in row.iter().enumerate() {
            img.put_pixel(
                x as u32,
                y as u32,
                if module.black() { BLACK } else { WHITE },
            )
        }
    }
    img
}

fn save_image(img: &RgbImage, path: &Path) -> Result<(), Error> {
    img.save(path).map_err(|e| e.to_string().into())
}

pub fn save_qrcode(code: &QRCode, path: &Path) -> Result<(), Error> {
    let buffer = modules_to_buffer(code);
    save_image(&buffer, path)
}
