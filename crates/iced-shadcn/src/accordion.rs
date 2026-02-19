use iced::alignment::Horizontal;
use iced::widget::{column, container, rule, text};
use iced::{Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionType {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone, Debug)]
pub enum AccordionState {
    Single(Option<String>),
    Multiple(Vec<String>),
}

impl Default for AccordionState {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl AccordionState {
    pub fn single(value: Option<String>) -> Self {
        Self::Single(value)
    }

    pub fn multiple(values: Vec<String>) -> Self {
        Self::Multiple(values)
    }

    pub fn is_open(&self, value: &str) -> bool {
        match self {
            AccordionState::Single(current) => current.as_deref() == Some(value),
            AccordionState::Multiple(items) => items.iter().any(|v| v == value),
        }
    }

    pub fn toggle(&mut self, value: &str, collapsible: bool) {
        match self {
            AccordionState::Single(current) => {
                if current.as_deref() == Some(value) {
                    if collapsible {
                        *current = None;
                    }
                } else {
                    *current = Some(value.to_string());
                }
            }
            AccordionState::Multiple(open_items) => {
                if let Some(index) = open_items.iter().position(|v| v == value) {
                    open_items.remove(index);
                } else {
                    open_items.push(value.to_string());
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AccordionProps {
    pub accordion_type: AccordionType,
    pub collapsible: bool,
    pub disabled: bool,
    pub compact: bool,
}

impl Default for AccordionProps {
    fn default() -> Self {
        Self {
            accordion_type: AccordionType::Single,
            collapsible: false,
            disabled: false,
            compact: false,
        }
    }
}

impl AccordionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn accordion_type(mut self, accordion_type: AccordionType) -> Self {
        self.accordion_type = accordion_type;
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
}

pub struct AccordionItemProps<'a, Message> {
    pub value: &'a str,
    pub label: String,
    pub content: Element<'a, Message>,
    pub disabled: bool,
}

impl<'a, Message> AccordionItemProps<'a, Message> {
    pub fn new(
        value: &'a str,
        label: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            value,
            label: label.into(),
            content: content.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn accordion<'a, Message: Clone + 'a, F>(
    items: Vec<AccordionItemProps<'a, Message>>,
    state: AccordionState,
    on_change: Option<F>,
    props: AccordionProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(AccordionState) -> Message + 'a,
{
    let theme = theme.clone();
    let on_change = on_change.as_ref();
    let total_items = items.len();
    let state = match props.accordion_type {
        AccordionType::Single => match state {
            AccordionState::Single(v) => AccordionState::Single(v),
            AccordionState::Multiple(v) => AccordionState::Single(v.first().cloned()),
        },
        AccordionType::Multiple => match state {
            AccordionState::Multiple(v) => AccordionState::Multiple(v),
            AccordionState::Single(v) => AccordionState::Multiple(v.into_iter().collect()),
        },
    };

    let mut body = column![].spacing(if props.compact { 4 } else { 8 });
    for (index, item) in items.into_iter().enumerate() {
        let is_open = state.is_open(item.value);
        let mut next_state = state.clone();
        next_state.toggle(item.value, props.collapsible);

        let on_press = on_change.map(|f| f(next_state.clone()));
        let disabled = props.disabled || item.disabled || on_press.is_none();

        let trigger = button_content(
            text(item.label).size(if props.compact { 12 } else { 13 }),
            on_press,
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(if props.compact {
                    ButtonSize::Size1
                } else {
                    ButtonSize::Size2
                })
                .disabled(disabled),
            &theme,
        );

        body = body.push(container(trigger).width(Length::Fill).padding([2, 0]));

        if is_open {
            body = body.push(container(item.content).width(Length::Fill).padding([0, 12]));
        }

        if index + 1 < total_items {
            let sep = rule::horizontal(1).style(move |_t: &iced::Theme| rule::Style {
                color: theme.palette.border,
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            });
            body = body.push(container(sep).width(Length::Fill));
        }
    }

    container(body)
        .width(Length::Fill)
        .align_x(Horizontal::Left)
        .into()
}
