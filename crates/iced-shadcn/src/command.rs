use std::borrow::Cow;

use iced::border::Border;
use iced::widget::Id;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length, Shadow};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::dialog::{DialogProps, dialog};
use crate::input::{InputProps, InputSize, InputVariant, input};
use crate::kbd::{KbdProps, KbdSize, kbd, kbd_shortcut};
use crate::scroll_area::{
    ScrollAreaProps, ScrollAreaScrollbarVisibility, ScrollAreaScrollbars, scroll_area,
};
use crate::separator::{SeparatorProps, SeparatorSize, separator};
use crate::spinner::{Spinner, SpinnerSize, SpinnerVariant, spinner};
use crate::theme::Theme;

/// Filter callback used by the command palette when `should_filter` is enabled.
pub type CommandFilter = fn(value: &str, search: &str, keywords: &[String]) -> f32;

#[derive(Clone, Copy, Debug)]
struct CommandTokens {
    bg: iced::Color,
    text: iced::Color,
    muted: iced::Color,
}

fn command_tokens(theme: &Theme) -> CommandTokens {
    CommandTokens {
        bg: theme.palette.popover,
        text: theme.palette.popover_foreground,
        muted: theme.palette.muted_foreground,
    }
}

/// Root `Command` configuration.
pub struct CommandProps<'a, Message> {
    pub id_source: Id,
    pub query: &'a str,
    pub on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    pub input: CommandInputProps<'a>,
    pub input_leading: Vec<Element<'a, Message>>,
    pub input_trailing: Vec<Element<'a, Message>>,
    pub list: CommandListProps<'a, Message>,
    pub empty: Option<CommandEmptyProps<'a>>,
    pub min_width: Option<f32>,
    pub surface_background: Option<iced::Color>,
    pub show_border: bool,
    pub show_shadow: bool,
    pub show_input_separator: bool,
    pub input_separator_color: Option<iced::Color>,
    pub input_list_gap: f32,
    pub show_item_border: bool,
    pub should_filter: bool,
    pub filter: CommandFilter,
    /// Custom corner radius for the command surface; falls back to `theme.radius.md`.
    pub radius: Option<f32>,
    /// Custom corner radius for item rows; falls back to theme defaults.
    pub item_radius: Option<f32>,
}

impl<'a, Message> CommandProps<'a, Message> {
    pub fn new(id_source: Id, query: &'a str, list: CommandListProps<'a, Message>) -> Self {
        Self {
            id_source,
            query,
            on_query_change: None,
            input: CommandInputProps::default(),
            input_leading: Vec::new(),
            input_trailing: Vec::new(),
            list,
            empty: None,
            min_width: None,
            surface_background: None,
            show_border: true,
            show_shadow: true,
            show_input_separator: true,
            input_separator_color: None,
            input_list_gap: 6.0,
            show_item_border: false,
            radius: None,
            item_radius: None,
            should_filter: true,
            filter: default_command_filter,
        }
    }

    pub fn on_query_change(mut self, callback: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query_change = Some(Box::new(callback));
        self
    }

    pub fn input(mut self, input: CommandInputProps<'a>) -> Self {
        self.input = input;
        self
    }

    pub fn input_leading(mut self, slot: impl Into<Element<'a, Message>>) -> Self {
        self.input_leading.push(slot.into());
        self
    }

    pub fn input_trailing(mut self, slot: impl Into<Element<'a, Message>>) -> Self {
        self.input_trailing.push(slot.into());
        self
    }

    pub fn empty(mut self, empty: CommandEmptyProps<'a>) -> Self {
        self.empty = Some(empty);
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width.max(1.0));
        self
    }

    pub fn surface_background(mut self, background: iced::Color) -> Self {
        self.surface_background = Some(background);
        self
    }

    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    pub fn show_container_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    pub fn show_shadow(mut self, show: bool) -> Self {
        self.show_shadow = show;
        self
    }

    pub fn show_input_separator(mut self, show: bool) -> Self {
        self.show_input_separator = show;
        self
    }

    pub fn input_separator_color(mut self, color: iced::Color) -> Self {
        self.input_separator_color = Some(color);
        self
    }

    pub fn input_list_gap(mut self, gap: f32) -> Self {
        self.input_list_gap = gap.max(0.0);
        self
    }

    /// Custom corner radius for the command surface.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }

    /// Custom corner radius for item rows.
    pub fn item_radius(mut self, item_radius: f32) -> Self {
        self.item_radius = Some(item_radius.max(0.0));
        self
    }

    pub fn show_item_border(mut self, show: bool) -> Self {
        self.show_item_border = show;
        self
    }

    pub fn should_filter(mut self, should_filter: bool) -> Self {
        self.should_filter = should_filter;
        self
    }

    pub fn filter(mut self, filter: CommandFilter) -> Self {
        self.filter = filter;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandInputProps<'a> {
    pub placeholder: &'a str,
    pub show_search_icon: bool,
    pub padding_x: Option<f32>,
    pub gap: Option<f32>,
}

impl<'a> CommandInputProps<'a> {
    pub fn new(placeholder: &'a str) -> Self {
        Self {
            placeholder,
            show_search_icon: true,
            padding_x: None,
            gap: None,
        }
    }

    pub fn show_search_icon(mut self, show: bool) -> Self {
        self.show_search_icon = show;
        self
    }

    pub fn padding_x(mut self, padding_x: f32) -> Self {
        self.padding_x = Some(padding_x.max(0.0));
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap.max(0.0));
        self
    }
}

impl Default for CommandInputProps<'_> {
    fn default() -> Self {
        Self::new("Type a command or search...")
    }
}

pub struct CommandListProps<'a, Message> {
    pub max_height: f32,
    pub entries: Vec<CommandListEntry<'a, Message>>,
}

impl<'a, Message> CommandListProps<'a, Message> {
    pub fn new(entries: Vec<CommandListEntry<'a, Message>>) -> Self {
        Self {
            max_height: crate::theme::ThemeStyles::default().command.list_max_height,
            entries,
        }
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height.max(1.0);
        self
    }
}

pub enum CommandListEntry<'a, Message> {
    Group(CommandGroupProps<'a, Message>),
    Item(CommandItemProps<'a, Message>),
    LinkItem(CommandLinkItemProps<'a, Message>),
    Separator(CommandSeparatorProps),
    Loading(CommandLoadingProps<'a>),
}

pub struct CommandGroupProps<'a, Message> {
    pub heading: Option<Cow<'a, str>>,
    pub value: Option<Cow<'a, str>>,
    pub force_mount: bool,
    pub entries: Vec<CommandListEntry<'a, Message>>,
}

impl<'a, Message> CommandGroupProps<'a, Message> {
    pub fn new(entries: Vec<CommandListEntry<'a, Message>>) -> Self {
        Self {
            heading: None,
            value: None,
            force_mount: false,
            entries,
        }
    }

    pub fn heading(mut self, heading: impl Into<Cow<'a, str>>) -> Self {
        self.heading = Some(heading.into());
        self
    }

    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandItemProps<'a, Message> {
    pub value: Cow<'a, str>,
    pub label: Cow<'a, str>,
    pub keywords: Vec<String>,
    pub icon: Option<Cow<'a, str>>,
    pub shortcut: Option<Cow<'a, str>>,
    pub loading: bool,
    pub disabled: bool,
    pub force_mount: bool,
    pub on_select: Option<Message>,
}

impl<'a, Message> CommandItemProps<'a, Message> {
    pub fn new(value: impl Into<Cow<'a, str>>, label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            keywords: Vec::new(),
            icon: None,
            shortcut: None,
            loading: false,
            disabled: false,
            force_mount: false,
            on_select: None,
        }
    }

    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    pub fn icon(mut self, icon: impl Into<Cow<'a, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<Cow<'a, str>>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn on_select(mut self, on_select: Message) -> Self {
        self.on_select = Some(on_select);
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandLinkItemProps<'a, Message> {
    pub value: Cow<'a, str>,
    pub label: Cow<'a, str>,
    pub href: Cow<'a, str>,
    pub keywords: Vec<String>,
    pub icon: Option<Cow<'a, str>>,
    pub shortcut: Option<Cow<'a, str>>,
    pub disabled: bool,
    pub force_mount: bool,
    pub on_select: Option<Message>,
}

impl<'a, Message> CommandLinkItemProps<'a, Message> {
    pub fn new(
        value: impl Into<Cow<'a, str>>,
        label: impl Into<Cow<'a, str>>,
        href: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            href: href.into(),
            keywords: Vec::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            force_mount: false,
            on_select: None,
        }
    }

    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    pub fn icon(mut self, icon: impl Into<Cow<'a, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<Cow<'a, str>>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn on_select(mut self, on_select: Message) -> Self {
        self.on_select = Some(on_select);
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CommandSeparatorProps {
    pub force_mount: bool,
}

impl CommandSeparatorProps {
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandLoadingProps<'a> {
    pub label: Cow<'a, str>,
    pub progress: Option<f32>,
}

impl<'a> CommandLoadingProps<'a> {
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            progress: None,
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandEmptyProps<'a> {
    pub text: Cow<'a, str>,
    pub force_mount: bool,
}

impl<'a> CommandEmptyProps<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            force_mount: false,
        }
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

pub fn command<'a, Message: Clone + 'a>(
    props: CommandProps<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let CommandProps {
        id_source,
        query,
        on_query_change,
        input,
        input_leading,
        input_trailing,
        list,
        empty,
        min_width,
        surface_background,
        show_border,
        show_shadow,
        show_input_separator,
        input_separator_color,
        input_list_gap,
        show_item_border,
        should_filter,
        filter,
        radius,
        item_radius,
    } = props;

    let mut tokens = command_tokens(theme);
    if let Some(background) = surface_background {
        tokens.bg = background;
    }
    let min_width = min_width.unwrap_or(theme.styles.command.min_width);
    let radius_md = radius.unwrap_or(theme.radius.md);
    let menu_shadow = theme.styles.menu.shadow;
    let container_border_color = theme.palette.border;

    let input = command_input(
        query,
        on_query_change,
        input,
        CommandInputSlots {
            leading: input_leading,
            trailing: input_trailing,
        },
        CommandInputChrome {
            show_separator: show_input_separator,
            separator_color: input_separator_color,
            tokens,
        },
        theme,
    );

    let rendered = render_entries(
        list.entries,
        query,
        should_filter,
        show_item_border,
        item_radius,
        filter,
        tokens,
        theme,
    );
    let visible_item_count = rendered.visible_items;
    let list_inset = theme.spacing.xs;
    let list_right_inset = list_inset + 8.0;
    let list_column = column(rendered.elements)
        .spacing(theme.styles.command.list_item_gap)
        .width(Length::Fill);
    let list = container(
        scroll_area(
            container(list_column)
                .padding(iced::Padding {
                    top: 0.0,
                    right: list_right_inset,
                    bottom: list_inset,
                    left: list_inset,
                })
                .width(Length::Fill),
            ScrollAreaProps::new()
                .bordered(false)
                .scrollbars(ScrollAreaScrollbars::Vertical)
                .scrollbar_visibility(ScrollAreaScrollbarVisibility::Auto),
            theme,
        )
        .height(Length::Fixed(list.max_height))
        .width(Length::Fill),
    )
    .padding(iced::Padding {
        top: 0.0,
        right: 0.0,
        bottom: 2.0,
        left: 0.0,
    })
    .width(Length::Fill);

    let mut body = column![input, list].spacing(input_list_gap);
    if let Some(empty) = empty
        && (empty.force_mount || visible_item_count == 0)
    {
        body = body.push(command_empty(empty, tokens));
    }
    let _ = id_source;

    let content_radius = (radius_md - if show_border { 1.0 } else { 0.0 }).max(0.0);
    let content = container(body)
        .width(Length::Fixed(min_width))
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(tokens.bg)),
            text_color: Some(tokens.text),
            border: Border {
                radius: content_radius.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            ..Default::default()
        });

    let shell_shadow = if show_shadow {
        Shadow {
            color: iced::Color {
                a: menu_shadow.opacity,
                ..iced::Color::BLACK
            },
            offset: iced::Vector::new(0.0, menu_shadow.offset_y),
            blur_radius: menu_shadow.blur_radius,
        }
    } else {
        Shadow::default()
    };

    if show_border {
        container(content)
            .padding(1.0)
            .style(move |_t| iced::widget::container::Style {
                background: Some(Background::Color(container_border_color)),
                border: Border {
                    radius: radius_md.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: shell_shadow,
                ..Default::default()
            })
            .into()
    } else {
        container(content)
            .style(move |_t| iced::widget::container::Style {
                border: Border {
                    radius: radius_md.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                shadow: shell_shadow,
                ..Default::default()
            })
            .into()
    }
}

struct CommandInputSlots<'a, Message> {
    leading: Vec<Element<'a, Message>>,
    trailing: Vec<Element<'a, Message>>,
}

struct CommandInputChrome {
    show_separator: bool,
    separator_color: Option<iced::Color>,
    tokens: CommandTokens,
}

fn command_input<'a, Message: Clone + 'a>(
    query: &'a str,
    on_query_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    props: CommandInputProps<'a>,
    slots: CommandInputSlots<'a, Message>,
    chrome: CommandInputChrome,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let tokens = chrome.tokens;
    let search_icon = text(char::from(lucide_icons::Icon::Search).to_string())
        .font(iced::Font::with_name("lucide"))
        .size(13)
        .style(move |_t| iced::widget::text::Style {
            color: Some(tokens.muted),
        });

    let mut input_theme = theme.clone();
    if let Some(padding_x) = props.padding_x {
        input_theme.styles.input.size1_padding_x = padding_x;
    }

    let field = input(
        query,
        props.placeholder,
        on_query_change,
        InputProps::new()
            .size(InputSize::Size1)
            .variant(InputVariant::Ghost),
        &input_theme,
    )
    .width(Length::Fill);

    let mut input_row = row!()
        .spacing(props.gap.unwrap_or(10.0))
        .align_y(Alignment::Center)
        .width(Length::Fill);
    if props.show_search_icon {
        input_row = input_row.push(search_icon);
    }
    for slot in slots.leading {
        input_row = input_row.push(slot);
    }
    input_row = input_row.push(field);
    for slot in slots.trailing {
        input_row = input_row.push(slot);
    }

    let row_padding_x = props.padding_x.unwrap_or(10.0);
    let mut input = column![container(input_row).padding([4.0, row_padding_x])]
        .spacing(0)
        .width(Length::Fill);

    if chrome.show_separator {
        let mut props = SeparatorProps::new()
            .size(SeparatorSize::Size4)
            .thickness(1.0)
            .gap(0.0);
        if let Some(color) = chrome.separator_color {
            props = props.custom_color(color);
        }
        let line = separator(props, theme).width(Length::Fill);
        input = input.push(container(line).padding([0.0, 12.0]));
    }

    input.into()
}

struct RenderedEntries<'a, Message> {
    elements: Vec<Element<'a, Message>>,
    visible_items: usize,
}

#[allow(clippy::too_many_arguments)]
fn render_entries<'a, Message: Clone + 'a>(
    entries: Vec<CommandListEntry<'a, Message>>,
    query: &str,
    should_filter: bool,
    show_item_border: bool,
    item_radius: Option<f32>,
    filter: CommandFilter,
    tokens: CommandTokens,
    theme: &'a Theme,
) -> RenderedEntries<'a, Message> {
    let mut elements = Vec::new();
    let mut visible_items = 0usize;

    for entry in entries {
        match entry {
            CommandListEntry::Group(group) => {
                let nested = render_entries(
                    group.entries,
                    query,
                    should_filter,
                    show_item_border,
                    item_radius,
                    filter,
                    tokens,
                    theme,
                );
                let group_visible = group.force_mount || !nested.elements.is_empty();
                if group_visible {
                    elements.push(command_group(group.heading, nested.elements, tokens, theme));
                    visible_items += nested.visible_items;
                }
            }
            CommandListEntry::Item(item) => {
                let visible = item.force_mount
                    || !should_filter
                    || command_matches(query, &item.value, &item.keywords, filter) > 0.0;
                if visible {
                    elements.push(command_item(
                        CommandItemRenderProps {
                            label: item.label,
                            icon: item.icon,
                            shortcut: item.shortcut,
                            loading: item.loading,
                            disabled: item.disabled,
                            on_select: item.on_select,
                        },
                        show_item_border,
                        item_radius,
                        tokens,
                        theme,
                    ));
                    visible_items += 1;
                }
            }
            CommandListEntry::LinkItem(item) => {
                let visible = item.force_mount
                    || !should_filter
                    || command_matches(query, &item.value, &item.keywords, filter) > 0.0;
                if visible {
                    elements.push(command_link_item(
                        LinkRenderProps {
                            label: item.label,
                            href: item.href,
                            icon: item.icon,
                            shortcut: item.shortcut,
                            disabled: item.disabled,
                            on_select: item.on_select,
                        },
                        show_item_border,
                        item_radius,
                        tokens,
                        theme,
                    ));
                    visible_items += 1;
                }
            }
            CommandListEntry::Separator(separator) => {
                let visible = !should_filter || query.trim().is_empty() || separator.force_mount;
                if visible {
                    elements.push(command_separator(theme));
                }
            }
            CommandListEntry::Loading(loading) => {
                elements.push(command_loading(loading, tokens, theme));
            }
        }
    }

    RenderedEntries {
        elements,
        visible_items,
    }
}

fn command_group<'a, Message: Clone + 'a>(
    heading: Option<Cow<'a, str>>,
    items: Vec<Element<'a, Message>>,
    tokens: CommandTokens,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut group = column!().spacing(theme.styles.command.group_item_gap);
    if let Some(heading) = heading {
        group = group.push(
            text(heading)
                .size(11)
                .wrapping(iced::widget::text::Wrapping::None)
                .style(move |_t| iced::widget::text::Style {
                    color: Some(tokens.muted),
                }),
        );
    }
    for item in items {
        group = group.push(item);
    }
    container(group).padding(6).into()
}

fn command_item<'a, Message: Clone + 'a>(
    props: CommandItemRenderProps<'a, Message>,
    show_item_border: bool,
    item_radius: Option<f32>,
    _tokens: CommandTokens,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let mut content = row!().align_y(Alignment::Center).spacing(8);
    if let Some(icon) = props.icon {
        content = content.push(text(icon).font(iced::Font::with_name("lucide")).size(13));
    }
    let label = container(
        text(props.label)
            .size(13)
            .wrapping(iced::widget::text::Wrapping::None),
    )
    .width(Length::Fill)
    .clip(true);
    content = content.push(label);

    if props.loading {
        content = content.push(spinner(
            Spinner::new(theme)
                .size(SpinnerSize::Size1)
                .variant(SpinnerVariant::PromptCircular)
                .color(theme.palette.muted_foreground)
                .animated(true)
                .duration_ms(900),
        ));
    }

    if let Some(shortcut) = props.shortcut {
        content = content.push(command_shortcut(shortcut, theme));
    }

    let mut item_button = ButtonProps::new()
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Size1)
        .disabled(props.disabled);
    if let Some(item_radius) = item_radius {
        item_button = item_button.custom_radius(item_radius);
    }
    let item = button_content(
        content,
        props.on_select.filter(|_| !props.disabled),
        item_button,
        theme,
    )
    .width(Length::Fill);

    if show_item_border {
        let item_border_radius = item_radius.unwrap_or(theme.radius.sm);
        let item_border_color = theme.palette.border;
        container(item)
            .width(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                border: Border {
                    radius: item_border_radius.into(),
                    width: 1.0,
                    color: item_border_color,
                },
                ..Default::default()
            })
            .into()
    } else {
        item.into()
    }
}

struct CommandItemRenderProps<'a, Message> {
    label: Cow<'a, str>,
    icon: Option<Cow<'a, str>>,
    shortcut: Option<Cow<'a, str>>,
    loading: bool,
    disabled: bool,
    on_select: Option<Message>,
}

struct LinkRenderProps<'a, Message> {
    label: Cow<'a, str>,
    href: Cow<'a, str>,
    icon: Option<Cow<'a, str>>,
    shortcut: Option<Cow<'a, str>>,
    disabled: bool,
    on_select: Option<Message>,
}

fn command_link_item<'a, Message: Clone + 'a>(
    props: LinkRenderProps<'a, Message>,
    show_item_border: bool,
    item_radius: Option<f32>,
    tokens: CommandTokens,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let mut content = row!().align_y(Alignment::Center).spacing(8);
    if let Some(icon) = props.icon {
        content = content.push(text(icon).font(iced::Font::with_name("lucide")).size(13));
    }
    let label = container(
        text(props.label)
            .size(13)
            .wrapping(iced::widget::text::Wrapping::None),
    )
    .width(Length::Fill)
    .clip(true);
    content = content.push(label).push(
        text(props.href)
            .size(10)
            .wrapping(iced::widget::text::Wrapping::None)
            .style(move |_t| iced::widget::text::Style {
                color: Some(tokens.muted),
            }),
    );

    if let Some(shortcut) = props.shortcut {
        content = content.push(command_shortcut(shortcut, theme));
    }

    let mut item_button = ButtonProps::new()
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Size1)
        .disabled(props.disabled);
    if let Some(item_radius) = item_radius {
        item_button = item_button.custom_radius(item_radius);
    }
    let item = button_content(
        content,
        props.on_select.filter(|_| !props.disabled),
        item_button,
        theme,
    )
    .width(Length::Fill);

    if show_item_border {
        let item_border_radius = item_radius.unwrap_or(theme.radius.sm);
        let item_border_color = theme.palette.border;
        container(item)
            .width(Length::Fill)
            .style(move |_t| iced::widget::container::Style {
                border: Border {
                    radius: item_border_radius.into(),
                    width: 1.0,
                    color: item_border_color,
                },
                ..Default::default()
            })
            .into()
    } else {
        item.into()
    }
}

fn command_loading<'a, Message: Clone + 'a>(
    loading: CommandLoadingProps<'a>,
    tokens: CommandTokens,
    theme: &Theme,
) -> Element<'a, Message> {
    let indicator = spinner(
        Spinner::new(theme)
            .size(SpinnerSize::Size1)
            .progress(loading.progress.unwrap_or(0.0))
            .color(tokens.muted),
    );
    row![
        indicator,
        text(loading.label)
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(tokens.muted),
            })
    ]
    .align_y(Alignment::Center)
    .spacing(8)
    .into()
}

fn command_separator<'a, Message: Clone + 'a>(theme: &Theme) -> Element<'a, Message> {
    container(
        separator(
            SeparatorProps::new()
                .size(SeparatorSize::Size4)
                .thickness(1.0)
                .gap(0.0),
            theme,
        )
        .width(Length::Fill),
    )
    .padding([0.0, 8.0])
    .width(Length::Fill)
    .into()
}

fn command_empty<'a, Message: Clone + 'a>(
    empty: CommandEmptyProps<'a>,
    tokens: CommandTokens,
) -> Element<'a, Message> {
    container(
        text(empty.text)
            .size(12)
            .style(move |_t| iced::widget::text::Style {
                color: Some(tokens.muted),
            }),
    )
    .width(Length::Fill)
    .padding(8)
    .into()
}

fn command_shortcut<'a, Message: Clone + 'a>(
    shortcut: Cow<'a, str>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let value = shortcut.trim();
    let props = KbdProps::new().size(KbdSize::Size1).shadow(false);

    if value.contains('+') {
        let labels: Vec<&str> = value
            .split('+')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect();
        if labels.len() > 1 {
            return kbd_shortcut(labels, props, theme);
        }
    }

    kbd(value.to_string(), props, theme)
}

fn command_matches(query: &str, value: &str, keywords: &[String], filter: CommandFilter) -> f32 {
    if query.trim().is_empty() {
        return 1.0;
    }
    filter(value, query, keywords)
}

fn default_command_filter(value: &str, search: &str, keywords: &[String]) -> f32 {
    let mut best = fuzzy_score(search, value);
    for keyword in keywords {
        best = best.max(fuzzy_score(search, keyword));
    }
    best
}

fn fuzzy_score(query: &str, text: &str) -> f32 {
    let query = query.trim().to_lowercase();
    let text = text.to_lowercase();
    if query.is_empty() {
        return 1.0;
    }
    if text.is_empty() {
        return 0.0;
    }

    let mut matched = 0usize;
    let mut query_chars = query.chars();
    let mut target = query_chars.next();
    for ch in text.chars() {
        if Some(ch) == target {
            matched += 1;
            target = query_chars.next();
            if target.is_none() {
                break;
            }
        }
    }

    if matched == 0 {
        return 0.0;
    }

    let ratio = matched as f32 / query.chars().count() as f32;
    if target.is_none() { ratio } else { ratio * 0.5 }
}

pub struct CommandDialogProps<'a, Message> {
    pub open: bool,
    pub on_close: Message,
    pub title: String,
    pub description: String,
    pub show_close_button: bool,
    pub dialog_props: DialogProps,
    pub command: CommandProps<'a, Message>,
}

impl<'a, Message> CommandDialogProps<'a, Message> {
    pub fn new(open: bool, on_close: Message, command: CommandProps<'a, Message>) -> Self {
        Self {
            open,
            on_close,
            title: "Command Palette".to_string(),
            description: "Search for a command to run...".to_string(),
            show_close_button: true,
            dialog_props: DialogProps::new().padding(0),
            command,
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
    let content = command(props.command, theme);
    dialog(
        base,
        props.open,
        content,
        props.on_close,
        props.dialog_props,
        theme,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CommandInputProps, CommandListEntry, CommandListProps, CommandProps,
        default_command_filter, fuzzy_score,
    };

    #[test]
    fn fuzzy_score_matches_subsequence() {
        let score = fuzzy_score("set", "settings");
        assert!(score > 0.0);
    }

    #[test]
    fn fuzzy_score_zero_when_no_match() {
        let score = fuzzy_score("zzz", "settings");
        assert_eq!(score, 0.0);
    }

    #[test]
    fn default_filter_uses_keywords() {
        let score = default_command_filter("Billing", "pay", &["payments".to_string()]);
        assert!(score > 0.0);
    }

    #[test]
    fn command_props_can_hide_input_separator() {
        let props = CommandProps::<()>::new(
            iced::widget::Id::new("command"),
            "",
            CommandListProps::new(Vec::<CommandListEntry<'_, ()>>::new()),
        );
        assert!(props.show_input_separator);

        let props = props.show_input_separator(false);
        assert!(!props.show_input_separator);
    }

    #[test]
    fn command_props_can_override_input_separator_color() {
        let color = iced::Color::from_rgb(0.2, 0.3, 0.4);
        let props = CommandProps::<()>::new(
            iced::widget::Id::new("command"),
            "",
            CommandListProps::new(Vec::<CommandListEntry<'_, ()>>::new()),
        )
        .input_separator_color(color);

        assert_eq!(props.input_separator_color, Some(color));
    }

    #[test]
    fn command_props_can_override_input_list_gap() {
        let props = CommandProps::<()>::new(
            iced::widget::Id::new("command"),
            "",
            CommandListProps::new(Vec::<CommandListEntry<'_, ()>>::new()),
        )
        .input_list_gap(0.0);

        assert_eq!(props.input_list_gap, 0.0);
    }

    #[test]
    fn command_props_can_override_surface_background() {
        let background = iced::Color::from_rgb(0.1, 0.2, 0.3);
        let props = CommandProps::<()>::new(
            iced::widget::Id::new("command"),
            "",
            CommandListProps::new(Vec::<CommandListEntry<'_, ()>>::new()),
        )
        .surface_background(background);

        assert_eq!(props.surface_background, Some(background));
    }

    #[test]
    fn command_props_accept_input_side_slots() {
        let props = CommandProps::<()>::new(
            iced::widget::Id::new("command"),
            "",
            CommandListProps::new(Vec::<CommandListEntry<'_, ()>>::new()),
        )
        .input_leading(iced::widget::text("leading"))
        .input_trailing(iced::widget::text("trailing"));

        assert_eq!(props.input_leading.len(), 1);
        assert_eq!(props.input_trailing.len(), 1);
    }

    #[test]
    fn command_input_props_can_hide_default_search_icon() {
        let props = CommandInputProps::new("Search").show_search_icon(false);

        assert!(!props.show_search_icon);
    }

    #[test]
    fn command_input_props_can_override_horizontal_padding() {
        let props = CommandInputProps::new("Search").padding_x(8.0);

        assert_eq!(props.padding_x, Some(8.0));
    }

    #[test]
    fn command_input_props_can_override_gap() {
        let props = CommandInputProps::new("Search").gap(0.0);

        assert_eq!(props.gap, Some(0.0));
    }
}
