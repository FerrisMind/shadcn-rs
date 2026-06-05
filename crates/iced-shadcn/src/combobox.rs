use iced::border::Border;
use iced::widget::{column, container, row, rule, text};
use iced::{Alignment, Background, Element, Length};
use std::hash::Hash;

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::input::{InputProps, InputSize, InputVariant, input};
use crate::popover::{PopoverProps, PopoverSize, popover};
use crate::theme::Theme;
use crate::tokens::{accent_soft, accent_text};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ComboboxSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

impl From<ComboboxSize> for ButtonSize {
    fn from(size: ComboboxSize) -> Self {
        match size {
            ComboboxSize::Size1 => ButtonSize::Size1,
            ComboboxSize::Size2 => ButtonSize::Size2,
            ComboboxSize::Size3 => ButtonSize::Size3,
        }
    }
}

impl From<ComboboxSize> for InputSize {
    fn from(size: ComboboxSize) -> Self {
        match size {
            ComboboxSize::Size1 => InputSize::Size1,
            ComboboxSize::Size2 => InputSize::Size2,
            ComboboxSize::Size3 => InputSize::Size3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonJustify {
    Start,
    Center,
    #[default]
    Between,
}

#[derive(Clone, Debug)]
pub enum SelectItem {
    Option {
        value: String,
        label: String,
        disabled: bool,
        text_value: Option<String>,
    },
    Group {
        label: String,
        items: Vec<SelectItem>,
    },
    Separator,
    Label(String),
}

impl SelectItem {
    pub fn option(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Option {
            value: value.into(),
            label: label.into(),
            disabled: false,
            text_value: None,
        }
    }

    pub fn option_disabled(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Option {
            value: value.into(),
            label: label.into(),
            disabled: true,
            text_value: None,
        }
    }

    pub fn option_with_text_value(
        value: impl Into<String>,
        label: impl Into<String>,
        text_value: impl Into<String>,
    ) -> Self {
        Self::Option {
            value: value.into(),
            label: label.into(),
            disabled: false,
            text_value: Some(text_value.into()),
        }
    }

    pub fn option_disabled_with_text_value(
        value: impl Into<String>,
        label: impl Into<String>,
        text_value: impl Into<String>,
    ) -> Self {
        Self::Option {
            value: value.into(),
            label: label.into(),
            disabled: true,
            text_value: Some(text_value.into()),
        }
    }

    pub fn group(label: impl Into<String>, items: Vec<SelectItem>) -> Self {
        Self::Group {
            label: label.into(),
            items,
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn label(text: impl Into<String>) -> Self {
        Self::Label(text.into())
    }
}

pub struct ComboboxProps<'a, Id> {
    pub id_source: Id,
    pub value: &'a Option<String>,
    pub search_value: &'a str,
    pub items: &'a [SelectItem],
    pub placeholder: &'a str,
    pub search_placeholder: &'a str,
    pub empty_text: &'a str,
    pub size: ComboboxSize,
    pub variant: InputVariant,
    pub trigger_variant: ButtonVariant,
    pub trigger_justify: ButtonJustify,
    pub disabled: bool,
    pub width: Option<f32>,
}

impl<'a, Id: Hash> ComboboxProps<'a, Id> {
    pub fn new(
        id_source: Id,
        value: &'a Option<String>,
        items: &'a [SelectItem],
        search_value: &'a str,
    ) -> Self {
        Self {
            id_source,
            value,
            search_value,
            items,
            placeholder: "Select option...",
            search_placeholder: "Search...",
            empty_text: "No option found.",
            size: ComboboxSize::Size2,
            variant: InputVariant::Surface,
            trigger_variant: ButtonVariant::Outline,
            trigger_justify: ButtonJustify::Between,
            disabled: false,
            width: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn search_placeholder(mut self, placeholder: &'a str) -> Self {
        self.search_placeholder = placeholder;
        self
    }

    pub fn empty_text(mut self, empty_text: &'a str) -> Self {
        self.empty_text = empty_text;
        self
    }

    pub fn size(mut self, size: ComboboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn trigger_variant(mut self, variant: ButtonVariant) -> Self {
        self.trigger_variant = variant;
        self
    }

    pub fn trigger_justify(mut self, justify: ButtonJustify) -> Self {
        self.trigger_justify = justify;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

fn get_selected_label(items: &[SelectItem], value: &Option<String>) -> Option<String> {
    if let Some(val) = value {
        for item in items {
            match item {
                SelectItem::Option { value, label, .. } if value == val => {
                    return Some(label.clone());
                }
                SelectItem::Group { items, .. } => {
                    if let Some(label) = get_selected_label(items, value) {
                        return Some(label);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn filter_items(items: &[SelectItem], search: &str) -> Vec<SelectItem> {
    if search.trim().is_empty() {
        return items.to_vec();
    }

    let search_lower = search.to_lowercase();
    let mut filtered = Vec::new();

    for item in items {
        match item {
            SelectItem::Option {
                value,
                label,
                text_value,
                disabled,
            } => {
                let searchable = text_value.as_deref().unwrap_or(label);
                if searchable.to_lowercase().contains(&search_lower)
                    || value.to_lowercase().contains(&search_lower)
                {
                    filtered.push(SelectItem::Option {
                        value: value.clone(),
                        label: label.clone(),
                        disabled: *disabled,
                        text_value: text_value.clone(),
                    });
                }
            }
            SelectItem::Group { label, items } => {
                let filtered_items = filter_items(items, search);
                if !filtered_items.is_empty() {
                    filtered.push(SelectItem::Group {
                        label: label.clone(),
                        items: filtered_items,
                    });
                }
            }
            SelectItem::Separator => filtered.push(SelectItem::Separator),
            SelectItem::Label(text) => filtered.push(SelectItem::Label(text.clone())),
        }
    }

    filtered
}

pub fn combobox<'a, Message: Clone + 'a, Id: Hash, F, G>(
    props: ComboboxProps<'a, Id>,
    on_value_change: Option<F>,
    on_search_change: Option<G>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    F: Fn(Option<String>) -> Message + 'a,
    G: Fn(String) -> Message + 'a,
{
    let selected_label = get_selected_label(props.items, props.value)
        .unwrap_or_else(|| props.placeholder.to_string());
    let size = ButtonSize::from(props.size);

    let label = text(selected_label).size(13);
    let chevron = text("▾").size(12);

    let trigger_content: Element<'a, Message> = match props.trigger_justify {
        ButtonJustify::Between => row![label, iced::widget::space().width(Length::Fill), chevron]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .into(),
        ButtonJustify::Center => {
            container(row![label, chevron].spacing(6).align_y(Alignment::Center))
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into()
        }
        ButtonJustify::Start => row![label, chevron]
            .spacing(6)
            .align_y(Alignment::Center)
            .into(),
    };

    let mut trigger = button_content(
        trigger_content,
        None::<Message>,
        ButtonProps::new()
            .variant(props.trigger_variant)
            .size(size)
            .disabled(props.disabled),
        theme,
    );

    if let Some(width) = props.width {
        trigger = trigger.width(Length::Fixed(width));
    }

    let items = filter_items(props.items, props.search_value);
    let on_value_change = on_value_change.as_ref();
    let search_enabled = on_search_change.is_some();
    let on_search_change = on_search_change.map(|f| move |value| f(value));

    let mut list: Vec<Element<'a, Message>> = Vec::new();
    for item in items {
        match item {
            SelectItem::Option {
                value,
                label,
                disabled,
                ..
            } => {
                let enabled = !disabled && on_value_change.is_some();
                let on_press = on_value_change
                    .map(|f| f(Some(value.clone())))
                    .filter(|_| enabled);
                let element = button_content(
                    text(label),
                    on_press,
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size1)
                        .disabled(!enabled),
                    theme,
                )
                .width(Length::Fill)
                .into();
                list.push(element);
            }
            SelectItem::Group { label, items } => {
                list.push(
                    text(label)
                        .size(11)
                        .style(move |_t| iced::widget::text::Style {
                            color: Some(theme.palette.muted_foreground),
                        })
                        .into(),
                );
                for child in items {
                    if let SelectItem::Option {
                        value,
                        label,
                        disabled,
                        ..
                    } = child
                    {
                        let enabled = !disabled && on_value_change.is_some();
                        let on_press = on_value_change
                            .map(|f| f(Some(value.clone())))
                            .filter(|_| enabled);
                        let element = button_content(
                            text(label),
                            on_press,
                            ButtonProps::new()
                                .variant(ButtonVariant::Ghost)
                                .size(ButtonSize::Size1)
                                .disabled(!enabled),
                            theme,
                        )
                        .width(Length::Fill)
                        .into();
                        list.push(element);
                    }
                }
            }
            SelectItem::Separator => {
                list.push(rule::horizontal(1).into());
            }
            SelectItem::Label(text_value) => {
                list.push(text(text_value).size(11).into());
            }
        }
    }

    if list.is_empty() {
        list.push(
            text(props.empty_text)
                .size(12)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                })
                .into(),
        );
    }

    let search_disabled = props.disabled || !search_enabled;
    let search_input = input(
        props.search_value,
        props.search_placeholder,
        on_search_change,
        InputProps::new()
            .size(InputSize::from(props.size))
            .variant(props.variant)
            .disabled(search_disabled),
        theme,
    )
    .width(Length::Fill);

    let content: Element<'a, Message> =
        container(column![search_input, column(list).spacing(4)].spacing(8))
            .padding(8)
            .width(Length::Shrink)
            .style(move |_t| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.popover)),
                text_color: Some(theme.palette.popover_foreground),
                border: Border {
                    radius: theme.radius.md.into(),
                    width: 1.0,
                    color: theme.palette.border,
                },
                ..Default::default()
            })
            .into();

    let trigger: Element<'a, Message> = container(trigger)
        .width(props.width.map(Length::Fixed).unwrap_or(Length::Shrink))
        .style(move |_t| combobox_trigger_style(theme, props.variant))
        .into();

    popover(
        trigger,
        content,
        PopoverProps::new().size(PopoverSize::Size2).offset(6.0),
        theme,
    )
    .into()
}

fn combobox_trigger_style(theme: &Theme, variant: InputVariant) -> iced::widget::container::Style {
    let palette = theme.palette;
    let (background, border_color, text_color, border_width) = match variant {
        InputVariant::Surface => (palette.background, palette.border, palette.foreground, 1.0),
        InputVariant::Classic => (palette.background, palette.border, palette.foreground, 1.0),
        InputVariant::Soft => (
            accent_soft(&palette, crate::tokens::AccentColor::Gray),
            palette.border,
            accent_text(&palette, crate::tokens::AccentColor::Gray),
            1.0,
        ),
        InputVariant::Ghost => (
            iced::Color::TRANSPARENT,
            iced::Color::TRANSPARENT,
            palette.foreground,
            0.0,
        ),
    };

    iced::widget::container::Style {
        background: Some(Background::Color(background)),
        text_color: Some(text_color),
        border: Border {
            radius: theme.radius.sm.into(),
            width: border_width,
            color: border_color,
        },
        ..Default::default()
    }
}
