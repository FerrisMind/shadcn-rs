use iced::border::Border;
use iced::widget::Id;
use iced::widget::{column, container, row, rule, text, text_input};
use iced::{Alignment, Background, Color, Element, Length};
use std::hash::Hash;

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::dialog::{DialogProps, dialog};
use crate::theme::Theme;

pub struct CommandProps {
    pub id_source: Id,
    pub min_width: Option<f32>,
    pub show_border: bool,
    pub show_shadow: bool,
}

impl CommandProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            min_width: None,
            show_border: true,
            show_shadow: true,
        }
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    pub fn show_shadow(mut self, show: bool) -> Self {
        self.show_shadow = show;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandInputProps {
    pub placeholder: String,
}

impl CommandInputProps {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandListProps {
    pub max_height: f32,
}

impl Default for CommandListProps {
    fn default() -> Self {
        Self { max_height: 300.0 }
    }
}

#[derive(Clone, Debug)]
pub struct CommandGroupProps {
    pub heading: Option<String>,
}

impl CommandGroupProps {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: Some(heading.into()),
        }
    }
}

pub struct CommandItemProps<'a, IdSource, Message> {
    pub id_source: IdSource,
    pub label: String,
    pub keywords: Vec<String>,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub disabled: bool,
    pub on_select: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<'a, IdSource: Hash, Message> CommandItemProps<'a, IdSource, Message> {
    pub fn new(id_source: IdSource, label: impl Into<String>) -> Self {
        Self {
            id_source,
            label: label.into(),
            keywords: Vec::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            on_select: None,
        }
    }

    pub fn keywords(mut self, keywords: &[&str]) -> Self {
        self.keywords = keywords.iter().map(|k| k.to_string()).collect();
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(mut self, callback: impl Fn() -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(callback));
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct CommandState {
    pub query: String,
}

#[derive(Clone, Copy, Debug)]
struct CommandTokens {
    bg: Color,
    text: Color,
    muted: Color,
    border: Color,
}

fn command_tokens(theme: &Theme) -> CommandTokens {
    CommandTokens {
        bg: theme.palette.popover,
        text: theme.palette.popover_foreground,
        muted: theme.palette.muted_foreground,
        border: theme.palette.border,
    }
}

pub struct CommandContext<'a, Message> {
    pub state: &'a CommandState,
    pub on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    tokens: CommandTokens,
}

pub fn command<'a, Message: Clone + 'a>(
    props: CommandProps,
    state: &'a CommandState,
    on_query_change: Option<impl Fn(String) -> Message + 'a>,
    theme: &'a Theme,
    add_contents: impl FnOnce(&CommandContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let tokens = command_tokens(theme);
    let ctx = CommandContext {
        state,
        on_query_change: on_query_change.map(|f| Box::new(f) as _),
        tokens,
    };

    let content = add_contents(&ctx);
    let min_width = props.min_width.unwrap_or(280.0);

    container(content)
        .width(Length::Fixed(min_width))
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(tokens.bg)),
            text_color: Some(tokens.text),
            border: Border {
                radius: theme.radius.md.into(),
                width: if props.show_border { 1.0 } else { 0.0 },
                color: tokens.border,
            },
            ..Default::default()
        })
        .into()
}

pub fn command_input<'a, Message: Clone + 'a>(
    ctx: &'a CommandContext<'a, Message>,
    props: CommandInputProps,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let mut input = text_input::TextInput::new(&props.placeholder, &ctx.state.query)
        .padding([8.0, 12.0])
        .size(14);

    if let Some(on_query_change) = ctx.on_query_change.as_ref() {
        input = input.on_input(on_query_change);
    } else {
        input = input.on_input_maybe(None::<fn(String) -> Message>);
    }

    container(input)
        .width(Length::Fill)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.background)),
            border: Border {
                radius: theme.radius.sm.into(),
                width: 1.0,
                color: theme.palette.border,
            },
            ..Default::default()
        })
        .into()
}

pub fn command_list<'a, Message: Clone + 'a>(
    _ctx: &'a CommandContext<'a, Message>,
    props: CommandListProps,
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let list = column(items).spacing(4);
    let list = container(list).width(Length::Fill);
    let max_height = props.max_height.max(1.0);
    let scroll = iced::widget::scrollable(list).height(Length::Fixed(max_height));
    scroll.into()
}

pub fn command_empty<'a, Message: Clone + 'a>(
    ctx: &'a CommandContext<'a, Message>,
    text_value: &'a str,
) -> Element<'a, Message> {
    text(text_value)
        .size(12)
        .style(move |_t| iced::widget::text::Style {
            color: Some(ctx.tokens.muted),
        })
        .into()
}

pub fn command_group<'a, Message: Clone + 'a>(
    ctx: &'a CommandContext<'a, Message>,
    props: CommandGroupProps,
    items: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut group = column(items).spacing(2);
    if let Some(heading) = props.heading {
        group = group.push(
            text(heading)
                .size(11)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(ctx.tokens.muted),
                }),
        );
    }
    group.into()
}

pub fn command_separator<'a, Message: Clone + 'a>() -> Element<'a, Message> {
    rule::horizontal(1).into()
}

pub fn command_item<'a, Message: Clone + 'a, IdSource: Hash>(
    ctx: &'a CommandContext<'a, Message>,
    props: CommandItemProps<'a, IdSource, Message>,
    theme: &'a Theme,
) -> Option<Element<'a, Message>> {
    let query = ctx.state.query.trim();
    if !command_matches(query, &props.label, &props.keywords) {
        return None;
    }

    let on_press = props.on_select.map(|f| f()).filter(|_| !props.disabled);

    let mut content_items: Vec<Element<'a, Message>> = Vec::new();
    if let Some(icon) = props.icon {
        content_items.push(text(icon).size(12).into());
    }
    content_items.push(text(props.label).size(13).into());
    content_items.push(iced::widget::space().width(Length::Fill).into());
    content_items.push(
        props
            .shortcut
            .map(|shortcut| text(shortcut).size(10).into())
            .unwrap_or_else(|| text("").into()),
    );

    let content = row(content_items).align_y(Alignment::Center);

    let element = button_content(
        content,
        on_press,
        ButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::One)
            .disabled(props.disabled),
        theme,
    )
    .width(Length::Fill)
    .into();

    Some(element)
}

pub fn command_shortcut<'a, Message: Clone + 'a>(text_value: &'a str) -> Element<'a, Message> {
    text(text_value).size(10).into()
}

fn command_matches(query: &str, label: &str, keywords: &[String]) -> bool {
    if query.is_empty() {
        return true;
    }
    if fuzzy_match(query, label) {
        return true;
    }
    keywords.iter().any(|kw| fuzzy_match(query, kw))
}

fn fuzzy_match(query: &str, text_value: &str) -> bool {
    let query_lower = query.to_lowercase();
    let mut q = query_lower.chars();
    let mut q_next = q.next();
    if q_next.is_none() {
        return true;
    }
    for ch in text_value.to_lowercase().chars() {
        if Some(ch) == q_next {
            q_next = q.next();
            if q_next.is_none() {
                return true;
            }
        }
    }
    false
}

pub struct CommandDialogProps<'a, Message> {
    pub open: bool,
    pub on_close: Message,
    pub title: String,
    pub description: String,
    pub show_close_button: bool,
    pub dialog_props: DialogProps,
    pub content: Element<'a, Message>,
}

impl<'a, Message> CommandDialogProps<'a, Message> {
    pub fn new(open: bool, on_close: Message, content: Element<'a, Message>) -> Self {
        Self {
            open,
            on_close,
            title: "Command Palette".to_string(),
            description: "Search for a command to run...".to_string(),
            show_close_button: true,
            dialog_props: DialogProps::new(),
            content,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    pub fn dialog_props(mut self, props: DialogProps) -> Self {
        self.dialog_props = props;
        self
    }
}

pub fn command_dialog<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    props: CommandDialogProps<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    dialog(
        base,
        props.open,
        props.content,
        props.on_close,
        props.dialog_props,
        theme,
    )
}
