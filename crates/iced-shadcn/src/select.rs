use iced::Background;
use iced::border::Border;
use iced::widget::overlay::menu;
use iced::widget::pick_list;
use iced::widget::text;
use iced::{Font, Shadow, Vector};
use lucide_icons::Icon as LucideIcon;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_color, accent_foreground, accent_soft, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectSize {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectTriggerVariant {
    Classic,
    Surface,
    Soft,
    Ghost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectContentVariant {
    Solid,
    Soft,
}

#[derive(Clone, Copy, Debug)]
pub struct SelectProps {
    pub size: SelectSize,
    pub variant: SelectTriggerVariant,
    pub content_variant: SelectContentVariant,
    pub color: AccentColor,
    pub radius: Option<ButtonRadius>,
    pub high_contrast: bool,
}

impl Default for SelectProps {
    fn default() -> Self {
        Self {
            size: SelectSize::Two,
            variant: SelectTriggerVariant::Surface,
            content_variant: SelectContentVariant::Solid,
            color: AccentColor::Gray,
            radius: None,
            high_contrast: false,
        }
    }
}

impl SelectProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SelectTriggerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn content_variant(mut self, content_variant: SelectContentVariant) -> Self {
        self.content_variant = content_variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

impl SelectSize {
    fn padding(self) -> [f32; 2] {
        match self {
            SelectSize::One => [6.0, 10.0],
            SelectSize::Two => [8.0, 12.0],
            SelectSize::Three => [10.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            SelectSize::One => 13,
            SelectSize::Two => 14,
            SelectSize::Three => 16,
        }
    }
}

fn select_radius(theme: &Theme, props: SelectProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.sm,
    }
}

pub fn select<'a, Message: Clone + 'a, T, F>(
    options: &'a [T],
    selected: Option<T>,
    placeholder: &'a str,
    on_select: F,
    props: SelectProps,
    theme: &Theme,
) -> pick_list::PickList<'a, T, &'a [T], T, Message>
where
    T: Clone + Eq + ToString + 'a,
    F: Fn(T) -> Message + 'a,
{
    let theme_for_field = theme.clone();
    let theme_for_menu = theme.clone();
    let handle = select_handle(props.size);

    pick_list::PickList::new(options, selected, on_select)
        .placeholder(placeholder)
        .padding(props.size.padding())
        .text_size(props.size.text_size())
        .handle(handle)
        .style(move |_iced_theme, status| select_style(&theme_for_field, props, status))
        .menu_style(move |_iced_theme| select_menu_style(&theme_for_menu, props))
}

fn select_style(theme: &Theme, props: SelectProps, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.palette;
    let radius = select_radius(theme, props);
    let text_color = palette.foreground;
    let soft_bg = accent_soft(&palette, props.color);
    let base_bg = if is_dark(&palette) {
        Background::Color(palette.input)
    } else {
        Background::Color(iced::Color::TRANSPARENT)
    };

    let mut border = Border {
        radius: radius.into(),
        width: 1.0,
        color: palette.input,
    };
    let background = match props.variant {
        SelectTriggerVariant::Soft => Background::Color(soft_bg),
        SelectTriggerVariant::Ghost => Background::Color(iced::Color::TRANSPARENT),
        SelectTriggerVariant::Classic | SelectTriggerVariant::Surface => base_bg,
    };

    match status {
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => {
            border.color = palette.ring;
        }
        pick_list::Status::Active => {}
    }

    if props.variant == SelectTriggerVariant::Ghost {
        border.width = 0.0;
    }

    pick_list::Style {
        text_color,
        placeholder_color: palette.muted_foreground,
        handle_color: palette.muted_foreground,
        background,
        border,
    }
}

fn select_menu_style(theme: &Theme, props: SelectProps) -> menu::Style {
    let palette = theme.palette;
    let radius = select_radius(theme, props);
    let accent = accent_color(&palette, props.color);
    let soft_bg = accent_soft(&palette, props.color);
    let accent_fg = accent_foreground(&palette, props.color);
    let text_color = palette.popover_foreground;

    let (background, selected_background, selected_text_color) = match props.content_variant {
        SelectContentVariant::Soft => (
            Background::Color(soft_bg),
            Background::Color(accent),
            accent_fg,
        ),
        SelectContentVariant::Solid => (
            Background::Color(palette.popover),
            Background::Color(accent),
            accent_fg,
        ),
    };

    let selected_text_color = if props.high_contrast {
        palette.background
    } else {
        selected_text_color
    };

    let shadow = Shadow {
        color: iced::Color {
            a: 0.15,
            ..iced::Color::BLACK
        },
        offset: Vector::new(0.0, 4.0),
        blur_radius: 12.0,
    };

    menu::Style {
        background,
        border: Border {
            width: 1.0,
            radius: radius.into(),
            color: palette.border,
        },
        text_color,
        selected_text_color,
        selected_background,
        shadow,
    }
}

fn select_handle(size: SelectSize) -> pick_list::Handle<Font> {
    let icon_size = match size {
        SelectSize::One => 12.0,
        SelectSize::Two => 14.0,
        SelectSize::Three => 16.0,
    };

    let icon = |icon: LucideIcon| pick_list::Icon {
        font: Font::with_name("lucide"),
        code_point: char::from(icon),
        size: Some(icon_size.into()),
        line_height: text::LineHeight::default(),
        shaping: text::Shaping::Basic,
    };

    pick_list::Handle::Static(icon(LucideIcon::ChevronDown))
}
