use std::fmt;

use ::image::RgbaImage;
use iced::widget::image;

use super::geometry::{centered_square, clamp_rect, export_crop};
use super::types::{
    ImageCropRect, ImageCropResult, ImageCropShape, ImageCropStatus, ImageCropperAction,
    ImageCropperSource,
};

const ZOOM_MIN: f32 = 1.0;
const ZOOM_MAX: f32 = 4.0;

#[derive(Clone)]
struct LoadedImage {
    name: Option<String>,
    mime: Option<String>,
    handle: image::Handle,
    rgba: RgbaImage,
}

impl fmt::Debug for LoadedImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadedImage")
            .field("name", &self.name)
            .field("mime", &self.mime)
            .field("width", &self.rgba.width())
            .field("height", &self.rgba.height())
            .finish()
    }
}

impl LoadedImage {
    fn from_source(source: ImageCropperSource) -> Result<Self, String> {
        if let Some(mime) = source.mime.as_deref()
            && !is_supported_mime(mime)
        {
            return Err(format!("unsupported image mime: {mime}"));
        }

        let image = ::image::load_from_memory(&source.bytes)
            .map_err(|error| format!("failed to decode image: {error}"))?;

        Ok(Self {
            name: source.name,
            mime: source.mime,
            handle: image::Handle::from_bytes(source.bytes),
            rgba: image.to_rgba8(),
        })
    }

    fn width(&self) -> u32 {
        self.rgba.width()
    }

    fn height(&self) -> u32 {
        self.rgba.height()
    }
}

#[derive(Clone, Debug)]
pub struct ImageCropperState {
    committed: Option<LoadedImage>,
    editing: Option<LoadedImage>,
    pub open: bool,
    pub zoom: f32,
    pub crop_rect: Option<ImageCropRect>,
    pub status: ImageCropStatus,
    pub latest_result: Option<ImageCropResult>,
}

impl Default for ImageCropperState {
    fn default() -> Self {
        Self {
            committed: None,
            editing: None,
            open: false,
            zoom: ZOOM_MIN,
            crop_rect: None,
            status: ImageCropStatus::Idle,
            latest_result: None,
        }
    }
}

impl ImageCropperState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn has_image(&self) -> bool {
        self.committed.is_some()
    }

    #[must_use]
    pub fn preview_handle(&self) -> Option<image::Handle> {
        self.committed.as_ref().map(|image| image.handle.clone())
    }

    #[must_use]
    pub fn editor_handle(&self) -> Option<image::Handle> {
        self.editing.as_ref().map(|image| image.handle.clone())
    }

    #[must_use]
    pub fn editor_dimensions(&self) -> Option<(u32, u32)> {
        self.editing
            .as_ref()
            .map(|image| (image.width(), image.height()))
    }

    pub fn apply(
        &mut self,
        action: ImageCropperAction,
        shape: ImageCropShape,
    ) -> Option<ImageCropResult> {
        match action {
            ImageCropperAction::OpenEditor => {
                if let Some(image) = self.committed.clone() {
                    self.editing = Some(image);
                    self.open = true;
                    self.zoom = ZOOM_MIN;
                    self.crop_rect = self
                        .crop_rect
                        .map(|rect| clamp_rect(rect, self.editor_dimensions().unwrap()))
                        .or_else(|| self.default_crop_rect());
                }
                None
            }
            ImageCropperAction::CloseEditor | ImageCropperAction::CropCancelled => {
                self.open = false;
                self.editing = None;
                self.zoom = ZOOM_MIN;
                None
            }
            ImageCropperAction::SetExternalSource(source) => {
                match LoadedImage::from_source(source) {
                    Ok(image) => {
                        self.committed = Some(image);
                        self.status = ImageCropStatus::Ready;
                        self.latest_result = None;
                        self.crop_rect = self.default_crop_rect();
                    }
                    Err(error) => self.status = ImageCropStatus::DecodeError(error),
                }
                None
            }
            ImageCropperAction::FileAccepted(source) => {
                if let Some(mime) = source.mime.as_deref()
                    && !is_supported_mime(mime)
                {
                    self.status = ImageCropStatus::UnsupportedFile {
                        name: source.name.clone(),
                        mime: source.mime.clone(),
                    };
                    return None;
                }

                match LoadedImage::from_source(source) {
                    Ok(image) => {
                        self.editing = Some(image);
                        self.crop_rect = self.default_crop_rect();
                        self.zoom = ZOOM_MIN;
                        self.open = true;
                        self.status = ImageCropStatus::Ready;
                    }
                    Err(error) => self.status = ImageCropStatus::DecodeError(error),
                }
                None
            }
            ImageCropperAction::UnsupportedFile { name, mime } => {
                self.status = ImageCropStatus::UnsupportedFile { name, mime };
                None
            }
            ImageCropperAction::CropRectChanged(rect) => {
                if let Some((width, height)) = self.editor_dimensions() {
                    self.crop_rect = Some(clamp_rect(rect, (width, height)));
                }
                None
            }
            ImageCropperAction::ZoomChanged(zoom) => {
                self.zoom = zoom.clamp(ZOOM_MIN, ZOOM_MAX);
                None
            }
            ImageCropperAction::CropConfirmed => {
                let editing = self.editing.clone()?;
                let rect = self
                    .crop_rect
                    .unwrap_or_else(|| centered_square(editing.width(), editing.height()));
                match export_crop(&editing.rgba, rect, shape) {
                    Ok(result) => {
                        let handle = image::Handle::from_bytes(result.png_bytes.clone());
                        let rgba = ::image::load_from_memory(&result.png_bytes)
                            .map(|image| image.to_rgba8())
                            .unwrap_or_else(|_| editing.rgba.clone());
                        self.committed = Some(LoadedImage {
                            name: editing.name.clone(),
                            mime: Some(String::from("image/png")),
                            handle,
                            rgba,
                        });
                        self.latest_result = Some(result.clone());
                        self.editing = None;
                        self.open = false;
                        self.zoom = ZOOM_MIN;
                        self.status = ImageCropStatus::Ready;
                        Some(result)
                    }
                    Err(error) => {
                        self.status = ImageCropStatus::DecodeError(error);
                        None
                    }
                }
            }
            ImageCropperAction::PickerRequested => None,
        }
    }

    fn default_crop_rect(&self) -> Option<ImageCropRect> {
        let image = self.editing.as_ref().or(self.committed.as_ref())?;
        Some(centered_square(image.width(), image.height()))
    }
}

fn is_supported_mime(mime: &str) -> bool {
    matches!(
        mime,
        "image/apng" | "image/avif" | "image/gif" | "image/jpeg" | "image/png" | "image/webp"
    )
}
