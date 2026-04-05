use iced::border::Border;
use iced::widget::scrollable::{AbsoluteOffset, Direction, Scrollbar, Status, Style, Viewport};
use iced::widget::{Id, container, scrollable};
use iced::{Background, Color, Element, Length, Shadow, Subscription};
use std::time::Duration;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_high, is_dark};

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

#[derive(Clone, Debug)]
pub struct ScrollAreaProps {
    pub id: Option<Id>,
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
    pub background: Option<Color>,
}

impl Default for ScrollAreaProps {
    fn default() -> Self {
        Self {
            id: None,
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
            background: None,
        }
    }
}

impl ScrollAreaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
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

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    fn resolved_scrollbar_widths(&self, theme: &Theme) -> (f32, f32) {
        let fallback = self
            .scrollbar_width
            .unwrap_or_else(|| self.size.scrollbar_width(theme));

        let rail_width = self.scrollbar_rail_width.unwrap_or(fallback);
        let thumb_width = self.scrollbar_thumb_width.unwrap_or(fallback);

        (rail_width, thumb_width)
    }

    fn resolved_scrollbar_margin(&self, theme: &Theme) -> f32 {
        self.scrollbar_margin
            .unwrap_or(theme.styles.scroll_area.default_scrollbar_margin)
    }

    fn resolved_scrollbar_spacing(&self) -> Option<f32> {
        self.scrollbar_spacing
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn scroll_area_radius(theme: &Theme, props: &ScrollAreaProps) -> f32 {
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

fn scroll_area_style(theme: &Theme, props: &ScrollAreaProps, status: Status) -> Style {
    let palette = theme.palette;
    let radius = scroll_area_radius(theme, props);

    let rail_bg = Color::TRANSPARENT;
    let scroller_bg = match status {
        Status::Active { .. } => {
            if is_dark(&palette) {
                apply_opacity(accent_high(&palette, AccentColor::Gray), 0.3)
            } else {
                apply_opacity(accent_high(&palette, AccentColor::Gray), 0.2)
            }
        }
        Status::Hovered { .. } => {
            if is_dark(&palette) {
                apply_opacity(accent_high(&palette, AccentColor::Gray), 0.6)
            } else {
                apply_opacity(accent_high(&palette, AccentColor::Gray), 0.5)
            }
        }
        Status::Dragged { .. } => {
            if is_dark(&palette) {
                apply_opacity(accent_high(&palette, AccentColor::Gray), 0.8)
            } else {
                apply_opacity(accent_high(&palette, AccentColor::Gray), 0.7)
            }
        }
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
    let rail = if show_scrollbars {
        rail_visible
    } else {
        rail_hidden
    };

    let border_width = if props.bordered { 1.0 } else { 0.0 };
    let border_color = if props.bordered {
        palette.border
    } else {
        Color::TRANSPARENT
    };

    Style {
        container: container::Style {
            background: Some(Background::Color(props.background.unwrap_or(palette.card))),
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
    let style_props = props.clone();
    let id = props.id.clone();
    let mut widget = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .direction(direction)
        .style(move |_iced_theme, status| scroll_area_style(&theme, &style_props, status));

    if let Some(id) = id {
        widget = widget.id(id);
    }
    widget
}

pub fn scroll_area_scroll_to_bottom<Message>(id: Id) -> iced::Task<Message> {
    iced::widget::operation::snap_to_end(id)
}

pub fn scroll_area_is_at_bottom(viewport: Viewport, threshold: f32) -> bool {
    if viewport.content_bounds().height <= viewport.bounds().height {
        return true;
    }
    is_at_bottom_from_relative_offset(viewport.relative_offset().y, threshold)
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollAreaScrollAnimation {
    pub enabled: bool,
    pub speed_px_per_sec: f32,
    pub tick_ms: u64,
    pub settle_distance_px: f32,
}

impl Default for ScrollAreaScrollAnimation {
    fn default() -> Self {
        Self {
            enabled: true,
            speed_px_per_sec: 2400.0,
            tick_ms: 16,
            settle_distance_px: 2.0,
        }
    }
}

impl ScrollAreaScrollAnimation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn speed_px_per_sec(mut self, speed_px_per_sec: f32) -> Self {
        self.speed_px_per_sec = speed_px_per_sec.max(1.0);
        self
    }

    pub fn tick_ms(mut self, tick_ms: u64) -> Self {
        self.tick_ms = tick_ms.max(1);
        self
    }

    pub fn settle_distance_px(mut self, settle_distance_px: f32) -> Self {
        self.settle_distance_px = settle_distance_px.max(0.0);
        self
    }
}

#[derive(Clone, Debug)]
pub struct ScrollAreaScrollAnimator {
    pub animation: ScrollAreaScrollAnimation,
    content_id: Option<Id>,
    active: bool,
    offset_y: f32,
    bounds_h: f32,
    content_h: f32,
}

impl Default for ScrollAreaScrollAnimator {
    fn default() -> Self {
        Self::new(ScrollAreaScrollAnimation::default())
    }
}

impl ScrollAreaScrollAnimator {
    pub fn new(animation: ScrollAreaScrollAnimation) -> Self {
        Self {
            animation,
            content_id: None,
            active: false,
            offset_y: 0.0,
            bounds_h: 0.0,
            content_h: 0.0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn subscription<Message: 'static, F>(&self, map: F) -> Subscription<Message>
    where
        F: Fn() -> Message + Copy + Send + 'static,
    {
        if self.active && self.animation.enabled {
            iced::time::every(Duration::from_millis(self.animation.tick_ms)).map(move |_| map())
        } else {
            Subscription::none()
        }
    }

    pub fn request_to_bottom<Message>(&mut self, id: Id) -> iced::Task<Message> {
        self.content_id = Some(id.clone());
        if !self.animation.enabled {
            self.active = false;
            return scroll_area_scroll_to_bottom(id);
        }

        self.active = true;
        iced::Task::none()
    }

    pub fn on_scrolled(&mut self, viewport: Viewport, threshold: f32) -> bool {
        self.offset_y = viewport.absolute_offset().y.max(0.0);
        self.bounds_h = viewport.bounds().height.max(0.0);
        self.content_h = viewport.content_bounds().height.max(0.0);

        let is_at_bottom = scroll_area_is_at_bottom(viewport, threshold);
        if is_at_bottom {
            self.active = false;
        }
        is_at_bottom
    }

    pub fn tick<Message>(&mut self) -> iced::Task<Message> {
        if !self.active || !self.animation.enabled {
            return iced::Task::none();
        }

        let Some(id) = self.content_id.clone() else {
            self.active = false;
            return iced::Task::none();
        };

        let max_scroll = (self.content_h - self.bounds_h).max(0.0);
        let remaining = (max_scroll - self.offset_y).max(0.0);
        if max_scroll <= 0.0 || remaining <= self.animation.settle_distance_px {
            self.active = false;
            return scroll_area_scroll_to_bottom(id);
        }

        let step = (self.animation.speed_px_per_sec * self.animation.tick_ms as f32 / 1000.0)
            .max(1.0)
            .min(remaining);
        iced::widget::operation::scroll_by(id, AbsoluteOffset { x: 0.0, y: step })
    }
}

fn is_at_bottom_from_relative_offset(relative_offset_y: f32, threshold: f32) -> bool {
    if !relative_offset_y.is_finite() {
        return true;
    }
    let normalized = threshold.clamp(0.0, 1.0);
    relative_offset_y >= 1.0 - normalized
}
