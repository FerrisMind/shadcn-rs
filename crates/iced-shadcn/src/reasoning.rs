use std::time::{Duration, Instant};

use iced::alignment::Alignment;
use iced::widget::text::LineHeight;
use iced::widget::{column, container, row, text};
use iced::{Element, Length, Padding};
use lucide_icons::Icon as LucideIcon;

use crate::collapsible::{CollapsibleContentProps, CollapsibleProps, collapsible};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug)]
pub struct ReasoningProps {
    pub default_open: bool,
    pub auto_close_delay_ms: u64,
    pub disabled: bool,
    pub compact: bool,
}

impl Default for ReasoningProps {
    fn default() -> Self {
        Self {
            default_open: true,
            auto_close_delay_ms: 1000,
            disabled: false,
            compact: false,
        }
    }
}

impl ReasoningProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn auto_close_delay_ms(mut self, delay_ms: u64) -> Self {
        self.auto_close_delay_ms = delay_ms;
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

#[derive(Clone, Copy, Debug)]
pub struct ReasoningContentProps {
    pub force_mount: bool,
    pub top_spacing: f32,
    pub muted: bool,
}

impl Default for ReasoningContentProps {
    fn default() -> Self {
        Self {
            force_mount: false,
            top_spacing: 0.0,
            muted: false,
        }
    }
}

impl ReasoningContentProps {
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

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    fn to_collapsible_content_props(self) -> CollapsibleContentProps {
        CollapsibleContentProps::new().force_mount(self.force_mount)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResponseProps {
    pub width: Length,
    pub muted: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ReasoningTextProps {
    pub size: f32,
    pub line_height: f32,
    pub paragraph_spacing: f32,
    pub muted: bool,
}

impl Default for ReasoningTextProps {
    fn default() -> Self {
        Self {
            size: 14.0,
            line_height: 1.2,
            paragraph_spacing: 2.0,
            muted: false,
        }
    }
}

impl ReasoningTextProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(1.0);
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height.max(0.5);
        self
    }

    pub fn paragraph_spacing(mut self, paragraph_spacing: f32) -> Self {
        self.paragraph_spacing = paragraph_spacing.max(0.0);
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }
}

impl Default for ResponseProps {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            muted: false,
        }
    }
}

impl ResponseProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ReasoningState {
    pub open: bool,
    pub is_streaming: bool,
    pub duration_seconds: u64,
    pub has_auto_closed: bool,
    pub started_at: Option<Instant>,
    pub auto_close_due_at: Option<Instant>,
}

impl Default for ReasoningState {
    fn default() -> Self {
        Self {
            open: true,
            is_streaming: false,
            duration_seconds: 0,
            has_auto_closed: false,
            started_at: None,
            auto_close_due_at: None,
        }
    }
}

impl ReasoningState {
    pub fn from_props(props: ReasoningProps) -> Self {
        Self {
            open: props.default_open,
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReasoningEffects {
    pub open_changed: Option<bool>,
    pub duration_changed: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
pub enum ReasoningUpdate {
    StreamingChanged { is_streaming: bool, now: Instant },
    OpenChanged(bool),
    Tick { now: Instant },
    Reset,
}

pub fn reasoning_reduce(
    state: &mut ReasoningState,
    update: ReasoningUpdate,
    props: ReasoningProps,
) -> ReasoningEffects {
    let mut effects = ReasoningEffects::default();

    match update {
        ReasoningUpdate::Reset => {
            *state = ReasoningState::from_props(props);
        }
        ReasoningUpdate::OpenChanged(open) => {
            state.open = open;
            effects.open_changed = Some(open);
        }
        ReasoningUpdate::StreamingChanged { is_streaming, now } => {
            if is_streaming {
                state.auto_close_due_at = None;
                if !state.is_streaming {
                    state.started_at = Some(now);
                }
                state.is_streaming = true;
            } else {
                let was_streaming = state.is_streaming;
                state.is_streaming = false;
                if was_streaming {
                    if let Some(started_at) = state.started_at {
                        let elapsed_s = now
                            .saturating_duration_since(started_at)
                            .as_secs_f64()
                            .ceil() as u64;
                        state.duration_seconds = elapsed_s;
                        effects.duration_changed = Some(elapsed_s);
                    }
                    state.started_at = None;
                }

                if props.default_open && state.open && !state.has_auto_closed {
                    state.auto_close_due_at =
                        Some(now + Duration::from_millis(props.auto_close_delay_ms));
                }
            }
        }
        ReasoningUpdate::Tick { now } => {
            if let Some(due_at) = state.auto_close_due_at
                && now >= due_at
            {
                state.open = false;
                state.has_auto_closed = true;
                state.auto_close_due_at = None;
                effects.open_changed = Some(false);
            }
        }
    }

    effects
}

#[derive(Clone, Copy, Debug)]
pub struct ReasoningTriggerProps<'a> {
    pub label_override: Option<&'a str>,
    pub show_brain_icon: bool,
    pub show_chevron: bool,
}

impl<'a> Default for ReasoningTriggerProps<'a> {
    fn default() -> Self {
        Self {
            label_override: None,
            show_brain_icon: true,
            show_chevron: true,
        }
    }
}

impl<'a> ReasoningTriggerProps<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label_override(mut self, label: &'a str) -> Self {
        self.label_override = Some(label);
        self
    }

    pub fn show_brain_icon(mut self, show: bool) -> Self {
        self.show_brain_icon = show;
        self
    }

    pub fn show_chevron(mut self, show: bool) -> Self {
        self.show_chevron = show;
        self
    }
}

pub fn reasoning_thinking_label(is_streaming: bool, duration_seconds: Option<u64>) -> String {
    if is_streaming || duration_seconds == Some(0) {
        return "Thinking...".to_string();
    }

    match duration_seconds {
        None => "Thought for a few seconds".to_string(),
        Some(seconds) => format!("Thought for {seconds} seconds"),
    }
}

pub fn reasoning_trigger_default<'a, Message: 'a>(
    is_open: bool,
    is_streaming: bool,
    duration_seconds: Option<u64>,
    props: ReasoningTriggerProps<'a>,
    _theme: &Theme,
) -> Element<'a, Message> {
    let label = props
        .label_override
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| reasoning_thinking_label(is_streaming, duration_seconds));
    let mut content = row![]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    if props.show_brain_icon {
        content = content.push(lucide_icon(LucideIcon::Brain, 16.0));
    }

    content = content.push(text(label).size(14));

    if props.show_chevron {
        let icon = if is_open {
            LucideIcon::ChevronUp
        } else {
            LucideIcon::ChevronDown
        };
        content = content.push(lucide_icon(icon, 16.0));
    }

    content.into()
}

pub fn reasoning_response<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ResponseProps,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let color = if props.muted {
        theme.palette.muted_foreground
    } else {
        theme.palette.popover_foreground
    };

    container(content)
        .width(props.width)
        .style(move |_theme| iced::widget::container::Style {
            text_color: Some(color),
            ..iced::widget::container::Style::default()
        })
}

pub fn reasoning_text<'a, Message: 'a>(
    content: &'a str,
    props: ReasoningTextProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let color = if props.muted {
        theme.palette.muted_foreground
    } else {
        theme.palette.popover_foreground
    };

    let lines = content.split('\n').map(|line| {
        text(line)
            .size(props.size)
            .line_height(LineHeight::Relative(props.line_height))
            .style(move |_theme: &iced::Theme| iced::widget::text::Style { color: Some(color) })
            .into()
    });

    column(lines)
        .spacing(props.paragraph_spacing)
        .width(Length::Fill)
        .into()
}

pub fn reasoning_content<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ReasoningContentProps,
    theme: &Theme,
) -> Element<'a, Message> {
    reasoning_response(content, ResponseProps::new().muted(props.muted), theme)
        .padding(Padding {
            top: props.top_spacing,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .into()
}

pub fn reasoning<'a, Message: Clone + 'a, F>(
    open: bool,
    trigger: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_open_change: Option<F>,
    reasoning_props: ReasoningProps,
    content_props: ReasoningContentProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    collapsible(
        open,
        trigger,
        reasoning_content(content, content_props, theme),
        on_open_change,
        content_props.to_collapsible_content_props(),
        CollapsibleProps::new()
            .disabled(reasoning_props.disabled)
            .compact(reasoning_props.compact)
            .trigger_hover_highlight(false),
        theme,
    )
}

fn lucide_icon<'a, Message: 'a>(icon: LucideIcon, size: f32) -> Element<'a, Message> {
    text(char::from(icon).to_string())
        .font(iced::Font::with_name("lucide"))
        .size(size)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reasoning_props_defaults_match_contract() {
        let props = ReasoningProps::new();
        assert!(props.default_open);
        assert_eq!(props.auto_close_delay_ms, 1000);
        assert!(!props.disabled);
        assert!(!props.compact);
    }

    #[test]
    fn reasoning_state_starts_from_props() {
        let state = ReasoningState::from_props(ReasoningProps::new().default_open(false));
        assert!(!state.open);
        assert_eq!(state.duration_seconds, 0);
        assert!(!state.is_streaming);
    }

    #[test]
    fn reducer_tracks_duration_with_ceil() {
        let mut state = ReasoningState::from_props(ReasoningProps::new());
        let start = Instant::now();
        let _ = reasoning_reduce(
            &mut state,
            ReasoningUpdate::StreamingChanged {
                is_streaming: true,
                now: start,
            },
            ReasoningProps::new(),
        );
        let effects = reasoning_reduce(
            &mut state,
            ReasoningUpdate::StreamingChanged {
                is_streaming: false,
                now: start + Duration::from_millis(1201),
            },
            ReasoningProps::new(),
        );
        assert_eq!(effects.duration_changed, Some(2));
        assert_eq!(state.duration_seconds, 2);
    }

    #[test]
    fn reducer_auto_closes_once_after_delay() {
        let props = ReasoningProps::new().auto_close_delay_ms(50);
        let mut state = ReasoningState::from_props(props);
        let start = Instant::now();

        let _ = reasoning_reduce(
            &mut state,
            ReasoningUpdate::StreamingChanged {
                is_streaming: true,
                now: start,
            },
            props,
        );
        let _ = reasoning_reduce(
            &mut state,
            ReasoningUpdate::StreamingChanged {
                is_streaming: false,
                now: start + Duration::from_millis(10),
            },
            props,
        );

        let first_tick = reasoning_reduce(
            &mut state,
            ReasoningUpdate::Tick {
                now: start + Duration::from_millis(100),
            },
            props,
        );
        assert_eq!(first_tick.open_changed, Some(false));
        assert!(state.has_auto_closed);

        state.open = true;
        let _ = reasoning_reduce(
            &mut state,
            ReasoningUpdate::StreamingChanged {
                is_streaming: false,
                now: start + Duration::from_millis(200),
            },
            props,
        );
        assert!(state.auto_close_due_at.is_none());
    }

    #[test]
    fn thinking_label_matches_reference_copy() {
        assert_eq!(reasoning_thinking_label(true, Some(10)), "Thinking...");
        assert_eq!(reasoning_thinking_label(false, Some(0)), "Thinking...");
        assert_eq!(
            reasoning_thinking_label(false, None),
            "Thought for a few seconds"
        );
        assert_eq!(
            reasoning_thinking_label(false, Some(7)),
            "Thought for 7 seconds"
        );
    }

    #[test]
    fn reasoning_text_props_defaults_are_dense() {
        let props = ReasoningTextProps::new();
        assert_eq!(props.size, 14.0);
        assert_eq!(props.line_height, 1.2);
        assert_eq!(props.paragraph_spacing, 2.0);
        assert!(!props.muted);
    }
}
