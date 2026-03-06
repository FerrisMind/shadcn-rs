use ::image::ImageEncoder;
use ::image::{Rgba, RgbaImage};

use super::types::{ImageCropRect, ImageCropResult, ImageCropShape};

pub const DEFAULT_CROP_SIZE: u32 = 256;
pub const MIN_CROP_SIZE: u32 = 32;

#[must_use]
pub fn centered_square(width: u32, height: u32) -> ImageCropRect {
    let size = width.min(height).min(DEFAULT_CROP_SIZE.max(MIN_CROP_SIZE));
    ImageCropRect {
        x: (width.saturating_sub(size)) / 2,
        y: (height.saturating_sub(size)) / 2,
        width: size,
        height: size,
    }
}

#[must_use]
pub fn clamp_rect(rect: ImageCropRect, image_size: (u32, u32)) -> ImageCropRect {
    let max_width = image_size.0.max(MIN_CROP_SIZE);
    let max_height = image_size.1.max(MIN_CROP_SIZE);
    let size = rect
        .width
        .min(rect.height)
        .max(MIN_CROP_SIZE)
        .min(max_width)
        .min(max_height);
    let x = rect.x.min(image_size.0.saturating_sub(size));
    let y = rect.y.min(image_size.1.saturating_sub(size));
    ImageCropRect::new(x, y, size, size)
}

#[must_use]
pub fn move_rect(rect: ImageCropRect, dx: i32, dy: i32, image_size: (u32, u32)) -> ImageCropRect {
    let max_x = image_size.0.saturating_sub(rect.width) as i32;
    let max_y = image_size.1.saturating_sub(rect.height) as i32;
    let x = (rect.x as i32 + dx).clamp(0, max_x) as u32;
    let y = (rect.y as i32 + dy).clamp(0, max_y) as u32;
    ImageCropRect::new(x, y, rect.width, rect.height)
}

pub fn export_crop(
    rgba: &RgbaImage,
    rect: ImageCropRect,
    shape: ImageCropShape,
) -> Result<ImageCropResult, String> {
    let rect = clamp_rect(rect, (rgba.width(), rgba.height()));
    let mut crop =
        ::image::imageops::crop_imm(rgba, rect.x, rect.y, rect.width, rect.height).to_image();

    if matches!(shape, ImageCropShape::Round) {
        let radius = crop.width().min(crop.height()) as f32 / 2.0;
        let center = radius;
        for (x, y, pixel) in crop.enumerate_pixels_mut() {
            let dx = x as f32 + 0.5 - center;
            let dy = y as f32 + 0.5 - center;
            if (dx * dx + dy * dy).sqrt() > radius {
                *pixel = Rgba([pixel[0], pixel[1], pixel[2], 0]);
            }
        }
    }

    let mut png_bytes = Vec::new();
    let encoder = ::image::codecs::png::PngEncoder::new(&mut png_bytes);
    encoder
        .write_image(
            crop.as_raw(),
            crop.width(),
            crop.height(),
            ::image::ExtendedColorType::Rgba8,
        )
        .map_err(|error| format!("failed to encode cropped image: {error}"))?;

    Ok(ImageCropResult {
        png_bytes,
        crop_rect_px: rect,
        output_width: crop.width(),
        output_height: crop.height(),
    })
}
