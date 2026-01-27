use iced::widget::{container, mouse_area, row, text};
use iced::{Alignment, Color, Element, Length, mouse};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub struct BreadcrumbProps {
    pub text_size: f32,
    pub item_spacing: f32,
    pub line_spacing: f32,
    pub separator_size: f32,
    pub ellipsis_size: f32,
    pub wrap: bool,
}

impl Default for BreadcrumbProps {
    fn default() -> Self {
        Self {
            text_size: 12.0,
            item_spacing: 6.0,
            line_spacing: 4.0,
            separator_size: 12.0,
            ellipsis_size: 20.0,
            wrap: true,
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
        muted: theme.palette.muted_foreground,
        foreground: theme.palette.foreground,
        separator: theme.palette.muted_foreground,
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
    let base = row(items)
        .spacing(spacing)
        .align_y(Alignment::Center);

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
    ctx: &'a BreadcrumbContext,
) -> Element<'a, Message> {
    let label = text(text_value.into())
        .size(ctx.metrics.text_size)
        .style(move |_t| iced::widget::text::Style {
            color: Some(ctx.tokens.muted),
        });

    let mut area = mouse_area(label);
    if let Some(msg) = on_press {
        area = area.on_press(msg).interaction(mouse::Interaction::Pointer);
    }
    area.into()
}

pub fn breadcrumb_page<'a, Message: Clone + 'a>(
    text_value: impl Into<String>,
    ctx: &'a BreadcrumbContext,
) -> Element<'a, Message> {
    text(text_value.into())
        .size(ctx.metrics.text_size)
        .style(move |_t| iced::widget::text::Style {
            color: Some(ctx.tokens.foreground),
        })
        .into()
}

pub fn breadcrumb_separator<'a, Message: Clone + 'a>(
    ctx: &'a BreadcrumbContext,
    custom: Option<String>,
) -> Element<'a, Message> {
    let text_value = custom.unwrap_or_else(|| "›".to_string());
    text(text_value)
        .size(ctx.metrics.separator_size)
        .style(move |_t| iced::widget::text::Style {
            color: Some(ctx.tokens.separator),
        })
        .into()
}

pub fn breadcrumb_ellipsis<'a, Message: Clone + 'a>(
    ctx: &'a BreadcrumbContext,
) -> Element<'a, Message> {
    container(
        text("…")
            .size(ctx.metrics.ellipsis_size)
            .style(move |_t| iced::widget::text::Style {
                color: Some(ctx.tokens.muted),
            }),
    )
    .width(Length::Fixed(ctx.metrics.ellipsis_size.max(12.0)))
    .height(Length::Fixed(ctx.metrics.ellipsis_size.max(12.0)))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
