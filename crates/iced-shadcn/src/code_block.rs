use std::time::Duration;

use iced::alignment::Horizontal;
use iced::highlighter;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text::{LineHeight, Span, Wrapping};
use iced::widget::{Space, container, rich_text, text};
use iced::{Background, Border, Color, Element, Font, Length, Padding, Shadow, Task};
use lucide_icons::Icon as LucideIcon;

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, icon_button};
use crate::scroll_area::ScrollAreaScrollbars;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub struct CodeBlockProps {
    pub width: Length,
    pub max_width: Option<f32>,
    pub padding: f32,
    pub radius: Option<f32>,
    pub background: Option<Color>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub show_shadow: bool,
}

impl Default for CodeBlockProps {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            max_width: None,
            padding: 0.0,
            radius: None,
            background: None,
            text_color: None,
            border_color: None,
            show_shadow: false,
        }
    }
}

impl CodeBlockProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(1.0));
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn show_shadow(mut self, show_shadow: bool) -> Self {
        self.show_shadow = show_shadow;
        self
    }
}

pub fn code_block<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: CodeBlockProps,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let theme = theme.clone();
    let radius = props.radius.unwrap_or(theme.radius.lg);
    let background = props.background.unwrap_or(theme.palette.card);
    let border_color = props.border_color.unwrap_or(theme.palette.border);
    let text_color = props.text_color.unwrap_or(theme.palette.card_foreground);
    let shadow = if props.show_shadow {
        Shadow {
            color: Color::from_rgba8(0, 0, 0, 0.12),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        }
    } else {
        Shadow::default()
    };

    let mut widget = container(content)
        .width(props.width)
        .padding(props.padding)
        .style(move |_iced_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            text_color: Some(text_color),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: radius.into(),
            },
            shadow,
            snap: true,
        });

    if let Some(max_width) = props.max_width {
        widget = widget.max_width(max_width);
    }

    widget
}

#[derive(Clone, Copy, Debug)]
pub struct CodeBlockGroupProps {
    pub spacing: f32,
    pub padding: Padding,
    pub show_bottom_border: bool,
    pub border_color: Option<Color>,
}

impl Default for CodeBlockGroupProps {
    fn default() -> Self {
        Self {
            spacing: 8.0,
            padding: Padding {
                top: 8.0,
                right: 8.0,
                bottom: 8.0,
                left: 16.0,
            },
            show_bottom_border: true,
            border_color: None,
        }
    }
}

impl CodeBlockGroupProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn show_bottom_border(mut self, show_bottom_border: bool) -> Self {
        self.show_bottom_border = show_bottom_border;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }
}

pub fn code_block_group<'a, Message: 'a>(
    leading: impl Into<Element<'a, Message>>,
    trailing: impl Into<Element<'a, Message>>,
    props: CodeBlockGroupProps,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let leading = leading.into();
    let trailing = trailing.into();
    let border_color = props.border_color.unwrap_or(theme.palette.border);
    let row_content = iced::widget::row![
        leading,
        container(trailing)
            .width(Length::Fill)
            .align_x(Horizontal::Right),
    ]
    .spacing(props.spacing)
    .align_y(iced::Alignment::Center);
    let header = container(row_content)
        .padding(props.padding)
        .width(Length::Fill);

    if props.show_bottom_border {
        let separator = container(Space::new().height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(move |_iced_theme| iced::widget::container::Style {
                background: Some(Background::Color(border_color)),
                ..iced::widget::container::Style::default()
            });

        container(iced::widget::column![header, separator].spacing(0)).width(Length::Fill)
    } else {
        container(header).width(Length::Fill)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CodeBlockCodeProps<'a> {
    pub code: &'a str,
    pub language: &'a str,
    pub highlighter_theme: highlighter::Theme,
    pub font_size: f32,
    pub line_height: f32,
    pub padding: Padding,
    pub scrollbars: ScrollAreaScrollbars,
    pub wrapping: Wrapping,
}

impl<'a> CodeBlockCodeProps<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            language: "rs",
            highlighter_theme: highlighter::Theme::Base16Ocean,
            font_size: 13.0,
            line_height: 1.35,
            padding: Padding {
                top: 16.0,
                right: 16.0,
                bottom: 16.0,
                left: 16.0,
            },
            scrollbars: ScrollAreaScrollbars::Horizontal,
            wrapping: Wrapping::None,
        }
    }

    pub fn language(mut self, language: &'a str) -> Self {
        self.language = language;
        self
    }

    pub fn highlighter_theme(mut self, highlighter_theme: highlighter::Theme) -> Self {
        self.highlighter_theme = highlighter_theme;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size.max(1.0);
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height.max(0.5);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn scrollbars(mut self, scrollbars: ScrollAreaScrollbars) -> Self {
        self.scrollbars = scrollbars;
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }
}

pub fn code_block_code<'a, Message: 'a>(
    props: CodeBlockCodeProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let spans = highlighted_spans(props.code, props.language, props.highlighter_theme, theme);
    let rich = rich_text(spans)
        .font(Font::MONOSPACE)
        .size(props.font_size)
        .line_height(LineHeight::Relative(props.line_height))
        .wrapping(props.wrapping)
        .width(Length::Shrink);

    let direction = scroll_direction(props.scrollbars);
    let scrollable = iced::widget::scrollable(container(rich).width(Length::Shrink))
        .direction(direction)
        .width(Length::Fill)
        .height(Length::Shrink);

    container(scrollable)
        .width(Length::Fill)
        .padding(props.padding)
        .into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CodeBlockCopyStatus {
    #[default]
    Idle,
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CodeBlockCopyState {
    pub status: CodeBlockCopyStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodeBlockCopyAction {
    Pressed { text: String },
    WriteFinished(Result<(), String>),
    ResetDue,
}

#[derive(Clone, Debug, Default)]
pub struct CodeBlockCopyUpdate {
    pub status_changed: Option<CodeBlockCopyStatus>,
    pub write_text: Option<String>,
    pub schedule_reset_ms: Option<u64>,
}

pub fn code_block_copy_reduce(
    state: &mut CodeBlockCopyState,
    action: CodeBlockCopyAction,
    reset_delay_ms: u64,
) -> CodeBlockCopyUpdate {
    match action {
        CodeBlockCopyAction::Pressed { text } => {
            state.status = CodeBlockCopyStatus::Success;

            CodeBlockCopyUpdate {
                status_changed: Some(state.status),
                write_text: Some(text),
                schedule_reset_ms: Some(reset_delay_ms.max(1)),
            }
        }
        CodeBlockCopyAction::WriteFinished(result) => {
            state.status = if result.is_ok() {
                CodeBlockCopyStatus::Success
            } else {
                CodeBlockCopyStatus::Failure
            };

            CodeBlockCopyUpdate {
                status_changed: Some(state.status),
                write_text: None,
                schedule_reset_ms: Some(reset_delay_ms.max(1)),
            }
        }
        CodeBlockCopyAction::ResetDue => {
            state.status = CodeBlockCopyStatus::Idle;

            CodeBlockCopyUpdate {
                status_changed: Some(state.status),
                write_text: None,
                schedule_reset_ms: None,
            }
        }
    }
}

pub fn code_block_copy_task<Message: Clone + Send + 'static>(
    update: CodeBlockCopyUpdate,
    _on_write_finished: impl Fn(Result<(), String>) -> Message + Copy + Send + 'static,
    on_reset_due: impl Fn() -> Message + Copy + Send + 'static,
) -> Task<Message> {
    let mut tasks: Vec<Task<Message>> = Vec::new();

    if let Some(text) = update.write_text {
        tasks.push(iced::clipboard::write(text));
    }

    if let Some(delay_ms) = update.schedule_reset_ms {
        tasks.push(Task::future(async move {
            let (tx, rx) = iced::futures::channel::oneshot::channel();

            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(delay_ms));
                let _ = tx.send(());
            });

            let _ = rx.await;
            on_reset_due()
        }));
    }

    Task::batch(tasks)
}

#[derive(Clone, Copy, Debug)]
pub struct CodeBlockCopyButtonProps {
    pub button: ButtonProps,
    pub icon_size: f32,
    pub idle_icon: LucideIcon,
    pub success_icon: LucideIcon,
    pub failure_icon: LucideIcon,
}

impl Default for CodeBlockCopyButtonProps {
    fn default() -> Self {
        Self {
            button: ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Size1),
            icon_size: 14.0,
            idle_icon: LucideIcon::Copy,
            success_icon: LucideIcon::Check,
            failure_icon: LucideIcon::X,
        }
    }
}

impl CodeBlockCopyButtonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn button(mut self, button: ButtonProps) -> Self {
        self.button = button;
        self
    }

    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size.max(1.0);
        self
    }

    pub fn idle_icon(mut self, icon: LucideIcon) -> Self {
        self.idle_icon = icon;
        self
    }

    pub fn success_icon(mut self, icon: LucideIcon) -> Self {
        self.success_icon = icon;
        self
    }

    pub fn failure_icon(mut self, icon: LucideIcon) -> Self {
        self.failure_icon = icon;
        self
    }
}

pub fn code_block_copy_button<'a, Message: Clone + 'a>(
    status: CodeBlockCopyStatus,
    on_press: Option<Message>,
    props: CodeBlockCopyButtonProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let icon = code_block_copy_icon(status, props);
    let icon_text = text(char::from(icon).to_string())
        .font(Font::with_name("lucide"))
        .size(props.icon_size);

    icon_button(icon_text, on_press, props.button, theme).into()
}

pub fn code_block_copy_icon(
    status: CodeBlockCopyStatus,
    props: CodeBlockCopyButtonProps,
) -> LucideIcon {
    match status {
        CodeBlockCopyStatus::Idle => props.idle_icon,
        CodeBlockCopyStatus::Success => props.success_icon,
        CodeBlockCopyStatus::Failure => props.failure_icon,
    }
}

fn scroll_direction(scrollbars: ScrollAreaScrollbars) -> Direction {
    match scrollbars {
        ScrollAreaScrollbars::Vertical => Direction::Vertical(Scrollbar::new()),
        ScrollAreaScrollbars::Horizontal => Direction::Horizontal(Scrollbar::new()),
        ScrollAreaScrollbars::Both => Direction::Both {
            vertical: Scrollbar::new(),
            horizontal: Scrollbar::new(),
        },
    }
}

fn highlighted_spans(
    code: &str,
    language: &str,
    theme: highlighter::Theme,
    shadcn_theme: &Theme,
) -> Vec<Span<'static>> {
    if code.is_empty() {
        return vec![Span::new("")];
    }

    let mut spans = Vec::new();
    let mut parser = highlighter::Stream::new(&highlighter::Settings {
        theme,
        token: language.to_owned(),
    });
    let lines = code.split('\n').collect::<Vec<_>>();

    for (index, line) in lines.iter().enumerate() {
        let mut has_highlight = false;
        for (range, highlight) in parser.highlight_line(line) {
            let token = line.get(range).unwrap_or_default().to_owned();
            spans.push(
                Span::new(token)
                    .color_maybe(highlight.color())
                    .font_maybe(highlight.font()),
            );
            has_highlight = true;
        }

        if !has_highlight {
            spans.push(
                Span::new((*line).to_owned())
                    .font(Font::MONOSPACE)
                    .color(shadcn_theme.palette.card_foreground),
            );
        }

        if index + 1 < lines.len() {
            spans.push(
                Span::new("\n")
                    .font(Font::MONOSPACE)
                    .color(shadcn_theme.palette.card_foreground),
            );
        }

        parser.commit();
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_block_props_defaults_and_builders() {
        let props = CodeBlockProps::new()
            .padding(4.0)
            .radius(12.0)
            .max_width(640.0);
        assert_eq!(props.padding, 4.0);
        assert_eq!(props.radius, Some(12.0));
        assert_eq!(props.max_width, Some(640.0));
    }

    #[test]
    fn copy_reduce_transitions_idle_success_and_reset() {
        let mut state = CodeBlockCopyState::default();
        let pressed = code_block_copy_reduce(
            &mut state,
            CodeBlockCopyAction::Pressed {
                text: "fn main() {}".to_owned(),
            },
            800,
        );
        assert!(pressed.write_text.is_some());
        assert_eq!(state.status, CodeBlockCopyStatus::Success);
        assert_eq!(pressed.schedule_reset_ms, Some(800));

        let reset = code_block_copy_reduce(&mut state, CodeBlockCopyAction::ResetDue, 800);
        assert_eq!(state.status, CodeBlockCopyStatus::Idle);
        assert_eq!(reset.schedule_reset_ms, None);
    }

    #[test]
    fn copy_reduce_handles_failure() {
        let mut state = CodeBlockCopyState::default();
        let update = code_block_copy_reduce(
            &mut state,
            CodeBlockCopyAction::WriteFinished(Err("denied".to_owned())),
            900,
        );
        assert_eq!(state.status, CodeBlockCopyStatus::Failure);
        assert_eq!(update.schedule_reset_ms, Some(900));
    }

    #[test]
    fn copy_icon_matches_status() {
        let props = CodeBlockCopyButtonProps::new();
        assert_eq!(
            char::from(code_block_copy_icon(CodeBlockCopyStatus::Idle, props)),
            char::from(LucideIcon::Copy)
        );
        assert_eq!(
            char::from(code_block_copy_icon(CodeBlockCopyStatus::Success, props)),
            char::from(LucideIcon::Check)
        );
        assert_eq!(
            char::from(code_block_copy_icon(CodeBlockCopyStatus::Failure, props)),
            char::from(LucideIcon::X)
        );
    }

    #[test]
    fn highlighted_spans_fallback_for_unknown_language_and_empty_code() {
        let theme = Theme::default();
        let unknown = highlighted_spans(
            "let x = 1;",
            "unknown-language",
            highlighter::Theme::Base16Ocean,
            &theme,
        );
        assert!(!unknown.is_empty());

        let empty = highlighted_spans("", "rs", highlighter::Theme::Base16Ocean, &theme);
        assert_eq!(empty.len(), 1);
        assert_eq!(empty[0].text.as_ref(), "");
    }
}
