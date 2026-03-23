use std::fmt::{Display, Formatter};
use std::str::FromStr;

use iced::alignment::Alignment;
use iced::widget::{column, container, row, text};
use iced::{Background, Color, Element, Length, Padding};
use lucide_icons::Icon as LucideIcon;

use crate::badge::{BadgeProps, BadgeSize, BadgeVariant, badge_content};
use crate::card::{CardProps, CardVariant, card};
use crate::code_block::{CodeBlockCodeProps, CodeBlockProps, code_block, code_block_code};
use crate::collapsible::{CollapsibleContentProps, CollapsibleProps, collapsible};
use crate::loader::{LoaderProps, loader};
use crate::theme::Theme;
use crate::tokens::mix;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToolUIPartState {
    #[default]
    InputStreaming,
    InputAvailable,
    OutputAvailable,
    OutputError,
}

impl ToolUIPartState {
    pub const fn as_str(self) -> &'static str {
        match self {
            ToolUIPartState::InputStreaming => "input-streaming",
            ToolUIPartState::InputAvailable => "input-available",
            ToolUIPartState::OutputAvailable => "output-available",
            ToolUIPartState::OutputError => "output-error",
        }
    }
}

impl Display for ToolUIPartState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ToolUIPartState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "input-streaming" => Ok(Self::InputStreaming),
            "input-available" => Ok(Self::InputAvailable),
            "output-available" => Ok(Self::OutputAvailable),
            "output-error" => Ok(Self::OutputError),
            _ => Err("unknown tool ui state"),
        }
    }
}

pub fn tool_status_label(state: ToolUIPartState) -> &'static str {
    match state {
        ToolUIPartState::InputStreaming => "Processing",
        ToolUIPartState::InputAvailable => "Ready",
        ToolUIPartState::OutputAvailable => "Completed",
        ToolUIPartState::OutputError => "Error",
    }
}

pub fn tool_status_icon(state: ToolUIPartState) -> LucideIcon {
    match state {
        ToolUIPartState::InputStreaming => LucideIcon::Circle,
        ToolUIPartState::InputAvailable => LucideIcon::Clock3,
        ToolUIPartState::OutputAvailable => LucideIcon::CircleCheck,
        ToolUIPartState::OutputError => LucideIcon::CircleX,
    }
}

pub fn tool_status_icon_color(state: ToolUIPartState, theme: &Theme) -> Color {
    match state {
        ToolUIPartState::InputStreaming => theme.palette.muted_foreground,
        ToolUIPartState::InputAvailable => theme.palette.primary,
        ToolUIPartState::OutputAvailable => Color::from_rgb8(22, 163, 74),
        ToolUIPartState::OutputError => theme.palette.destructive,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToolProps {
    pub disabled: bool,
    pub compact: bool,
    pub trigger_hover_highlight: bool,
    pub bordered: bool,
    pub show_shadow: bool,
}

impl Default for ToolProps {
    fn default() -> Self {
        Self {
            disabled: false,
            compact: false,
            trigger_hover_highlight: false,
            bordered: true,
            show_shadow: false,
        }
    }
}

impl ToolProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn trigger_hover_highlight(mut self, trigger_hover_highlight: bool) -> Self {
        self.trigger_hover_highlight = trigger_hover_highlight;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn show_shadow(mut self, show_shadow: bool) -> Self {
        self.show_shadow = show_shadow;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToolContentProps {
    pub force_mount: bool,
    pub top_spacing: f32,
}

impl Default for ToolContentProps {
    fn default() -> Self {
        Self {
            force_mount: false,
            top_spacing: 0.0,
        }
    }
}

impl ToolContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn top_spacing(mut self, top_spacing: f32) -> Self {
        self.top_spacing = top_spacing.max(0.0);
        self
    }

    fn to_collapsible_content_props(self) -> CollapsibleContentProps {
        CollapsibleContentProps::new().force_mount(self.force_mount)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToolHeaderProps<'a> {
    pub tool_type: &'a str,
    pub state: ToolUIPartState,
    pub show_wrench_icon: bool,
    pub show_chevron: bool,
}

impl<'a> ToolHeaderProps<'a> {
    pub fn new(tool_type: &'a str, state: ToolUIPartState) -> Self {
        Self {
            tool_type,
            state,
            show_wrench_icon: true,
            show_chevron: true,
        }
    }

    pub fn show_wrench_icon(mut self, show_wrench_icon: bool) -> Self {
        self.show_wrench_icon = show_wrench_icon;
        self
    }

    pub fn show_chevron(mut self, show_chevron: bool) -> Self {
        self.show_chevron = show_chevron;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToolInputProps<'a> {
    pub formatted_input: &'a str,
    pub label: &'a str,
    pub language: &'a str,
    pub padding: Padding,
}

impl<'a> ToolInputProps<'a> {
    pub fn new(formatted_input: &'a str) -> Self {
        Self {
            formatted_input,
            label: "Parameters",
            language: "json",
            padding: Padding {
                top: 16.0,
                right: 16.0,
                bottom: 16.0,
                left: 16.0,
            },
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn language(mut self, language: &'a str) -> Self {
        self.language = language;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolOutputValue<'a> {
    Code { content: &'a str, language: &'a str },
    Text(&'a str),
}

impl<'a> ToolOutputValue<'a> {
    pub fn json(content: &'a str) -> Self {
        Self::Code {
            content,
            language: "json",
        }
    }

    pub fn code(content: &'a str, language: &'a str) -> Self {
        Self::Code { content, language }
    }

    pub fn text(content: &'a str) -> Self {
        Self::Text(content)
    }
}

#[derive(Clone, Debug)]
pub struct ToolOutputProps<'a> {
    pub output: Option<ToolOutputValue<'a>>,
    pub error_text: Option<&'a str>,
    pub result_label: &'a str,
    pub error_label: &'a str,
    pub padding: Padding,
}

impl<'a> Default for ToolOutputProps<'a> {
    fn default() -> Self {
        Self {
            output: None,
            error_text: None,
            result_label: "Result",
            error_label: "Error",
            padding: Padding {
                top: 16.0,
                right: 16.0,
                bottom: 16.0,
                left: 16.0,
            },
        }
    }
}

impl<'a> ToolOutputProps<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(mut self, output: ToolOutputValue<'a>) -> Self {
        self.output = Some(output);
        self
    }

    pub fn error_text(mut self, error_text: &'a str) -> Self {
        self.error_text = Some(error_text);
        self
    }

    pub fn result_label(mut self, result_label: &'a str) -> Self {
        self.result_label = result_label;
        self
    }

    pub fn error_label(mut self, error_label: &'a str) -> Self {
        self.error_label = error_label;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct ToolState {
    pub state: ToolUIPartState,
    pub open: bool,
}

impl ToolState {
    pub fn new(state: ToolUIPartState, open: bool) -> Self {
        Self { state, open }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolEffects {
    pub state_changed: Option<ToolUIPartState>,
    pub open_changed: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolUpdate {
    StateChanged(ToolUIPartState),
    OpenChanged(bool),
    Reset { state: ToolUIPartState, open: bool },
}

pub fn tool_reduce(state: &mut ToolState, update: ToolUpdate) -> ToolEffects {
    let mut effects = ToolEffects::default();

    match update {
        ToolUpdate::StateChanged(next_state) => {
            if state.state != next_state {
                state.state = next_state;
                effects.state_changed = Some(next_state);
            }

            if next_state == ToolUIPartState::InputAvailable && !state.open {
                state.open = true;
                effects.open_changed = Some(true);
            }
        }
        ToolUpdate::OpenChanged(open) => {
            if state.open != open {
                state.open = open;
                effects.open_changed = Some(open);
            }
        }
        ToolUpdate::Reset {
            state: reset_state,
            open,
        } => {
            state.state = reset_state;
            state.open = open;
            effects.state_changed = Some(reset_state);
            effects.open_changed = Some(open);
        }
    }

    effects
}

pub fn tool_header_default<'a, Message: Clone + 'a>(
    open: bool,
    props: ToolHeaderProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let status_icon = tool_status_icon(props.state);
    let status_color = tool_status_icon_color(props.state, theme);
    let status_label = tool_status_label(props.state);
    let status_visual: Element<'a, Message> = if props.state == ToolUIPartState::InputStreaming {
        loader(
            LoaderProps::new()
                .size(14.0)
                .color(status_color)
                .duration_ms(900),
            theme,
        )
    } else {
        lucide_icon_colored(status_icon, 14.0, status_color)
    };

    let status_chip = badge_content(
        row![status_visual, text(status_label).size(12),]
            .spacing(6)
            .align_y(Alignment::Center),
        BadgeProps::new()
            .variant(BadgeVariant::Secondary)
            .size(BadgeSize::Size1),
        theme,
    );

    let mut leading = row![].spacing(8).align_y(Alignment::Center);

    if props.show_wrench_icon {
        leading = leading.push(lucide_icon_colored(
            LucideIcon::Wrench,
            14.0,
            theme.palette.muted_foreground,
        ));
    }

    leading = leading
        .push(text(props.tool_type).size(14))
        .push(status_chip);

    let mut header = row![container(leading).width(Length::Fill),]
        .align_y(Alignment::Center)
        .spacing(12)
        .width(Length::Fill);

    if props.show_chevron {
        let chevron = if open {
            LucideIcon::ChevronUp
        } else {
            LucideIcon::ChevronDown
        };
        header = header.push(lucide_icon_colored(
            chevron,
            14.0,
            theme.palette.muted_foreground,
        ));
    }

    container(header)
        .padding(Padding {
            top: 0.0,
            right: 12.0,
            bottom: 0.0,
            left: 12.0,
        })
        .width(Length::Fill)
        .into()
}

pub fn tool_content<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ToolContentProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let text_color = theme.palette.popover_foreground;
    container(content)
        .padding(Padding {
            top: props.top_spacing,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            text_color: Some(text_color),
            ..iced::widget::container::Style::default()
        })
        .into()
}

pub fn tool_input<'a, Message: 'a>(
    props: ToolInputProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let label_color = theme.palette.muted_foreground;
    let code = code_block_code(
        CodeBlockCodeProps::new(props.formatted_input)
            .language(props.language)
            .padding(12.0),
        theme,
    );
    let surface = mix(theme.palette.background, theme.palette.muted, 0.5);
    let block = code_block(
        code,
        CodeBlockProps::new()
            .padding(0.0)
            .background(surface)
            .border_color(theme.palette.border)
            .radius(theme.radius.sm),
        theme,
    );

    container(
        column![
            text(props.label)
                .size(12)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(label_color),
                }),
            block,
        ]
        .spacing(8)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(props.padding)
    .into()
}

pub fn tool_output<'a, Message: 'a>(
    props: ToolOutputProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    if !tool_should_render_output(props.output.as_ref(), props.error_text) {
        return container(column![]).width(Length::Fill).into();
    }

    let title = if props.error_text.is_some() {
        props.error_label
    } else {
        props.result_label
    };

    let label_color = theme.palette.muted_foreground;
    let mut body = column![text(title).size(12).style(move |_theme: &iced::Theme| {
        iced::widget::text::Style {
            color: Some(label_color),
        }
    })]
    .spacing(8)
    .width(Length::Fill);

    if let Some(error_text) = props.error_text {
        let background = mix(theme.palette.destructive, theme.palette.background, 0.9);
        let text_color = theme.palette.destructive;
        let border_color = mix(theme.palette.destructive, theme.palette.border, 0.5);
        let radius = theme.radius.sm;
        let error = container(text(error_text))
            .width(Length::Fill)
            .padding(12.0)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(background)),
                text_color: Some(text_color),
                border: iced::border::Border {
                    color: border_color,
                    width: 1.0,
                    radius: radius.into(),
                },
                ..iced::widget::container::Style::default()
            });
        body = body.push(error);
    } else if let Some(output) = props.output {
        match output {
            ToolOutputValue::Code { content, language } => {
                let code = code_block_code(
                    CodeBlockCodeProps::new(content)
                        .language(language)
                        .padding(12.0),
                    theme,
                );
                let surface = mix(theme.palette.background, theme.palette.muted, 0.5);
                body = body.push(code_block(
                    code,
                    CodeBlockProps::new()
                        .padding(0.0)
                        .background(surface)
                        .border_color(theme.palette.border)
                        .radius(theme.radius.sm),
                    theme,
                ));
            }
            ToolOutputValue::Text(content) => {
                let surface = mix(theme.palette.background, theme.palette.muted, 0.5);
                let text_color = theme.palette.foreground;
                let border_color = theme.palette.border;
                let radius = theme.radius.sm;
                let text_body = container(text(content))
                    .width(Length::Fill)
                    .padding(12.0)
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(surface)),
                        text_color: Some(text_color),
                        border: iced::border::Border {
                            color: border_color,
                            width: 1.0,
                            radius: radius.into(),
                        },
                        ..iced::widget::container::Style::default()
                    });
                body = body.push(text_body);
            }
        }
    }

    container(body)
        .width(Length::Fill)
        .padding(props.padding)
        .into()
}

pub fn tool_should_render_output(
    output: Option<&ToolOutputValue<'_>>,
    error_text: Option<&str>,
) -> bool {
    output.is_some() || error_text.is_some_and(|text| !text.is_empty())
}

pub fn tool<'a, Message: Clone + 'a, F>(
    open: bool,
    header: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_open_change: Option<F>,
    tool_props: ToolProps,
    content_props: ToolContentProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let content = tool_content(content, content_props, theme);
    let collapsible = collapsible(
        open,
        header,
        content,
        on_open_change,
        content_props.to_collapsible_content_props(),
        CollapsibleProps::new()
            .disabled(tool_props.disabled)
            .compact(tool_props.compact)
            .trigger_hover_highlight(tool_props.trigger_hover_highlight),
        theme,
    );

    if !tool_props.bordered {
        return collapsible;
    }

    card(
        collapsible,
        CardProps::new()
            .variant(CardVariant::Ghost)
            .show_shadow(tool_props.show_shadow)
            .padding(0.0)
            .radius(theme.radius.sm),
        theme,
    )
    .into()
}

fn lucide_icon_colored<'a, Message: 'a>(
    icon: LucideIcon,
    size: f32,
    color: Color,
) -> Element<'a, Message> {
    text(char::from(icon).to_string())
        .font(iced::Font::with_name("lucide"))
        .size(size)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style { color: Some(color) })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_labels_match_ai_elements_contract() {
        assert_eq!(
            tool_status_label(ToolUIPartState::InputStreaming),
            "Processing"
        );
        assert_eq!(tool_status_label(ToolUIPartState::InputAvailable), "Ready");
        assert_eq!(
            tool_status_label(ToolUIPartState::OutputAvailable),
            "Completed"
        );
        assert_eq!(tool_status_label(ToolUIPartState::OutputError), "Error");
    }

    #[test]
    fn state_roundtrip_from_str() {
        for state in [
            ToolUIPartState::InputStreaming,
            ToolUIPartState::InputAvailable,
            ToolUIPartState::OutputAvailable,
            ToolUIPartState::OutputError,
        ] {
            let parsed = ToolUIPartState::from_str(state.as_str());
            assert_eq!(parsed.ok(), Some(state));
        }
        assert!(ToolUIPartState::from_str("unknown").is_err());
    }

    #[test]
    fn reducer_auto_opens_when_input_becomes_available() {
        let mut state = ToolState::new(ToolUIPartState::InputStreaming, false);
        let effects = tool_reduce(
            &mut state,
            ToolUpdate::StateChanged(ToolUIPartState::InputAvailable),
        );

        assert_eq!(effects.state_changed, Some(ToolUIPartState::InputAvailable));
        assert_eq!(effects.open_changed, Some(true));
        assert!(state.open);
    }

    #[test]
    fn reducer_open_changed_is_reported_when_toggled() {
        let mut state = ToolState::new(ToolUIPartState::InputStreaming, false);
        let effects = tool_reduce(&mut state, ToolUpdate::OpenChanged(true));
        assert_eq!(effects.open_changed, Some(true));
        assert!(state.open);
    }

    #[test]
    fn output_render_guard_matches_reference_behavior() {
        assert!(!tool_should_render_output(None, None));
        assert!(!tool_should_render_output(None, Some("")));
        assert!(tool_should_render_output(None, Some("error")));
        assert!(tool_should_render_output(
            Some(&ToolOutputValue::json("{\"ok\":true}")),
            None
        ));
    }

    #[test]
    fn output_value_constructors_keep_language() {
        let json = ToolOutputValue::json("{\"a\":1}");
        match json {
            ToolOutputValue::Code { language, .. } => assert_eq!(language, "json"),
            ToolOutputValue::Text(_) => panic!("expected code output"),
        }

        let code = ToolOutputValue::code("SELECT 1", "sql");
        match code {
            ToolOutputValue::Code { language, .. } => assert_eq!(language, "sql"),
            ToolOutputValue::Text(_) => panic!("expected code output"),
        }
    }
}
