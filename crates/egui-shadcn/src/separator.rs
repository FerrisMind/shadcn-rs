//! Separator for structuring layouts with shadcn spacing and sizes.
//!
//! # Example
//! ```rust
//! use egui_shadcn::{separator, SeparatorOrientation, SeparatorSize, Theme};
//!
//! fn ui(ui: &mut egui::Ui, theme: &Theme) {
//!     separator(
//!         ui,
//!         theme,
//!         SeparatorOrientation::Horizontal,
//!         SeparatorSize::Md,
//!     );
//! }
//! ```

use crate::theme::Theme;
use crate::tokens::mix;
use egui::{Color32, Response, Sense, Stroke, Ui, Vec2, pos2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeparatorSize {
    Size1,

    Size2,

    Size3,

    #[default]
    Size4,
}

impl SeparatorSize {
    pub fn length_px(self, available: f32) -> f32 {
        match self {
            SeparatorSize::Size1 => 16.0,
            SeparatorSize::Size2 => 24.0,
            SeparatorSize::Size3 => 48.0,
            SeparatorSize::Size4 => available,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SeparatorProps {
    pub orientation: SeparatorOrientation,
    pub size: SeparatorSize,
    pub thickness: f32,
    pub gap: f32,
    pub length: Option<f32>,
    pub color: Option<Color32>,

    pub decorative: bool,

    pub high_contrast: bool,

    pub as_child: bool,
}

impl Default for SeparatorProps {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            size: SeparatorSize::default(),
            thickness: 1.0,
            gap: 0.0,
            length: None,
            color: None,
            decorative: false,
            high_contrast: false,
            as_child: false,
        }
    }
}

impl SeparatorProps {
    pub fn with_orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_size(mut self, size: SeparatorSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }
}

pub fn separator(ui: &mut Ui, theme: &Theme, props: SeparatorProps) -> Response {
    let available = match props.orientation {
        SeparatorOrientation::Horizontal => ui.available_width(),
        SeparatorOrientation::Vertical => ui.available_height(),
    };

    let desired_length = props
        .length
        .unwrap_or_else(|| props.size.length_px(available));

    let size = match props.orientation {
        SeparatorOrientation::Horizontal => {
            Vec2::new(desired_length, props.thickness + 2.0 * props.gap)
        }
        SeparatorOrientation::Vertical => {
            Vec2::new(props.thickness + 2.0 * props.gap, desired_length)
        }
    };

    let (rect, response) = ui.allocate_at_least(size, Sense::hover());

    let stroke_color = if let Some(color) = props.color {
        color
    } else if props.high_contrast {
        mix(theme.palette.border, theme.palette.foreground, 0.2)
    } else {
        theme.palette.border
    };
    let stroke = Stroke::new(props.thickness, stroke_color);

    let (start, end) = match props.orientation {
        SeparatorOrientation::Horizontal => (
            pos2(rect.left(), rect.center().y),
            pos2(rect.right(), rect.center().y),
        ),
        SeparatorOrientation::Vertical => (
            pos2(rect.center().x, rect.top()),
            pos2(rect.center().x, rect.bottom()),
        ),
    };

    ui.painter().line_segment([start, end], stroke);
    response
}
