use iced::border::Border;
use iced::widget::scrollable::{Direction, Scrollbar, Status, Style};
use iced::widget::{container, scrollable};
use iced::{Background, Color, Element, Length, Shadow};

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_high, accent_low, is_dark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaSize {
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaScrollbars {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaScrollbarVisibility {
    Auto,
    Visible,
    Hidden,
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAreaProps {
    pub size: ScrollAreaSize,
    pub radius: Option<ButtonRadius>,
    pub bordered: bool,
    pub scrollbars: ScrollAreaScrollbars,
    pub scrollbar_visibility: ScrollAreaScrollbarVisibility,
    pub scrollbar_width: Option<f32>,
    pub scrollbar_rail_width: Option<f32>,
    pub scrollbar_thumb_width: Option<f32>,
    pub scrollbar_margin: Option<f32>,
    pub scrollbar_spacing: Option<f32>,
}

impl Default for ScrollAreaProps {
    fn default() -> Self {
        Self {
            size: ScrollAreaSize::Size1,
            radius: None,
            bordered: true,
            scrollbars: ScrollAreaScrollbars::Both,
            scrollbar_visibility: ScrollAreaScrollbarVisibility::Auto,
            scrollbar_width: None,
            scrollbar_rail_width: None,
            scrollbar_thumb_width: None,
            scrollbar_margin: None,
            scrollbar_spacing: None,
        }
    }
}

impl ScrollAreaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: ScrollAreaSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn scrollbars(mut self, scrollbars: ScrollAreaScrollbars) -> Self {
        self.scrollbars = scrollbars;
        self
    }

    pub fn scrollbar_visibility(mut self, visibility: ScrollAreaScrollbarVisibility) -> Self {
        self.scrollbar_visibility = visibility;
        self
    }

    pub fn scrollbar_width(mut self, scrollbar_width: f32) -> Self {
        self.scrollbar_width = Some(scrollbar_width.clamp(2.0, 32.0));
        self
    }

    pub fn scrollbar_rail_width(mut self, scrollbar_rail_width: f32) -> Self {
        self.scrollbar_rail_width = Some(scrollbar_rail_width.clamp(2.0, 32.0));
        self
    }

    pub fn scrollbar_thumb_width(mut self, scrollbar_thumb_width: f32) -> Self {
        self.scrollbar_thumb_width = Some(scrollbar_thumb_width.clamp(2.0, 32.0));
        self
    }

    pub fn scrollbar_margin(mut self, scrollbar_margin: f32) -> Self {
        self.scrollbar_margin = Some(scrollbar_margin.clamp(0.0, 32.0));
        self
    }

    pub fn scrollbar_spacing(mut self, scrollbar_spacing: f32) -> Self {
        self.scrollbar_spacing = Some(scrollbar_spacing.clamp(0.0, 64.0));
        self
    }

    fn resolved_scrollbar_widths(self, theme: &Theme) -> (f32, f32) {
        let fallback = self
            .scrollbar_width
            .unwrap_or_else(|| self.size.scrollbar_width(theme));

        let rail_width = self.scrollbar_rail_width.unwrap_or(fallback);
        let thumb_width = self.scrollbar_thumb_width.unwrap_or(fallback);

        (rail_width, thumb_width)
    }

    fn resolved_scrollbar_margin(self, theme: &Theme) -> f32 {
        self.scrollbar_margin
            .unwrap_or(theme.styles.scroll_area.default_scrollbar_margin)
    }

    fn resolved_scrollbar_spacing(self) -> Option<f32> {
        self.scrollbar_spacing
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn scroll_area_radius(theme: &Theme, props: ScrollAreaProps) -> f32 {
    match props.radius {
        Some(ButtonRadius::None) => 0.0,
        Some(ButtonRadius::Small) => theme.radius.sm,
        Some(ButtonRadius::Medium) => theme.radius.md,
        Some(ButtonRadius::Large) => theme.radius.lg,
        Some(ButtonRadius::Full) => 9999.0,
        None => theme.radius.md,
    }
}

impl ScrollAreaSize {
    fn scrollbar_width(self, theme: &Theme) -> f32 {
        match self {
            ScrollAreaSize::Size1 => theme.styles.scroll_area.size1_scrollbar_width,
            ScrollAreaSize::Size2 => theme.styles.scroll_area.size2_scrollbar_width,
            ScrollAreaSize::Size3 => theme.styles.scroll_area.size3_scrollbar_width,
        }
    }
}

fn scroll_area_style(theme: &Theme, props: ScrollAreaProps, status: Status) -> Style {
    let palette = theme.palette;
    let radius = scroll_area_radius(theme, props);

    let rail_bg = if is_dark(&palette) {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.7)
    } else {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.6)
    };
    let scroller_bg = if is_dark(&palette) {
        apply_opacity(accent_high(&palette, AccentColor::Gray), 0.9)
    } else {
        apply_opacity(accent_high(&palette, AccentColor::Gray), 0.8)
    };

    let rail_visible = scrollable::Rail {
        background: Some(Background::Color(rail_bg)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: (radius.min(9999.0)).into(),
        },
        scroller: scrollable::Scroller {
            background: Background::Color(scroller_bg),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: (radius.min(9999.0)).into(),
            },
        },
    };
    let rail_hidden = scrollable::Rail {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: (radius.min(9999.0)).into(),
        },
        scroller: scrollable::Scroller {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: (radius.min(9999.0)).into(),
            },
        },
    };

    let show_scrollbars = match props.scrollbar_visibility {
        ScrollAreaScrollbarVisibility::Visible => true,
        ScrollAreaScrollbarVisibility::Hidden => false,
        ScrollAreaScrollbarVisibility::Auto => {
            matches!(status, Status::Hovered { .. } | Status::Dragged { .. })
        }
    };
    let rail = if show_scrollbars { rail_visible } else { rail_hidden };

    let border_width = if props.bordered { 1.0 } else { 0.0 };
    let border_color = if props.bordered {
        palette.border
    } else {
        Color::TRANSPARENT
    };

    Style {
        container: container::Style {
            background: Some(Background::Color(palette.card)),
            text_color: Some(palette.card_foreground),
            border: Border {
                color: border_color,
                width: border_width,
                radius: radius.into(),
            },
            shadow: Shadow::default(),
            snap: true,
        },
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: if show_scrollbars {
            Some(Background::Color(rail_bg))
        } else {
            Some(Background::Color(Color::TRANSPARENT))
        },
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(apply_opacity(palette.background, 0.85)),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow::default(),
            icon: palette.muted_foreground,
        },
    }
}

pub fn scroll_area<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ScrollAreaProps,
    theme: &Theme,
) -> scrollable::Scrollable<'a, Message> {
    let (scrollbar_width, scroller_width) = props.resolved_scrollbar_widths(theme);
    let mut scrollbar = Scrollbar::new()
        .width(scrollbar_width)
        .scroller_width(scroller_width)
        .margin(props.resolved_scrollbar_margin(theme));

    if let Some(spacing) = props.resolved_scrollbar_spacing() {
        scrollbar = scrollbar.spacing(spacing);
    }

    let direction = match props.scrollbars {
        ScrollAreaScrollbars::Vertical => Direction::Vertical(scrollbar),
        ScrollAreaScrollbars::Horizontal => Direction::Horizontal(scrollbar),
        ScrollAreaScrollbars::Both => Direction::Both {
            vertical: scrollbar,
            horizontal: scrollbar,
        },
    };

    let theme = theme.clone();
    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .direction(direction)
        .style(move |_iced_theme, status| scroll_area_style(&theme, props, status))
}
