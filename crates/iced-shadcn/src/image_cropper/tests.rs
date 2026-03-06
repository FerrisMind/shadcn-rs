use ::image::ImageEncoder;
use ::image::{Rgba, RgbaImage};

use super::geometry::{clamp_rect, export_crop, move_rect};
use super::{
    ImageCropRect, ImageCropShape, ImageCropStatus, ImageCropperAction, ImageCropperSource,
    ImageCropperState,
};

fn sample_png(width: u32, height: u32) -> Vec<u8> {
    let mut image = RgbaImage::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([(x % 255) as u8, (y % 255) as u8, 120, 255]);
    }
    let mut bytes = Vec::new();
    ::image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            ::image::ExtendedColorType::Rgba8,
        )
        .expect("png");
    bytes
}

#[test]
fn moving_crop_rect_stays_in_bounds() {
    let rect = ImageCropRect::new(10, 10, 80, 80);
    let moved = move_rect(rect, -40, 500, (100, 100));
    assert_eq!(moved, ImageCropRect::new(0, 20, 80, 80));
}

#[test]
fn clamp_rect_preserves_square_ratio() {
    let rect = clamp_rect(ImageCropRect::new(20, 30, 120, 60), (100, 100));
    assert_eq!(rect.width, rect.height);
    assert!(rect.x + rect.width <= 100);
    assert!(rect.y + rect.height <= 100);
}

#[test]
fn rect_export_returns_expected_dimensions() {
    let rgba = ::image::load_from_memory(&sample_png(128, 96))
        .expect("decode")
        .to_rgba8();
    let result = export_crop(
        &rgba,
        ImageCropRect::new(16, 10, 40, 40),
        ImageCropShape::Rect,
    )
    .expect("crop");
    assert_eq!(result.output_width, 40);
    assert_eq!(result.output_height, 40);
    let decoded = ::image::load_from_memory(&result.png_bytes).expect("decode result");
    assert_eq!(decoded.width(), 40);
    assert_eq!(decoded.height(), 40);
}

#[test]
fn round_export_transparent_corners() {
    let rgba = ::image::load_from_memory(&sample_png(128, 128))
        .expect("decode")
        .to_rgba8();
    let result = export_crop(
        &rgba,
        ImageCropRect::new(16, 16, 64, 64),
        ImageCropShape::Round,
    )
    .expect("crop");
    let decoded = ::image::load_from_memory(&result.png_bytes)
        .expect("decode result")
        .to_rgba8();
    assert_eq!(decoded.get_pixel(0, 0)[3], 0);
    assert_eq!(decoded.get_pixel(63, 0)[3], 0);
    assert_eq!(decoded.get_pixel(0, 63)[3], 0);
    assert_eq!(decoded.get_pixel(32, 32)[3], 255);
}

#[test]
fn file_accept_opens_dialog() {
    let mut state = ImageCropperState::new();
    let source = ImageCropperSource::new(sample_png(96, 96)).mime("image/png");
    state.apply(
        ImageCropperAction::FileAccepted(source),
        ImageCropShape::Round,
    );
    assert!(state.open);
    assert!(state.editor_handle().is_some());
    assert!(state.crop_rect.is_some());
}

#[test]
fn cancel_restores_previous_committed_preview() {
    let mut state = ImageCropperState::new();
    state.apply(
        ImageCropperAction::SetExternalSource(
            ImageCropperSource::new(sample_png(64, 64)).mime("image/png"),
        ),
        ImageCropShape::Round,
    );
    let preview = state.preview_handle();
    state.apply(
        ImageCropperAction::FileAccepted(
            ImageCropperSource::new(sample_png(96, 96)).mime("image/png"),
        ),
        ImageCropShape::Round,
    );
    state.apply(ImageCropperAction::CropCancelled, ImageCropShape::Round);
    assert!(!state.open);
    assert!(state.preview_handle().is_some());
    assert_eq!(preview.is_some(), state.preview_handle().is_some());
}

#[test]
fn confirm_crop_commits_preview_and_result() {
    let mut state = ImageCropperState::new();
    state.apply(
        ImageCropperAction::FileAccepted(
            ImageCropperSource::new(sample_png(96, 128)).mime("image/png"),
        ),
        ImageCropShape::Rect,
    );
    state.apply(
        ImageCropperAction::CropRectChanged(ImageCropRect::new(8, 16, 48, 48)),
        ImageCropShape::Rect,
    );
    let result = state
        .apply(ImageCropperAction::CropConfirmed, ImageCropShape::Rect)
        .expect("result");
    assert!(!state.open);
    assert!(state.preview_handle().is_some());
    assert_eq!(state.latest_result.as_ref(), Some(&result));
    assert_eq!(result.output_width, 48);
}

#[test]
fn unsupported_svg_updates_status() {
    let mut state = ImageCropperState::new();
    state.apply(
        ImageCropperAction::FileAccepted(
            ImageCropperSource::new(b"<svg/>".to_vec()).mime("image/svg+xml"),
        ),
        ImageCropShape::Round,
    );
    assert!(matches!(
        state.status,
        ImageCropStatus::UnsupportedFile { .. }
    ));
}
