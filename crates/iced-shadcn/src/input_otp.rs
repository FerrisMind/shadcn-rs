use std::cell::Cell;
use std::fmt::{self, Debug};

use iced::border::{Border, Radius};
use iced::widget::{container, row, text, text_input};
use iced::{Alignment, Background, Element, Length};
use regex::Regex;

use crate::theme::Theme;

pub struct InputOTPOnComplete<'a, Message>(pub Box<dyn Fn(String) -> Message + 'a>);

impl<'a, Message> Debug for InputOTPOnComplete<'a, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputOTPOnComplete").finish()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InputOTPState {
    pub cursor: usize,
    pub focused: bool,
}

#[derive(Debug)]
pub struct InputOTPProps<'a, Message> {
    pub max_length: usize,
    pub pattern: Option<&'a Regex>,
    pub on_complete: Option<InputOTPOnComplete<'a, Message>>,
}

impl<'a, Message> Default for InputOTPProps<'a, Message> {
    fn default() -> Self {
        Self {
            max_length: 6,
            pattern: None,
            on_complete: None,
        }
    }
}

impl<'a, Message> InputOTPProps<'a, Message> {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            pattern: None,
            on_complete: None,
        }
    }

    pub fn pattern(mut self, pattern: &'a Regex) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn on_complete(mut self, callback: impl Fn(String) -> Message + 'a) -> Self {
        self.on_complete = Some(InputOTPOnComplete(Box::new(callback)));
        self
    }
}

pub struct InputOTPContext<'a, Message> {
    pub value: &'a str,
    pub max_length: usize,
    pub pattern: Option<&'a Regex>,
    pub theme: &'a Theme,
    pub enabled: bool,
    chars: Vec<char>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_complete: Option<Box<dyn Fn(String) -> Message + 'a>>,
    group_slot_count: Cell<usize>,
}

pub fn input_otp<'a, Message: Clone + 'a, F>(
    value: &'a str,
    props: InputOTPProps<'a, Message>,
    on_input: Option<F>,
    theme: &'a Theme,
    add_contents: impl FnOnce(&InputOTPContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let (chars, _) = normalized_chars(value, props.max_length, props.pattern);
    let ctx = InputOTPContext {
        value,
        max_length: props.max_length,
        pattern: props.pattern,
        theme,
        enabled: on_input.is_some(),
        chars,
        on_input: on_input.map(|f| Box::new(f) as _),
        on_complete: props.on_complete.map(|cb| cb.0),
        group_slot_count: Cell::new(0),
    };

    add_contents(&ctx)
}

pub fn input_otp_group<'a, Message: Clone + 'a>(
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    row(items).spacing(0).align_y(Alignment::Center).into()
}

pub fn input_otp_slot<'a, Message: Clone + 'a>(
    context: &'a InputOTPContext<'a, Message>,
    index: usize,
) -> Element<'a, Message> {
    input_otp_slot_impl(context, index, false)
}

pub fn input_otp_slot_last<'a, Message: Clone + 'a>(
    context: &'a InputOTPContext<'a, Message>,
    index: usize,
) -> Element<'a, Message> {
    let element = input_otp_slot_impl(context, index, true);
    context.group_slot_count.set(0);
    element
}

pub fn input_otp_separator<'a, Message: Clone + 'a>(theme: &'a Theme) -> Element<'a, Message> {
    text("-")
        .size(14)
        .style(move |_t| iced::widget::text::Style {
            color: Some(theme.palette.muted_foreground),
        })
        .into()
}

fn input_otp_slot_impl<'a, Message: Clone + 'a>(
    context: &'a InputOTPContext<'a, Message>,
    index: usize,
    is_last_in_group: bool,
) -> Element<'a, Message> {
    let position_in_group = context.group_slot_count.get();
    context.group_slot_count.set(position_in_group + 1);

    let radius =
        corner_radius_for_slot_in_group(context.theme, position_in_group, is_last_in_group);
    let slot_char = context.chars.get(index).copied();
    let slot_value = slot_char.map(|ch| ch.to_string()).unwrap_or_default();

    let mut input = text_input::TextInput::new("", &slot_value)
        .padding([8.0, 0.0])
        .size(14)
        .width(Length::Fixed(36.0))
        .style(move |_t, status| input_otp_style(context.theme, radius, status));

    if let Some(on_input) = context.on_input.as_ref() {
        let on_complete = context.on_complete.as_ref();
        let current_chars = context.chars.clone();
        let max_length = context.max_length;
        let pattern = context.pattern;
        input = input.on_input(move |value| {
            let next = apply_slot_input(&current_chars, index, &value, max_length, pattern);
            if next.len() == max_length
                && let Some(on_complete) = on_complete
            {
                return (on_complete)(next);
            }
            (on_input)(next)
        });
    } else {
        input = input.on_input_maybe(None::<fn(String) -> Message>);
    }

    let element = container(input)
        .width(Length::Fixed(36.0))
        .height(Length::Fixed(40.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();

    if context.enabled {
        element
    } else {
        container(element)
            .style(move |_t| iced::widget::container::Style {
                text_color: Some(context.theme.palette.muted_foreground),
                ..Default::default()
            })
            .into()
    }
}

fn input_otp_style(theme: &Theme, radius: Radius, status: text_input::Status) -> text_input::Style {
    let palette = theme.palette;
    let mut border = Border {
        radius,
        width: 1.0,
        color: palette.border,
    };
    let mut background = Background::Color(palette.background);
    let mut value = palette.foreground;
    let mut placeholder = palette.muted_foreground;

    match status {
        text_input::Status::Hovered => {
            border.color = palette.ring;
        }
        text_input::Status::Focused { .. } => {
            border.color = palette.ring;
            border.width = 1.5;
        }
        text_input::Status::Disabled => {
            background = Background::Color(palette.muted);
            value = palette.muted_foreground;
            placeholder = palette.muted_foreground;
        }
        text_input::Status::Active => {}
    }

    text_input::Style {
        background,
        border,
        icon: palette.muted_foreground,
        placeholder,
        value,
        selection: palette.ring,
    }
}

fn is_allowed_char(pattern: Option<&Regex>, ch: char) -> bool {
    if ch.is_control() {
        return false;
    }
    match pattern {
        Some(pattern) => {
            let mut buffer = [0u8; 4];
            pattern.is_match(ch.encode_utf8(&mut buffer))
        }
        None => true,
    }
}

fn normalized_chars(value: &str, max_length: usize, pattern: Option<&Regex>) -> (Vec<char>, bool) {
    let mut chars = Vec::new();
    for ch in value.chars() {
        if chars.len() >= max_length {
            break;
        }
        if is_allowed_char(pattern, ch) {
            chars.push(ch);
        }
    }
    let normalized = chars.iter().collect::<String>();
    (chars, normalized != value)
}

fn apply_slot_input(
    current: &[char],
    index: usize,
    input: &str,
    max_length: usize,
    pattern: Option<&Regex>,
) -> String {
    let mut chars = current.to_vec();
    let next_char = input.chars().find(|ch| is_allowed_char(pattern, *ch));

    match next_char {
        Some(ch) => {
            if index < chars.len() {
                chars[index] = ch;
            } else if chars.len() < max_length {
                chars.push(ch);
            }
        }
        None => {
            if input.is_empty() && index < chars.len() {
                chars.remove(index);
            }
        }
    }

    chars.truncate(max_length);
    chars.iter().collect()
}

fn corner_radius_for_slot_in_group(
    theme: &Theme,
    position_in_group: usize,
    is_last: bool,
) -> Radius {
    let radius = theme.radius.sm;

    match (position_in_group, is_last) {
        (0, true) => Radius::new(radius),
        (0, false) => Radius::default().top_left(radius).bottom_left(radius),
        (_, true) => Radius::default().top_right(radius).bottom_right(radius),
        _ => Radius::default(),
    }
}
