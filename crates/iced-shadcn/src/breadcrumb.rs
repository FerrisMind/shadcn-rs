use iced::widget::{container, mouse_area, row, text};
use iced::{Alignment, Color, Element, Font, Length, mouse};
use lucide_icons::Icon as LucideIcon;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbProps {
    pub text_size: f32,
    pub item_spacing: f32,
    pub line_spacing: f32,
    pub separator_size: f32,
    pub ellipsis_size: f32,
    pub wrap: bool,
    pub muted_color: Option<Color>,
    pub current_color: Option<Color>,
    pub separator_color: Option<Color>,
}

impl Default for BreadcrumbProps {
    fn default() -> Self {
        Self {
            text_size: 12.0,
            item_spacing: 4.0,
            line_spacing: 4.0,
            separator_size: 12.0,
            ellipsis_size: 20.0,
            wrap: true,
            muted_color: None,
            current_color: None,
            separator_color: None,
        }
    }
}

impl BreadcrumbProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    pub fn item_spacing(mut self, item_spacing: f32) -> Self {
        self.item_spacing = item_spacing;
        self
    }

    pub fn line_spacing(mut self, line_spacing: f32) -> Self {
        self.line_spacing = line_spacing;
        self
    }

    pub fn separator_size(mut self, separator_size: f32) -> Self {
        self.separator_size = separator_size;
        self
    }

    pub fn ellipsis_size(mut self, ellipsis_size: f32) -> Self {
        self.ellipsis_size = ellipsis_size;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn muted_color(mut self, color: Color) -> Self {
        self.muted_color = Some(color);
        self
    }

    pub fn current_color(mut self, color: Color) -> Self {
        self.current_color = Some(color);
        self
    }

    pub fn separator_color(mut self, color: Color) -> Self {
        self.separator_color = Some(color);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbTokens {
    pub muted: Color,
    pub foreground: Color,
    pub separator: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbMetrics {
    pub text_size: f32,
    pub item_spacing: f32,
    pub line_spacing: f32,
    pub separator_size: f32,
    pub ellipsis_size: f32,
    pub wrap: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbContext {
    pub tokens: BreadcrumbTokens,
    pub metrics: BreadcrumbMetrics,
}

fn spacing_px(value: f32) -> f32 {
    value.max(0.0)
}

pub fn breadcrumb<R>(
    theme: &Theme,
    props: BreadcrumbProps,
    add_contents: impl FnOnce(&BreadcrumbContext) -> R,
) -> R {
    let tokens = BreadcrumbTokens {
        muted: props.muted_color.unwrap_or(theme.palette.muted_foreground),
        foreground: props.current_color.unwrap_or(theme.palette.foreground),
        separator: props
            .separator_color
            .unwrap_or(props.muted_color.unwrap_or(theme.palette.muted_foreground)),
    };
    let metrics = BreadcrumbMetrics {
        text_size: props.text_size,
        item_spacing: props.item_spacing,
        line_spacing: props.line_spacing,
        separator_size: props.separator_size,
        ellipsis_size: props.ellipsis_size,
        wrap: props.wrap,
    };
    let ctx = BreadcrumbContext { tokens, metrics };
    add_contents(&ctx)
}

pub fn breadcrumb_list<'a, Message: Clone + 'a>(
    ctx: &BreadcrumbContext,
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let spacing = spacing_px(ctx.metrics.item_spacing);
    let base = row(items).spacing(spacing).align_y(Alignment::Center);

    if ctx.metrics.wrap {
        base.wrap()
            .vertical_spacing(ctx.metrics.line_spacing)
            .into()
    } else {
        base.into()
    }
}

pub fn breadcrumb_item<'a, Message: Clone + 'a>(
    ctx: &BreadcrumbContext,
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    row(items)
        .spacing(spacing_px(ctx.metrics.item_spacing))
        .align_y(Alignment::Center)
        .into()
}

pub fn breadcrumb_link<'a, Message: Clone + 'a>(
    text_value: impl Into<String>,
    on_press: Option<Message>,
    ctx: &BreadcrumbContext,
) -> Element<'a, Message> {
    let muted = ctx.tokens.muted;
    let text_size = ctx.metrics.text_size;
    let label = text(text_value.into())
        .size(text_size)
        .style(move |_t| iced::widget::text::Style { color: Some(muted) });

    let mut area = mouse_area(label);
    if let Some(msg) = on_press {
        area = area.on_press(msg).interaction(mouse::Interaction::Pointer);
    }
    area.into()
}

pub fn breadcrumb_page<'a, Message: Clone + 'a>(
    text_value: impl Into<String>,
    ctx: &BreadcrumbContext,
) -> Element<'a, Message> {
    let foreground = ctx.tokens.foreground;
    let text_size = ctx.metrics.text_size;
    text(text_value.into())
        .size(text_size)
        .style(move |_t| iced::widget::text::Style {
            color: Some(foreground),
        })
        .into()
}

pub fn breadcrumb_separator<'a, Message: Clone + 'a>(
    ctx: &BreadcrumbContext,
    custom: Option<String>,
) -> Element<'a, Message> {
    let separator = ctx.tokens.separator;
    let separator_size = ctx.metrics.separator_size;
    let (text_value, use_lucide_font) = match custom {
        Some(custom) => (custom, false),
        None => (char::from(LucideIcon::ChevronRight).to_string(), true),
    };
    let separator_width = if use_lucide_font {
        (separator_size * 0.7).max(6.0)
    } else {
        separator_size.max(6.0)
    };

    let mut label =
        text(text_value)
            .size(separator_size)
            .style(move |_t| iced::widget::text::Style {
                color: Some(separator),
            });

    if use_lucide_font {
        label = label.font(Font::with_name("lucide"));
    }

    container(label)
        .width(Length::Fixed(separator_width))
        .center_x(Length::Fill)
        .into()
}

pub fn breadcrumb_ellipsis<'a, Message: Clone + 'a>(
    ctx: &BreadcrumbContext,
) -> Element<'a, Message> {
    let muted = ctx.tokens.muted;
    let ellipsis_size = ctx.metrics.ellipsis_size;
    container(
        text("…")
            .size(ellipsis_size)
            .style(move |_t| iced::widget::text::Style { color: Some(muted) }),
    )
    .width(Length::Fixed(ellipsis_size.max(12.0)))
    .height(Length::Fixed(ellipsis_size.max(12.0)))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
