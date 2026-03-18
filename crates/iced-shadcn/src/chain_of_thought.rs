use std::time::{Duration, Instant};

use iced::alignment::Alignment;
use iced::widget::text::LineHeight;
use iced::widget::{column, container, row, text};
use iced::{Color, Element, Length, Padding};
use lucide_icons::Icon as LucideIcon;

use crate::badge::{BadgeProps, BadgeSize, BadgeVariant, badge};
use crate::collapsible::{CollapsibleContentProps, CollapsibleProps, collapsible};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct ChainOfThoughtProps {
    pub default_open: bool,
    pub disabled: bool,
    pub compact: bool,
}

impl ChainOfThoughtProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
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
pub struct ChainOfThoughtContentProps {
    pub force_mount: bool,
    pub top_spacing: f32,
}

impl Default for ChainOfThoughtContentProps {
    fn default() -> Self {
        Self {
            force_mount: false,
            top_spacing: 8.0,
        }
    }
}

impl ChainOfThoughtContentProps {
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
pub struct ChainOfThoughtHeaderProps<'a> {
    pub label_override: Option<&'a str>,
    pub show_brain_icon: bool,
    pub show_chevron: bool,
}

impl<'a> Default for ChainOfThoughtHeaderProps<'a> {
    fn default() -> Self {
        Self {
            label_override: None,
            show_brain_icon: true,
            show_chevron: true,
        }
    }
}

impl<'a> ChainOfThoughtHeaderProps<'a> {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChainOfThoughtStepStatus {
    #[default]
    Complete,
    Active,
    Pending,
}

#[derive(Clone, Copy, Debug)]
pub struct ChainOfThoughtStepProps<'a> {
    pub icon: LucideIcon,
    pub label: &'a str,
    pub description: Option<&'a str>,
    pub status: ChainOfThoughtStepStatus,
    pub delay_ms: Option<u64>,
}

impl<'a> ChainOfThoughtStepProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            icon: LucideIcon::Dot,
            label,
            description: None,
            status: ChainOfThoughtStepStatus::Complete,
            delay_ms: None,
        }
    }

    pub fn icon(mut self, icon: LucideIcon) -> Self {
        self.icon = icon;
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn status(mut self, status: ChainOfThoughtStepStatus) -> Self {
        self.status = status;
        self
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = Some(delay_ms);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChainOfThoughtImageProps<'a> {
    pub caption: Option<&'a str>,
    pub max_height: f32,
    pub max_width: f32,
    pub padding: f32,
}

impl Default for ChainOfThoughtImageProps<'_> {
    fn default() -> Self {
        Self {
            caption: None,
            max_height: 352.0,
            max_width: 420.0,
            padding: 12.0,
        }
    }
}

impl<'a> ChainOfThoughtImageProps<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height.max(1.0);
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width.max(1.0);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChainOfThoughtSearchResultsProps {
    pub spacing: f32,
}

impl Default for ChainOfThoughtSearchResultsProps {
    fn default() -> Self {
        Self { spacing: 8.0 }
    }
}

impl ChainOfThoughtSearchResultsProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChainOfThoughtSearchResultProps<'a, Message> {
    pub badge: BadgeProps<'a, Message>,
}

impl<Message> Default for ChainOfThoughtSearchResultProps<'_, Message> {
    fn default() -> Self {
        Self {
            badge: BadgeProps::new()
                .variant(BadgeVariant::Secondary)
                .size(BadgeSize::Size1),
        }
    }
}

impl<'a, Message> ChainOfThoughtSearchResultProps<'a, Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn badge(mut self, badge: BadgeProps<'a, Message>) -> Self {
        self.badge = badge;
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct ChainOfThoughtState {
    pub open: bool,
    pub opened_at: Option<Instant>,
}

impl ChainOfThoughtState {
    pub fn from_props(props: ChainOfThoughtProps) -> Self {
        Self {
            open: props.default_open,
            opened_at: if props.default_open {
                Some(Instant::now())
            } else {
                None
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChainOfThoughtEffects {
    pub open_changed: Option<bool>,
}

#[derive(Clone, Copy, Debug)]
pub enum ChainOfThoughtUpdate {
    OpenChanged { open: bool, now: Instant },
    Reset { now: Instant },
}

pub fn chain_of_thought_reduce(
    state: &mut ChainOfThoughtState,
    update: ChainOfThoughtUpdate,
    props: ChainOfThoughtProps,
) -> ChainOfThoughtEffects {
    let mut effects = ChainOfThoughtEffects::default();

    match update {
        ChainOfThoughtUpdate::OpenChanged { open, now } => {
            state.open = open;
            state.opened_at = if open { Some(now) } else { None };
            effects.open_changed = Some(open);
        }
        ChainOfThoughtUpdate::Reset { now } => {
            state.open = props.default_open;
            state.opened_at = if props.default_open { Some(now) } else { None };
        }
    }

    effects
}

pub fn chain_of_thought_step_is_visible(
    state: &ChainOfThoughtState,
    now: Instant,
    step_index: usize,
    delay_override_ms: Option<u64>,
) -> bool {
    if !state.open {
        return false;
    }

    let Some(opened_at) = state.opened_at else {
        return true;
    };

    let delay_ms = delay_override_ms.unwrap_or((step_index as u64) * 150);
    now.saturating_duration_since(opened_at) >= Duration::from_millis(delay_ms)
}

pub fn chain_of_thought_header_default<'a, Message: 'a>(
    is_open: bool,
    props: ChainOfThoughtHeaderProps<'a>,
    _theme: &Theme,
) -> Element<'a, Message> {
    let mut content = row![]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    if props.show_brain_icon {
        content = content.push(lucide_icon(LucideIcon::Brain, 16.0));
    }

    let label = props.label_override.unwrap_or("Chain of Thought");
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

pub fn chain_of_thought_content<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ChainOfThoughtContentProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let text_color = theme.palette.popover_foreground;
    container(content)
        .width(Length::Fill)
        .padding(Padding {
            top: props.top_spacing,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .style(move |_theme| iced::widget::container::Style {
            text_color: Some(text_color),
            ..iced::widget::container::Style::default()
        })
        .into()
}

pub fn chain_of_thought_step<'a, Message: 'a>(
    props: ChainOfThoughtStepProps<'a>,
    content: Option<impl Into<Element<'a, Message>>>,
    is_visible: bool,
    _is_last: bool,
    theme: &Theme,
) -> Element<'a, Message> {
    if !is_visible {
        return container(column![]).width(Length::Fill).into();
    }

    let text_color = theme.palette.muted_foreground;
    let content_element = content.map(Into::into);

    let icon_column = {
        let icon = container(lucide_icon_styled(props.icon, 16.0, text_color))
            .width(16)
            .height(16)
            .center_x(Length::Fixed(16.0))
            .center_y(Length::Fixed(16.0));
        container(column![icon].spacing(8).align_x(Alignment::Center)).width(16)
    };

    let mut body = column![
        text(props.label)
            .size(14)
            .line_height(LineHeight::Relative(1.2))
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(text_color),
            })
    ]
    .spacing(8)
    .width(Length::Fill);

    if let Some(description) = props.description {
        body = body.push(
            text(description)
                .size(12)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(text_color),
                }),
        );
    }

    if let Some(content) = content_element {
        body = body.push(content);
    }

    container(row![icon_column, body].spacing(8).align_y(Alignment::Start))
        .style(move |_theme| iced::widget::container::Style {
            text_color: Some(text_color),
            ..iced::widget::container::Style::default()
        })
        .into()
}

pub fn chain_of_thought_search_results<'a, Message: 'a>(
    items: impl IntoIterator<Item = Element<'a, Message>>,
    props: ChainOfThoughtSearchResultsProps,
) -> Element<'a, Message> {
    row(items)
        .spacing(props.spacing)
        .align_y(Alignment::Center)
        .into()
}

pub fn chain_of_thought_search_result<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    props: ChainOfThoughtSearchResultProps<'a, Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    badge(label, props.badge, theme)
}

pub fn chain_of_thought_image<'a, Message: 'a>(
    media: impl Into<Element<'a, Message>>,
    props: ChainOfThoughtImageProps<'a>,
    theme: &Theme,
) -> Element<'a, Message> {
    let muted_color = theme.palette.muted;
    let border_color = theme.palette.border;
    let caption_color = theme.palette.muted_foreground;
    let radius = theme.radius.md;

    let image_box = container(media)
        .width(Length::Shrink)
        .max_width(props.max_width)
        .max_height(props.max_height)
        .padding(props.padding)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(muted_color)),
            border: iced::border::Border {
                color: border_color,
                width: 1.0,
                radius: radius.into(),
            },
            ..iced::widget::container::Style::default()
        });

    let mut block = column![image_box].spacing(8).width(Length::Shrink);

    if let Some(caption) = props.caption {
        block = block.push(text(caption).size(12).style(move |_theme: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(caption_color),
            }
        }));
    }

    container(block)
        .padding(Padding {
            top: 8.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .into()
}

pub fn chain_of_thought<'a, Message: Clone + 'a, F>(
    open: bool,
    header: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_open_change: Option<F>,
    chain_props: ChainOfThoughtProps,
    content_props: ChainOfThoughtContentProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    collapsible(
        open,
        header,
        chain_of_thought_content(content, content_props, theme),
        on_open_change,
        content_props.to_collapsible_content_props(),
        CollapsibleProps::new()
            .disabled(chain_props.disabled)
            .compact(chain_props.compact)
            .trigger_hover_highlight(false),
        theme,
    )
}

fn lucide_icon_styled<'a, Message: 'a>(
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
    fn chain_of_thought_defaults_match_reference() {
        let props = ChainOfThoughtProps::new();
        assert!(!props.default_open);
        assert!(!props.disabled);
        assert!(!props.compact);
    }

    #[test]
    fn state_respects_default_open() {
        let state_closed = ChainOfThoughtState::from_props(ChainOfThoughtProps::new());
        assert!(!state_closed.open);
        assert!(state_closed.opened_at.is_none());

        let state_open =
            ChainOfThoughtState::from_props(ChainOfThoughtProps::new().default_open(true));
        assert!(state_open.open);
        assert!(state_open.opened_at.is_some());
    }

    #[test]
    fn reducer_updates_open_state_and_timestamp() {
        let mut state = ChainOfThoughtState::from_props(ChainOfThoughtProps::new());
        let now = Instant::now();
        let effects = chain_of_thought_reduce(
            &mut state,
            ChainOfThoughtUpdate::OpenChanged { open: true, now },
            ChainOfThoughtProps::new(),
        );

        assert_eq!(effects.open_changed, Some(true));
        assert!(state.open);
        assert_eq!(state.opened_at, Some(now));
    }

    #[test]
    fn visibility_honors_stagger_delay() {
        let opened_at = Instant::now();
        let state = ChainOfThoughtState {
            open: true,
            opened_at: Some(opened_at),
        };

        assert!(!chain_of_thought_step_is_visible(
            &state,
            opened_at + Duration::from_millis(149),
            1,
            None
        ));
        assert!(chain_of_thought_step_is_visible(
            &state,
            opened_at + Duration::from_millis(150),
            1,
            None
        ));
    }
}
