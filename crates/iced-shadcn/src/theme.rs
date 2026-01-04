use crate::tokens::{Palette, Radius, Spacing};

#[derive(Clone, Debug)]
pub struct Theme {
    pub palette: Palette,
    pub radius: Radius,
    pub spacing: Spacing,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            palette: Palette::light(),
            radius: Radius::default(),
            spacing: Spacing::default(),
        }
    }

    pub fn dark() -> Self {
        Self {
            palette: Palette::dark(),
            radius: Radius::default(),
            spacing: Spacing::default(),
        }
    }

    pub fn with_palette(palette: Palette) -> Self {
        Self {
            palette,
            radius: Radius::default(),
            spacing: Spacing::default(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}
