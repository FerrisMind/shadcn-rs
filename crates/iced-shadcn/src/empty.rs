use iced::advanced::text::Wrapping;
use iced::alignment::{Horizontal, Vertical};
use iced::border::Border;
use iced::font::Weight;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Color, Element, Font, Length};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

#[derive(Clone, Debug)]
pub struct EmptyProps<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> EmptyProps<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            icon: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyRootProps {
    pub gap: f32,
    pub padding: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub bordered: bool,
    pub dashed: bool,
    pub background: Option<Color>,
}

impl Default for EmptyRootProps {
    fn default() -> Self {
        Self {
            gap: 0.0,
            padding: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            bordered: false,
            dashed: true,
            background: None,
        }
    }
}

impl EmptyRootProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn dashed(mut self, dashed: bool) -> Self {
        self.dashed = dashed;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyHeaderProps {
    pub gap: f32,
    pub max_width: f32,
}

impl Default for EmptyHeaderProps {
    fn default() -> Self {
        Self {
            gap: 0.0,
            max_width: 0.0,
        }
    }
}

impl EmptyHeaderProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyMediaProps {
    pub variant: EmptyMediaVariant,
    pub size: f32,
    pub icon_size: f32,
}

impl Default for EmptyMediaProps {
    fn default() -> Self {
        Self {
            variant: EmptyMediaVariant::Default,
            size: 0.0,
            icon_size: 0.0,
        }
    }
}

impl EmptyMediaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: EmptyMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyTitleProps {
    pub size: f32,
}

impl Default for EmptyTitleProps {
    fn default() -> Self {
        Self { size: 0.0 }
    }
}

impl EmptyTitleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyDescriptionProps {
    pub size: f32,
    pub max_width: f32,
}

impl Default for EmptyDescriptionProps {
    fn default() -> Self {
        Self {
            size: 0.0,
            max_width: 0.0,
        }
    }
}

impl EmptyDescriptionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyContentProps {
    pub gap: f32,
    pub max_width: f32,
}

impl Default for EmptyContentProps {
    fn default() -> Self {
        Self {
            gap: 0.0,
            max_width: 0.0,
        }
    }
}

impl EmptyContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }
}

pub fn empty_root<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: EmptyRootProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let gap = if props.gap > 0.0 {
        props.gap
    } else {
        theme.styles.empty.root_gap
    };
    let padding = if props.padding > 0.0 {
        props.padding
    } else {
        theme.styles.empty.root_padding
    };
    let max_width = if props.max_width > 0.0 {
        props.max_width
    } else {
        theme.styles.empty.root_max_width
    };
    let border_color = if props.dashed {
        apply_opacity(theme.palette.border, 0.9)
    } else {
        theme.palette.border
    };
    let background = props.background;
    let radius = theme.radius.lg;
    let min_height = if props.min_height > 0.0 {
        props.min_height
    } else {
        theme.styles.empty.root_min_height
    };

    container(column![content.into()].spacing(gap).width(Length::Fill))
        .padding(padding)
        .width(Length::Fill)
        .max_width(max_width)
        .center_x(Length::Fill)
        .style(move |_t| iced::widget::container::Style {
            background: background.map(Background::Color),
            border: Border {
                color: if props.bordered {
                    border_color
                } else {
                    Color::TRANSPARENT
                },
                width: if props.bordered { 1.0 } else { 0.0 },
                radius: radius.into(),
            },
            ..Default::default()
        })
        .height(if min_height > 0.0 {
            Length::Fixed(min_height)
        } else {
            Length::Shrink
        })
        .into()
}

pub fn empty_header<'a, Message: 'a>(
    items: Vec<Element<'a, Message>>,
    props: EmptyHeaderProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let gap = if props.gap > 0.0 {
        props.gap
    } else {
        theme.styles.empty.header_gap
    };
    let max_width = if props.max_width > 0.0 {
        props.max_width
    } else {
        theme.styles.empty.header_max_width
    };

    container(
        column(items)
            .spacing(gap)
            .align_x(Alignment::Center)
            .width(Length::Shrink),
    )
    .width(Length::Fill)
    .center_x(Length::Fill)
    .max_width(max_width)
    .into()
}

pub fn empty_media<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: EmptyMediaProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let size = if props.size > 0.0 {
        props.size
    } else {
        theme.styles.empty.media_size
    };
    let icon_size = if props.icon_size > 0.0 {
        props.icon_size
    } else {
        theme.styles.empty.media_icon_size
    };
    let background = match props.variant {
        EmptyMediaVariant::Default => None,
        EmptyMediaVariant::Icon => Some(Background::Color(theme.palette.muted)),
    };
    let text_color = match props.variant {
        EmptyMediaVariant::Default => None,
        EmptyMediaVariant::Icon => Some(theme.palette.foreground),
    };
    let radius = theme.radius.md;

    container(content)
        .padding(match props.variant {
            EmptyMediaVariant::Default => 0.0,
            EmptyMediaVariant::Icon => ((size - icon_size) / 2.0).max(theme.spacing.sm),
        })
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_t| iced::widget::container::Style {
            background,
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
            ..Default::default()
        })
        .into()
}

pub fn empty_title<'a, Message: 'a>(
    value: impl Into<String>,
    props: EmptyTitleProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let size = if props.size > 0.0 {
        props.size
    } else {
        theme.styles.empty.title_size
    };
    text(value.into())
        .size(size)
        .font(Font {
            weight: Weight::Medium,
            ..Font::DEFAULT
        })
        .style(move |_t| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        })
        .align_x(iced::alignment::Horizontal::Center)
        .into()
}

pub fn empty_description<'a, Message: 'a>(
    value: impl Into<String>,
    props: EmptyDescriptionProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let size = if props.size > 0.0 {
        props.size
    } else {
        theme.styles.empty.description_size
    };
    let max_width = if props.max_width > 0.0 {
        props.max_width
    } else {
        theme.styles.empty.description_max_width
    };
    container(
        text(value.into())
            .size(size)
            .wrapping(Wrapping::WordOrGlyph)
            .style(move |_t| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            })
            .align_x(iced::alignment::Horizontal::Center),
    )
    .max_width(max_width)
    .into()
}

pub fn empty_content<'a, Message: 'a>(
    items: Vec<Element<'a, Message>>,
    props: EmptyContentProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let gap = if props.gap > 0.0 {
        props.gap
    } else {
        theme.styles.empty.content_gap
    };
    let max_width = if props.max_width > 0.0 {
        props.max_width
    } else {
        theme.styles.empty.content_max_width
    };

    container(
        column(items)
            .spacing(gap)
            .align_x(Alignment::Center)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .center_x(Length::Fill)
    .max_width(max_width)
    .into()
}

pub fn empty<'a, Message: 'a>(props: EmptyProps<'a>, theme: &'a Theme) -> Element<'a, Message> {
    let mut header_items = Vec::new();

    if let Some(icon) = props.icon {
        header_items.push(empty_media(
            text(icon)
                .size(theme.styles.empty.media_icon_size)
                .font(Font::with_name("lucide"))
                .style(move |_t| iced::widget::text::Style {
                    color: Some(theme.palette.foreground),
                }),
            EmptyMediaProps::new().variant(EmptyMediaVariant::Icon),
            theme,
        ));
    }

    header_items.push(empty_title(props.title, EmptyTitleProps::new(), theme));

    if let Some(description) = props.description {
        header_items.push(empty_description(
            description,
            EmptyDescriptionProps::new(),
            theme,
        ));
    }

    empty_root(
        row![empty_header(header_items, EmptyHeaderProps::new(), theme)]
            .width(Length::Fill)
            .align_y(Alignment::Center),
        EmptyRootProps::new(),
        theme,
    )
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_props_builder() {
        let props = EmptyProps::new("No results")
            .description("Try adjusting your search.")
            .icon("x");

        assert_eq!(props.title, "No results");
        assert_eq!(props.description, Some("Try adjusting your search."));
        assert_eq!(props.icon, Some("x"));
    }

    #[test]
    fn empty_root_props_builder() {
        let props = EmptyRootProps::new()
            .bordered(true)
            .dashed(false)
            .padding(32.0)
            .gap(12.0)
            .max_width(420.0)
            .min_height(240.0);

        assert!(props.bordered);
        assert!(!props.dashed);
        assert_eq!(props.padding, 32.0);
        assert_eq!(props.gap, 12.0);
        assert_eq!(props.max_width, 420.0);
        assert_eq!(props.min_height, 240.0);
    }

    #[test]
    fn empty_media_props_builder() {
        let props = EmptyMediaProps::new()
            .variant(EmptyMediaVariant::Icon)
            .size(48.0)
            .icon_size(20.0);

        assert_eq!(props.variant, EmptyMediaVariant::Icon);
        assert_eq!(props.size, 48.0);
        assert_eq!(props.icon_size, 20.0);
    }
}
