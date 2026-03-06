use std::rc::Rc;

use crate::dialog::DialogProps;
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageCropperSource {
    pub bytes: Vec<u8>,
    pub mime: Option<String>,
    pub name: Option<String>,
}

impl ImageCropperSource {
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            mime: None,
            name: None,
        }
    }

    #[must_use]
    pub fn mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = Some(mime.into());
        self
    }

    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageCropShape {
    Round,
    Rect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageCropRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl ImageCropRect {
    #[must_use]
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageCropResult {
    pub png_bytes: Vec<u8>,
    pub crop_rect_px: ImageCropRect,
    pub output_width: u32,
    pub output_height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageCropStatus {
    Idle,
    Ready,
    UnsupportedFile {
        name: Option<String>,
        mime: Option<String>,
    },
    DecodeError(String),
}

#[derive(Clone, Debug)]
pub enum ImageCropperAction {
    OpenEditor,
    CloseEditor,
    SetExternalSource(ImageCropperSource),
    FileAccepted(ImageCropperSource),
    UnsupportedFile {
        name: Option<String>,
        mime: Option<String>,
    },
    CropRectChanged(ImageCropRect),
    ZoomChanged(f32),
    CropConfirmed,
    CropCancelled,
    PickerRequested,
}

#[derive(Clone, Copy, Debug)]
pub struct ImageCropperProps {
    pub shape: ImageCropShape,
    pub dialog: DialogProps,
    pub cropper_height: f32,
    pub preview_size: f32,
    pub zoom_step: f32,
    pub disabled: bool,
}

impl Default for ImageCropperProps {
    fn default() -> Self {
        Self {
            shape: ImageCropShape::Round,
            dialog: DialogProps::new().max_width(720),
            cropper_height: 360.0,
            preview_size: 112.0,
            zoom_step: 0.05,
            disabled: false,
        }
    }
}

impl ImageCropperProps {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn shape(mut self, shape: ImageCropShape) -> Self {
        self.shape = shape;
        self
    }

    #[must_use]
    pub fn dialog(mut self, dialog: DialogProps) -> Self {
        self.dialog = dialog;
        self
    }

    #[must_use]
    pub fn cropper_height(mut self, height: f32) -> Self {
        self.cropper_height = height.max(160.0);
        self
    }

    #[must_use]
    pub fn preview_size(mut self, size: f32) -> Self {
        self.preview_size = size.max(40.0);
        self
    }

    #[must_use]
    pub fn zoom_step(mut self, step: f32) -> Self {
        self.zoom_step = step.clamp(0.01, 0.5);
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct ImageCropperContext<'a, Message> {
    pub props: ImageCropperProps,
    pub state: &'a super::state::ImageCropperState,
    pub theme: &'a Theme,
    pub(crate) on_action: Rc<dyn Fn(ImageCropperAction) -> Message + 'a>,
}

impl<'a, Message> Clone for ImageCropperContext<'a, Message> {
    fn clone(&self) -> Self {
        Self {
            props: self.props,
            state: self.state,
            theme: self.theme,
            on_action: Rc::clone(&self.on_action),
        }
    }
}

impl<'a, Message> ImageCropperContext<'a, Message> {
    pub(crate) fn message(&self, action: ImageCropperAction) -> Message {
        (self.on_action)(action)
    }
}
