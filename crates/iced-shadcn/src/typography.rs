use iced::Font;
use iced::font::Weight;
use iced::widget::Text;
use iced::widget::text::{Alignment, IntoFragment, LineHeight, Style, Wrapping};

use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_text};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAs {
    Span,
    Div,
    Label,
    P,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingAs {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextSize {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWeight {
    Light,
    Regular,
    Medium,
    Bold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWrap {
    Wrap,
    NoWrap,
    Pretty,
    Balance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeadingTrim {
    Normal,
    Start,
    End,
    Both,
}

#[derive(Clone, Copy, Debug)]
pub struct TextProps {
    pub as_tag: TextAs,
    pub size: TextSize,
    pub weight: TextWeight,
    pub align: TextAlign,
    pub wrap: TextWrap,
    pub trim: LeadingTrim,
    pub truncate: bool,
    pub color: AccentColor,
    pub high_contrast: bool,
}

impl Default for TextProps {
    fn default() -> Self {
        Self {
            as_tag: TextAs::Span,
            size: TextSize::Three,
            weight: TextWeight::Regular,
            align: TextAlign::Left,
            wrap: TextWrap::Wrap,
            trim: LeadingTrim::Normal,
            truncate: false,
            color: AccentColor::Gray,
            high_contrast: false,
        }
    }
}

impl TextProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_tag(mut self, as_tag: TextAs) -> Self {
        self.as_tag = as_tag;
        self
    }

    pub fn size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn trim(mut self, trim: LeadingTrim) -> Self {
        self.trim = trim;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HeadingProps {
    pub as_tag: HeadingAs,
    pub size: TextSize,
    pub weight: TextWeight,
    pub align: TextAlign,
    pub wrap: TextWrap,
    pub trim: LeadingTrim,
    pub truncate: bool,
    pub color: AccentColor,
    pub high_contrast: bool,
}

impl Default for HeadingProps {
    fn default() -> Self {
        Self {
            as_tag: HeadingAs::H1,
            size: TextSize::Six,
            weight: TextWeight::Bold,
            align: TextAlign::Left,
            wrap: TextWrap::Wrap,
            trim: LeadingTrim::Normal,
            truncate: false,
            color: AccentColor::Gray,
            high_contrast: false,
        }
    }
}

impl HeadingProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_tag(mut self, as_tag: HeadingAs) -> Self {
        self.as_tag = as_tag;
        self
    }

    pub fn size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn trim(mut self, trim: LeadingTrim) -> Self {
        self.trim = trim;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

fn size_to_px(size: TextSize) -> u32 {
    match size {
        TextSize::One => 12,
        TextSize::Two => 14,
        TextSize::Three => 16,
        TextSize::Four => 18,
        TextSize::Five => 20,
        TextSize::Six => 24,
        TextSize::Seven => 28,
        TextSize::Eight => 35,
        TextSize::Nine => 60,
    }
}

fn line_height_text(size: TextSize) -> LineHeight {
    let px = match size {
        TextSize::One => 16.0,
        TextSize::Two => 20.0,
        TextSize::Three => 24.0,
        TextSize::Four => 26.0,
        TextSize::Five => 28.0,
        TextSize::Six => 30.0,
        TextSize::Seven => 36.0,
        TextSize::Eight => 40.0,
        TextSize::Nine => 60.0,
    };
    LineHeight::Absolute(px.into())
}

fn line_height_heading(size: TextSize) -> LineHeight {
    let px = match size {
        TextSize::One => 16.0,
        TextSize::Two => 18.0,
        TextSize::Three => 22.0,
        TextSize::Four => 24.0,
        TextSize::Five => 26.0,
        TextSize::Six => 30.0,
        TextSize::Seven => 36.0,
        TextSize::Eight => 40.0,
        TextSize::Nine => 60.0,
    };
    LineHeight::Absolute(px.into())
}

fn weight_to_font(weight: TextWeight) -> Font {
    let weight = match weight {
        TextWeight::Light => Weight::Light,
        TextWeight::Regular => Weight::Normal,
        TextWeight::Medium => Weight::Medium,
        TextWeight::Bold => Weight::Bold,
    };

    Font {
        weight,
        ..Font::DEFAULT
    }
}

fn align_to_iced(align: TextAlign) -> Alignment {
    match align {
        TextAlign::Left => Alignment::Left,
        TextAlign::Center => Alignment::Center,
        TextAlign::Right => Alignment::Right,
        TextAlign::Justify => Alignment::Justified,
    }
}

fn wrap_to_iced(wrap: TextWrap, truncate: bool) -> Wrapping {
    if truncate || matches!(wrap, TextWrap::NoWrap) {
        Wrapping::None
    } else {
        Wrapping::Word
    }
}

pub fn text<'a>(content: impl IntoFragment<'a>, props: TextProps, theme: &Theme) -> Text<'a> {
    let mut color = accent_text(&theme.palette, props.color);
    if props.high_contrast {
        color = theme.palette.foreground;
    }

    Text::new(content)
        .size(size_to_px(props.size))
        .line_height(line_height_text(props.size))
        .font(weight_to_font(props.weight))
        .align_x(align_to_iced(props.align))
        .wrapping(wrap_to_iced(props.wrap, props.truncate))
        .style(move |_theme| Style { color: Some(color) })
}

pub fn heading<'a>(content: impl IntoFragment<'a>, props: HeadingProps, theme: &Theme) -> Text<'a> {
    let mut color = accent_text(&theme.palette, props.color);
    if props.high_contrast {
        color = theme.palette.foreground;
    }

    Text::new(content)
        .size(size_to_px(props.size))
        .line_height(line_height_heading(props.size))
        .font(weight_to_font(props.weight))
        .align_x(align_to_iced(props.align))
        .wrapping(wrap_to_iced(props.wrap, props.truncate))
        .style(move |_theme| Style { color: Some(color) })
}
