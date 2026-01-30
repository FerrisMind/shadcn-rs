use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

use crate::button::{ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, button_content};
use crate::popover::{PopoverProps, PopoverSize, popover};
use crate::theme::Theme;

const DEFAULT_CONTENT_WIDTH: f32 = 320.0;
const DEFAULT_CONTENT_MAX_HEIGHT: f32 = 360.0;

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuProps {
    pub item_gap: f32,
    pub indicator_width: f32,
    pub indicator_height: f32,
    pub indicator_offset: f32,
}

impl NavigationMenuProps {
    pub fn new() -> Self {
        Self {
            item_gap: 6.0,
            indicator_width: 24.0,
            indicator_height: 2.0,
            indicator_offset: 2.0,
        }
    }

    pub fn item_gap(mut self, item_gap: f32) -> Self {
        self.item_gap = item_gap;
        self
    }

    pub fn indicator_width(mut self, indicator_width: f32) -> Self {
        self.indicator_width = indicator_width;
        self
    }

    pub fn indicator_height(mut self, indicator_height: f32) -> Self {
        self.indicator_height = indicator_height;
        self
    }

    pub fn indicator_offset(mut self, indicator_offset: f32) -> Self {
        self.indicator_offset = indicator_offset;
        self
    }
}

impl Default for NavigationMenuProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NavigationMenuSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum NavigationMenuAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuContentProps {
    pub width: Option<f32>,
    pub max_height: Option<f32>,
    pub side: NavigationMenuSide,
    pub align: NavigationMenuAlign,
    pub side_offset: f32,
    pub align_offset: f32,
    pub padding: f32,
}

impl Default for NavigationMenuContentProps {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationMenuContentProps {
    pub fn new() -> Self {
        Self {
            width: None,
            max_height: None,
            side: NavigationMenuSide::Bottom,
            align: NavigationMenuAlign::Start,
            side_offset: 6.0,
            align_offset: 0.0,
            padding: 12.0,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn side(mut self, side: NavigationMenuSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: NavigationMenuAlign) -> Self {
        self.align = align;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuLinkProps {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding: f32,
    pub rounding: Option<ButtonRadius>,
    pub active: bool,
    pub disabled: bool,
}

impl Default for NavigationMenuLinkProps {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationMenuLinkProps {
    pub fn new() -> Self {
        Self {
            min_width: None,
            min_height: None,
            padding: 8.0,
            rounding: None,
            active: false,
            disabled: false,
        }
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn rounding(mut self, rounding: ButtonRadius) -> Self {
        self.rounding = Some(rounding);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn navigation_menu<'a, Message: Clone + 'a>(
    items: Vec<Element<'a, Message>>,
    props: NavigationMenuProps,
) -> Element<'a, Message> {
    row(items)
        .spacing(props.item_gap)
        .align_y(Alignment::Center)
        .into()
}

pub fn navigation_menu_item<'a, Message: Clone + 'a>(
    trigger: impl Into<Element<'a, Message>>,
    content: Option<impl Into<Element<'a, Message>>>,
    props: NavigationMenuContentProps,
    theme: &Theme,
) -> Element<'a, Message> {
    if let Some(content) = content {
        let width = props.width.unwrap_or(DEFAULT_CONTENT_WIDTH);
        let max_height = props.max_height.unwrap_or(DEFAULT_CONTENT_MAX_HEIGHT);
        let content: Element<'a, Message> = container(content.into())
            .padding(props.padding)
            .width(Length::Fixed(width))
            .max_height(max_height)
            .into();

        popover(
            trigger,
            content,
            PopoverProps::new()
                .size(PopoverSize::Two)
                .offset(props.side_offset),
            theme,
        )
        .into()
    } else {
        trigger.into()
    }
}

pub fn navigation_menu_trigger<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    let content = row![text(label.into()).size(13), text("▾").size(12)]
        .spacing(4)
        .align_y(Alignment::Center);

    button_content(
        content,
        on_press,
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::Two),
        theme,
    )
    .into()
}

pub fn navigation_menu_link<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: NavigationMenuLinkProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let variant = if props.active {
        ButtonVariant::Soft
    } else {
        ButtonVariant::Ghost
    };

    let button = button_content(
        content,
        on_press,
        ButtonProps::new()
            .variant(variant)
            .size(ButtonSize::One)
            .radius(props.rounding.unwrap_or(ButtonRadius::Small))
            .disabled(props.disabled),
        theme,
    );

    let mut wrapper = container(button).padding(props.padding);

    if let Some(width) = props.min_width {
        wrapper = wrapper.width(Length::Fixed(width));
    }
    if let Some(height) = props.min_height {
        wrapper = wrapper.height(Length::Fixed(height));
    }

    wrapper.into()
}
