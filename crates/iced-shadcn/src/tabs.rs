use std::time::{Duration, Instant};

use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::keyboard;
use iced::keyboard::key::{self, Key};
use iced::mouse;
use iced::touch;
use iced::widget::{button as button_widget, button as iced_button, column, container, text};
use iced::{Background, Color, Element, Event, Length, Point, Rectangle, Shadow, Size, Vector};

use crate::theme::Theme;
use crate::tokens::{
    AccentColor, accent_color, accent_high, accent_low, accent_soft, accent_text, is_dark,
};

const INDICATOR_ANIM_MS: u64 = 160;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsVariant {
    Underline,
    Soft,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

pub type TabsDirection = TabsOrientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsActivationMode {
    Automatic,
    Manual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsListLoop {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsSize {
    Size1,
    Size2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsJustify {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsHover {
    None,
    Subtle,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TabsListVariant {
    #[default]
    Pill,
    Line,
    Outline,
}

impl From<TabsVariant> for TabsListVariant {
    fn from(value: TabsVariant) -> Self {
        match value {
            TabsVariant::Underline => TabsListVariant::Line,
            TabsVariant::Soft => TabsListVariant::Pill,
            TabsVariant::Outline => TabsListVariant::Outline,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TabItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl TabItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TabsProps {
    pub variant: TabsVariant,
    pub orientation: TabsDirection,
    pub activation_mode: TabsActivationMode,
    pub list_loop: TabsListLoop,
    pub size: TabsSize,
    pub wrap: TabsWrap,
    pub justify: TabsJustify,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub full_width: bool,
    pub transparent_container: bool,
    pub hover: TabsHover,
    pub hover_intensity: Option<f32>,
}

impl Default for TabsProps {
    fn default() -> Self {
        Self {
            variant: TabsVariant::Underline,
            orientation: TabsDirection::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            list_loop: TabsListLoop::Enabled,
            size: TabsSize::Size2,
            wrap: TabsWrap::NoWrap,
            justify: TabsJustify::Start,
            color: AccentColor::Gray,
            high_contrast: false,
            full_width: false,
            transparent_container: false,
            hover: TabsHover::Soft,
            hover_intensity: None,
        }
    }
}

impl TabsProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: TabsDirection) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn activation_mode(mut self, activation_mode: TabsActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    pub fn list_loop(mut self, list_loop: TabsListLoop) -> Self {
        self.list_loop = list_loop;
        self
    }

    pub fn size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }

    pub fn wrap(mut self, wrap: TabsWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn justify(mut self, justify: TabsJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn transparent_container(mut self, transparent_container: bool) -> Self {
        self.transparent_container = transparent_container;
        self
    }

    pub fn hover(mut self, hover: TabsHover) -> Self {
        self.hover = hover;
        self
    }

    pub fn hover_intensity(mut self, intensity: f32) -> Self {
        self.hover_intensity = Some(intensity.clamp(0.0, 1.0));
        self
    }

    pub fn split(self) -> (TabsRootProps, TabsListProps) {
        (
            TabsRootProps {
                orientation: self.orientation,
                activation_mode: self.activation_mode,
                list_loop: self.list_loop,
            },
            TabsListProps {
                variant: self.variant.into(),
                size: self.size,
                wrap: self.wrap,
                justify: self.justify,
                color: self.color,
                high_contrast: self.high_contrast,
                full_width: self.full_width,
                transparent_container: self.transparent_container,
                hover: self.hover,
                hover_intensity: self.hover_intensity,
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TabsRootProps {
    pub orientation: TabsDirection,
    pub activation_mode: TabsActivationMode,
    pub list_loop: TabsListLoop,
}

impl Default for TabsRootProps {
    fn default() -> Self {
        Self {
            orientation: TabsDirection::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            list_loop: TabsListLoop::Enabled,
        }
    }
}

impl TabsRootProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: TabsDirection) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn activation_mode(mut self, activation_mode: TabsActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    pub fn list_loop(mut self, list_loop: TabsListLoop) -> Self {
        self.list_loop = list_loop;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TabsListProps {
    pub variant: TabsListVariant,
    pub size: TabsSize,
    pub wrap: TabsWrap,
    pub justify: TabsJustify,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub full_width: bool,
    pub transparent_container: bool,
    pub hover: TabsHover,
    pub hover_intensity: Option<f32>,
}

impl Default for TabsListProps {
    fn default() -> Self {
        Self {
            variant: TabsListVariant::Pill,
            size: TabsSize::Size2,
            wrap: TabsWrap::NoWrap,
            justify: TabsJustify::Start,
            color: AccentColor::Gray,
            high_contrast: false,
            full_width: false,
            transparent_container: false,
            hover: TabsHover::Soft,
            hover_intensity: None,
        }
    }
}

impl TabsListProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: TabsListVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }

    pub fn wrap(mut self, wrap: TabsWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn justify(mut self, justify: TabsJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn transparent_container(mut self, transparent_container: bool) -> Self {
        self.transparent_container = transparent_container;
        self
    }

    pub fn hover(mut self, hover: TabsHover) -> Self {
        self.hover = hover;
        self
    }

    pub fn hover_intensity(mut self, intensity: f32) -> Self {
        self.hover_intensity = Some(intensity.clamp(0.0, 1.0));
        self
    }
}

pub enum TabsTriggerContent<'a, Message> {
    Text(String),
    Element(Element<'a, Message>),
}

pub struct TabsTriggerItem<'a, Message> {
    pub value: String,
    pub content: TabsTriggerContent<'a, Message>,
    pub disabled: bool,
}

impl<'a, Message> TabsTriggerItem<'a, Message> {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            content: TabsTriggerContent::Text(label.into()),
            disabled: false,
        }
    }

    pub fn with_content(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            value: value.into(),
            content: TabsTriggerContent::Element(content.into()),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn tabs_trigger<'a, Message>(
    value: impl Into<String>,
    label: impl Into<String>,
) -> TabsTriggerItem<'a, Message> {
    TabsTriggerItem::new(value, label)
}

pub fn tabs_trigger_with<'a, Message>(
    value: impl Into<String>,
    content: impl Into<Element<'a, Message>>,
) -> TabsTriggerItem<'a, Message> {
    TabsTriggerItem::with_content(value, content)
}

pub struct TabsContentItem<'a, Message> {
    pub value: String,
    pub content: Element<'a, Message>,
}

impl<'a, Message> TabsContentItem<'a, Message> {
    pub fn new(value: impl Into<String>, content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            value: value.into(),
            content: content.into(),
        }
    }
}

pub fn tabs_content<'a, Message>(
    value: impl Into<String>,
    content: impl Into<Element<'a, Message>>,
) -> TabsContentItem<'a, Message> {
    TabsContentItem::new(value, content)
}

pub fn tabs_contents<'a, Message: Clone + 'a>(
    items: Vec<TabsContentItem<'a, Message>>,
    active: &str,
) -> Element<'a, Message> {
    if let Some(item) = items.into_iter().find(|item| item.value == active) {
        item.content
    } else {
        container(text(""))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into()
    }
}

impl TabsSize {
    fn padding(self, theme: &Theme) -> [f32; 2] {
        match self {
            TabsSize::Size1 => [theme.spacing.sm - 2.0, theme.spacing.md - 2.0],
            TabsSize::Size2 => [theme.spacing.sm, theme.spacing.md],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            TabsSize::Size1 => 12,
            TabsSize::Size2 => 13,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct TabsMetrics {
    list_padding: f32,
    gap: f32,
    line_gap: f32,
    indicator_height: f32,
    radius: f32,
}

fn tabs_metrics(props: TabsListProps, theme: &Theme) -> TabsMetrics {
    let list_padding = match props.size {
        TabsSize::Size1 => theme.styles.tabs.list_padding_size1,
        TabsSize::Size2 => theme.styles.tabs.list_padding_size2,
    };

    TabsMetrics {
        list_padding,
        gap: theme.styles.tabs.gap,
        line_gap: theme.styles.tabs.line_gap,
        indicator_height: theme.styles.tabs.indicator_height,
        radius: theme.radius.sm,
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn list_style(theme: &Theme, props: TabsListProps) -> (Option<Background>, Border, Shadow) {
    let palette = theme.palette;
    match props.variant {
        TabsListVariant::Pill => {
            let bg = if props.transparent_container {
                None
            } else if props.color == AccentColor::Gray {
                Some(palette.muted)
            } else if props.high_contrast {
                Some(accent_low(&palette, props.color))
            } else {
                Some(accent_soft(&palette, props.color))
            };
            (
                bg.map(Background::Color),
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: theme.radius.sm.into(),
                },
                Shadow::default(),
            )
        }
        TabsListVariant::Line => (
            None,
            Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: theme.radius.sm.into(),
            },
            Shadow::default(),
        ),
        TabsListVariant::Outline => (
            Some(Background::Color(palette.card)),
            Border {
                color: palette.border,
                width: 1.0,
                radius: theme.radius.sm.into(),
            },
            Shadow::default(),
        ),
    }
}

fn trigger_style(
    theme: &Theme,
    props: TabsListProps,
    is_active: bool,
    status: button_widget::Status,
) -> button_widget::Style {
    let palette = theme.palette;
    let accent = accent_color(&palette, props.color);
    let active_txt = accent_text(&palette, props.color);

    let is_hovered = matches!(status, button_widget::Status::Hovered);
    let is_pressed = matches!(status, button_widget::Status::Pressed);
    let is_disabled = matches!(status, button_widget::Status::Disabled);

    let muted_bg = if is_dark(&palette) {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.75)
    } else {
        apply_opacity(accent_low(&palette, AccentColor::Gray), 0.55)
    };

    let hover_strength = props.hover_intensity.unwrap_or(match props.hover {
        TabsHover::None => 0.0,
        TabsHover::Subtle => 0.6,
        TabsHover::Soft => 1.0,
    });

    let hover_pill = if hover_strength <= 0.0 {
        Background::Color(Color::TRANSPARENT)
    } else {
        Background::Color(apply_opacity(muted_bg, hover_strength))
    };

    let (background, border) = match props.variant {
        TabsListVariant::Line => {
            let bg = if is_pressed {
                Background::Color(apply_opacity(muted_bg, 0.9))
            } else if is_hovered {
                hover_pill
            } else {
                Background::Color(Color::TRANSPARENT)
            };

            (
                bg,
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: theme.radius.sm.into(),
                },
            )
        }
        TabsListVariant::Pill => {
            let bg = if is_active {
                Background::Color(palette.background)
            } else if is_hovered {
                hover_pill
            } else {
                Background::Color(Color::TRANSPARENT)
            };

            (
                bg,
                Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: theme.radius.sm.into(),
                },
            )
        }
        TabsListVariant::Outline => {
            let bg = if is_active {
                Background::Color(palette.card)
            } else if is_hovered {
                if hover_strength <= 0.0 {
                    Background::Color(Color::TRANSPARENT)
                } else {
                    Background::Color(apply_opacity(palette.card, 0.9 * hover_strength))
                }
            } else {
                Background::Color(Color::TRANSPARENT)
            };
            let border_color = if is_active { accent } else { palette.border };

            (
                bg,
                Border {
                    color: border_color,
                    width: 1.0,
                    radius: theme.radius.sm.into(),
                },
            )
        }
    };

    let mut text_color = if is_active {
        if props.variant == TabsListVariant::Line && props.color != AccentColor::Gray {
            active_txt
        } else {
            palette.foreground
        }
    } else {
        palette.muted_foreground
    };

    if is_disabled {
        text_color = apply_opacity(text_color, 0.6);
    }

    let shadow = if is_active && matches!(props.variant, TabsListVariant::Pill) {
        let strength = if is_dark(&palette) {
            (theme.styles.tabs.active_pill_shadow.opacity * 1.95).clamp(0.0, 1.0)
        } else {
            theme.styles.tabs.active_pill_shadow.opacity
        };
        Shadow {
            color: apply_opacity(palette.foreground, strength),
            offset: Vector::new(0.0, theme.styles.tabs.active_pill_shadow.offset_y),
            blur_radius: theme.styles.tabs.active_pill_shadow.blur_radius,
        }
    } else {
        Shadow::default()
    };

    button_widget::Style {
        background: Some(background),
        text_color,
        border,
        shadow,
        snap: true,
    }
}

#[derive(Clone, Debug)]
struct TabsTriggerMeta {
    value: String,
    disabled: bool,
}

#[derive(Debug, Default)]
struct TabsListState {
    focused: bool,
    focus_visible: bool,
    focused_index: Option<usize>,
    active_index: Option<usize>,
    trigger_bounds: Vec<Rectangle>,
    indicator_from: Option<Rectangle>,
    indicator_to: Option<Rectangle>,
    indicator_started: Option<Instant>,
    last_redraw: Option<Instant>,
}

struct TabsListWidget<'a, Message> {
    triggers: Vec<Element<'a, Message>>,
    items: Vec<TabsTriggerMeta>,
    active: String,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    root_props: TabsRootProps,
    list_props: TabsListProps,
    theme: Theme,
}

impl<'a, Message> TabsListWidget<'a, Message> {
    fn active_index(&self) -> Option<usize> {
        resolve_active_index(&self.items, &self.active)
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for TabsListWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        self.triggers.iter().map(|child| Tree::new(child)).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.triggers);
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<TabsListState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(TabsListState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let metrics = tabs_metrics(self.list_props, &self.theme);
        let max = limits.max();
        let count = self.triggers.len();
        let mut child_nodes = Vec::with_capacity(count);

        let full_width_each = if self.list_props.full_width
            && matches!(self.root_props.orientation, TabsOrientation::Horizontal)
            && matches!(self.list_props.wrap, TabsWrap::NoWrap)
            && count > 0
        {
            let available = (max.width
                - metrics.list_padding * 2.0
                - metrics.gap * (count.saturating_sub(1) as f32))
                .max(0.0);
            available / count as f32
        } else {
            0.0
        };

        for (index, child) in self.triggers.iter_mut().enumerate() {
            let child_limits = if full_width_each > 0.0 {
                layout::Limits::new(
                    Size::new(full_width_each, 0.0),
                    Size::new(full_width_each, max.height),
                )
            } else {
                layout::Limits::new(Size::ZERO, max)
            };

            let node =
                child
                    .as_widget_mut()
                    .layout(&mut tree.children[index], renderer, &child_limits);
            child_nodes.push(node);
        }

        let mut lines = Vec::new();
        match self.root_props.orientation {
            TabsOrientation::Horizontal => {
                let mut current = Line::default();
                for (index, node) in child_nodes.iter().enumerate() {
                    let size = node.size();
                    let proposed = if current.indices.is_empty() {
                        size.width
                    } else {
                        current.width + metrics.gap + size.width
                    };

                    let should_wrap = !matches!(self.list_props.wrap, TabsWrap::NoWrap)
                        && !current.indices.is_empty()
                        && (metrics.list_padding * 2.0 + proposed) > max.width;

                    if should_wrap {
                        lines.push(current);
                        current = Line::default();
                    }

                    current.indices.push(index);
                    current.width = if current.width == 0.0 {
                        size.width
                    } else {
                        current.width + metrics.gap + size.width
                    };
                    current.height = current.height.max(size.height);
                }
                if !current.indices.is_empty() {
                    lines.push(current);
                }
            }
            TabsOrientation::Vertical => {
                for (index, node) in child_nodes.iter().enumerate() {
                    let size = node.size();
                    lines.push(Line {
                        indices: vec![index],
                        width: size.width,
                        height: size.height,
                    });
                }
            }
        }

        if lines.is_empty() {
            lines.push(Line::default());
        }

        let content_width = lines.iter().map(|line| line.width).fold(0.0, f32::max);
        let content_height = match self.root_props.orientation {
            TabsOrientation::Horizontal => {
                lines.iter().map(|line| line.height).sum::<f32>()
                    + metrics.line_gap * (lines.len().saturating_sub(1) as f32)
            }
            TabsOrientation::Vertical => {
                lines.iter().map(|line| line.height).sum::<f32>()
                    + metrics.gap * (lines.len().saturating_sub(1) as f32)
            }
        };

        let mut width = content_width + metrics.list_padding * 2.0;
        let height = content_height + metrics.list_padding * 2.0;

        if self.list_props.full_width {
            width = max.width;
        }

        let min = limits.min();
        let max = limits.max();
        let size = Size::new(
            width.clamp(min.width, max.width.max(min.width)),
            height.clamp(min.height, max.height.max(min.height)),
        );

        let mut y = metrics.list_padding;
        let wrap_reverse = matches!(self.list_props.wrap, TabsWrap::WrapReverse);

        if wrap_reverse && matches!(self.root_props.orientation, TabsOrientation::Horizontal) {
            let total_lines_height = content_height;
            y = size.height - metrics.list_padding - total_lines_height;
        }

        let mut trigger_bounds = Vec::with_capacity(child_nodes.len());
        trigger_bounds.resize(child_nodes.len(), Rectangle::default());

        for (line_index, line) in lines.iter().enumerate() {
            let line_space = (size.width - metrics.list_padding * 2.0 - line.width).max(0.0);
            let offset = match self.list_props.justify {
                TabsJustify::Start => 0.0,
                TabsJustify::Center => line_space / 2.0,
                TabsJustify::End => line_space,
            };

            let mut x = metrics.list_padding + offset;
            for (pos, index) in line.indices.iter().enumerate() {
                let node_size = child_nodes[*index].size();
                let center_offset = (line.height - node_size.height).max(0.0) / 2.0;
                let child_y = y + center_offset;

                let node =
                    std::mem::replace(&mut child_nodes[*index], layout::Node::new(Size::ZERO));
                child_nodes[*index] = node.move_to(Point::new(x, child_y));
                trigger_bounds[*index] = child_nodes[*index].bounds();

                if pos + 1 < line.indices.len() {
                    x += node_size.width + metrics.gap;
                }
            }

            if line_index + 1 < lines.len() {
                let gap = match self.root_props.orientation {
                    TabsOrientation::Horizontal => metrics.line_gap,
                    TabsOrientation::Vertical => metrics.gap,
                };
                y += line.height + gap;
            }
        }

        let state = tree.state.downcast_mut::<TabsListState>();
        state.trigger_bounds = trigger_bounds;

        layout::Node::with_children(size, child_nodes)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<TabsListState>();

        for (index, child) in self.triggers.iter_mut().enumerate() {
            if let Some(child_layout) = layout.children().nth(index) {
                child.as_widget_mut().update(
                    &mut tree.children[index],
                    event,
                    child_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );
            }
        }

        let bounds = layout.bounds();
        let active_index = self.active_index();

        if active_index != state.active_index {
            let now = state.last_redraw.unwrap_or_else(Instant::now);
            let from = state
                .active_index
                .and_then(|idx| state.trigger_bounds.get(idx).copied());
            let to = active_index.and_then(|idx| state.trigger_bounds.get(idx).copied());

            state.indicator_from = from;
            state.indicator_to = to;
            state.indicator_started = if from.is_some() && to.is_some() {
                Some(now)
            } else {
                None
            };
            state.active_index = active_index;
            state.focused_index = active_index;

            if state.indicator_started.is_some() {
                shell.request_redraw();
            }
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) {
                    state.focused = true;
                    state.focus_visible = false;
                    state.focused_index = state.focused_index.or(active_index);
                } else if state.focused {
                    state.focused = false;
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if !state.focused {
                    return;
                }

                state.focus_visible = true;
                let current = state.focused_index.or(active_index);
                let loop_enabled = matches!(self.root_props.list_loop, TabsListLoop::Enabled);

                let next = match key {
                    Key::Named(key::Named::ArrowRight)
                        if matches!(self.root_props.orientation, TabsOrientation::Horizontal) =>
                    {
                        current
                            .and_then(|idx| next_enabled_index(&self.items, idx, 1, loop_enabled))
                    }
                    Key::Named(key::Named::ArrowLeft)
                        if matches!(self.root_props.orientation, TabsOrientation::Horizontal) =>
                    {
                        current
                            .and_then(|idx| next_enabled_index(&self.items, idx, -1, loop_enabled))
                    }
                    Key::Named(key::Named::ArrowDown)
                        if matches!(self.root_props.orientation, TabsOrientation::Vertical) =>
                    {
                        current
                            .and_then(|idx| next_enabled_index(&self.items, idx, 1, loop_enabled))
                    }
                    Key::Named(key::Named::ArrowUp)
                        if matches!(self.root_props.orientation, TabsOrientation::Vertical) =>
                    {
                        current
                            .and_then(|idx| next_enabled_index(&self.items, idx, -1, loop_enabled))
                    }
                    Key::Named(key::Named::Home) => first_enabled_index(&self.items),
                    Key::Named(key::Named::End) => last_enabled_index(&self.items),
                    Key::Named(key::Named::Enter) | Key::Named(key::Named::Space) => {
                        if matches!(self.root_props.activation_mode, TabsActivationMode::Manual)
                            && current.is_some()
                        {
                            current
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(next_index) = next {
                    state.focused_index = Some(next_index);

                    let should_activate = match key {
                        Key::Named(key::Named::Enter) | Key::Named(key::Named::Space) => true,
                        _ => matches!(
                            self.root_props.activation_mode,
                            TabsActivationMode::Automatic
                        ),
                    };

                    if should_activate && let Some(on_change) = self.on_value_change.as_ref() {
                        let value = self.items[next_index].value.clone();
                        shell.publish((on_change)(value));
                    }

                    shell.capture_event();
                }
            }
            Event::Window(iced::window::Event::RedrawRequested(now)) => {
                state.last_redraw = Some(*now);
                if let Some(started) = state.indicator_started {
                    let elapsed = now.saturating_duration_since(started);
                    if elapsed < Duration::from_millis(INDICATOR_ANIM_MS) {
                        shell.request_redraw();
                    } else {
                        state.indicator_started = None;
                        state.indicator_from = None;
                    }
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        for (index, child) in self.triggers.iter().enumerate() {
            if let Some(child_layout) = layout.children().nth(index) {
                let interaction = child.as_widget().mouse_interaction(
                    &tree.children[index],
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                );
                if interaction != mouse::Interaction::default() {
                    return interaction;
                }
            }
        }

        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let metrics = tabs_metrics(self.list_props, &self.theme);
        let (background, border, shadow) = list_style(&self.theme, self.list_props);

        if let Some(background) = background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border,
                    shadow,
                    ..renderer::Quad::default()
                },
                background,
            );
        } else if border.width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border,
                    shadow,
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }

        if matches!(self.list_props.variant, TabsListVariant::Line) {
            let line_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - 1.0,
                width: bounds.width,
                height: 1.0,
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: line_bounds,
                    ..renderer::Quad::default()
                },
                Background::Color(self.theme.palette.border),
            );
        }

        for (index, child) in self.triggers.iter().enumerate() {
            if let Some(child_layout) = layout.children().nth(index) {
                child.as_widget().draw(
                    &tree.children[index],
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }
        }

        let state = tree.state.downcast_ref::<TabsListState>();

        if matches!(self.list_props.variant, TabsListVariant::Line) && state.active_index.is_some()
        {
            let indicator_color = if self.list_props.high_contrast {
                accent_high(&self.theme.palette, self.list_props.color)
            } else {
                accent_color(&self.theme.palette, self.list_props.color)
            };

            let offset = Vector::new(bounds.x, bounds.y);
            let indicator = indicator_rect(state, self.root_props, metrics, offset);
            if let Some(rect) = indicator {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: rect,
                        ..renderer::Quad::default()
                    },
                    Background::Color(indicator_color),
                );
            }
        }

        if state.focused
            && state.focus_visible
            && let Some(focus_index) = state.focused_index
            && let Some(rect) = state.trigger_bounds.get(focus_index)
        {
            let focus_color = if self.list_props.high_contrast {
                accent_high(&self.theme.palette, self.list_props.color)
            } else {
                self.theme.palette.ring
            };

            let focus_rect = Rectangle {
                x: rect.x + bounds.x - 2.0,
                y: rect.y + bounds.y - 2.0,
                width: rect.width + 4.0,
                height: rect.height + 4.0,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: focus_rect,
                    border: Border {
                        color: focus_color,
                        width: 2.0,
                        radius: metrics.radius.into(),
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }
    }
}

impl<'a, Message: Clone + 'a> From<TabsListWidget<'a, Message>> for Element<'a, Message> {
    fn from(widget: TabsListWidget<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}

#[derive(Default)]
struct Line {
    indices: Vec<usize>,
    width: f32,
    height: f32,
}

fn resolve_active_index(items: &[TabsTriggerMeta], active: &str) -> Option<usize> {
    items
        .iter()
        .position(|item| item.value == active && !item.disabled)
        .or_else(|| items.iter().position(|item| !item.disabled))
}

fn first_enabled_index(items: &[TabsTriggerMeta]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

fn last_enabled_index(items: &[TabsTriggerMeta]) -> Option<usize> {
    items.iter().rposition(|item| !item.disabled)
}

fn next_enabled_index(
    items: &[TabsTriggerMeta],
    start: usize,
    direction: i32,
    loop_enabled: bool,
) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    let mut index = start as i32;
    for _ in 0..items.len() {
        index += direction;
        if index < 0 || index >= items.len() as i32 {
            if loop_enabled {
                index = if direction > 0 {
                    0
                } else {
                    items.len() as i32 - 1
                };
            } else {
                return None;
            }
        }

        let idx = index as usize;
        if !items[idx].disabled {
            return Some(idx);
        }
    }

    None
}

fn indicator_rect(
    state: &TabsListState,
    root_props: TabsRootProps,
    metrics: TabsMetrics,
    offset: Vector,
) -> Option<Rectangle> {
    let to = state.indicator_to;
    let from = state.indicator_from;
    let now = state.last_redraw.unwrap_or_else(Instant::now);

    let rect = if let Some(started) = state.indicator_started {
        let duration = Duration::from_millis(INDICATOR_ANIM_MS);
        let t = (now.saturating_duration_since(started).as_secs_f32() / duration.as_secs_f32())
            .clamp(0.0, 1.0);
        if let (Some(from), Some(to)) = (from, to) {
            Some(lerp_rect(from, to, t))
        } else {
            to
        }
    } else {
        to
    }?;

    let rect = match root_props.orientation {
        TabsOrientation::Horizontal => Rectangle {
            x: rect.x + offset.x,
            y: rect.y + offset.y + rect.height - metrics.indicator_height,
            width: rect.width,
            height: metrics.indicator_height,
        },
        TabsOrientation::Vertical => Rectangle {
            x: rect.x + offset.x,
            y: rect.y + offset.y,
            width: metrics.indicator_height,
            height: rect.height,
        },
    };

    Some(rect)
}

fn lerp_rect(from: Rectangle, to: Rectangle, t: f32) -> Rectangle {
    Rectangle {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
        width: from.width + (to.width - from.width) * t,
        height: from.height + (to.height - from.height) * t,
    }
}

pub fn tabs_list<'a, Message: Clone + 'a, F>(
    items: Vec<TabsTriggerItem<'a, Message>>,
    active: &str,
    on_value_change: Option<F>,
    root_props: TabsRootProps,
    list_props: TabsListProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let theme = theme.clone();
    let on_value_change =
        on_value_change.map(|f| Box::new(f) as Box<dyn Fn(String) -> Message + 'a>);

    let resolved_active = items
        .iter()
        .find(|item| item.value == active && !item.disabled)
        .or_else(|| items.iter().find(|item| !item.disabled))
        .map(|item| item.value.clone())
        .unwrap_or_else(|| active.to_string());

    let mut triggers = Vec::with_capacity(items.len());
    let mut meta = Vec::with_capacity(items.len());

    for item in items {
        let is_disabled = item.disabled || on_value_change.is_none();
        let content = match item.content {
            TabsTriggerContent::Text(label) => text(label).size(list_props.size.text_size()).into(),
            TabsTriggerContent::Element(element) => element,
        };

        let is_active = item.value == resolved_active;
        let mut trigger = iced_button(
            container(content)
                .center_x(Length::Shrink)
                .center_y(Length::Shrink),
        )
        .padding(list_props.size.padding(&theme))
        .style({
            let theme = theme.clone();
            move |_iced_theme, status| trigger_style(&theme, list_props, is_active, status)
        });

        if !is_disabled && let Some(on_change) = on_value_change.as_ref() {
            trigger = trigger.on_press((on_change)(item.value.clone()));
        }

        let trigger: Element<'a, Message> = trigger.into();
        triggers.push(trigger);
        meta.push(TabsTriggerMeta {
            value: item.value,
            disabled: is_disabled,
        });
    }

    TabsListWidget {
        triggers,
        items: meta,
        active: resolved_active,
        on_value_change,
        root_props,
        list_props,
        theme,
    }
    .into()
}

pub fn tabs_root<'a, Message: Clone + 'a>(
    list: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![list.into(), content.into()]
        .spacing(8)
        .width(Length::Fill)
        .into()
}

pub fn tabs<'a, Message: Clone + 'a, F>(
    items: Vec<TabItem>,
    active: &str,
    on_value_change: Option<F>,
    props: TabsProps,
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let (root_props, list_props) = props.split();
    let list_items = items
        .into_iter()
        .map(|item| TabsTriggerItem::new(item.id, item.label).disabled(item.disabled))
        .collect();

    let list = tabs_list(
        list_items,
        active,
        on_value_change,
        root_props,
        list_props,
        theme,
    );
    tabs_root(list, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_active_prefers_enabled_match() {
        let items = vec![
            TabsTriggerMeta {
                value: "Size1".to_string(),
                disabled: true,
            },
            TabsTriggerMeta {
                value: "Size2".to_string(),
                disabled: false,
            },
        ];

        assert_eq!(resolve_active_index(&items, "Size1"), Some(1));
        assert_eq!(resolve_active_index(&items, "Size2"), Some(1));
    }

    #[test]
    fn next_enabled_respects_loop() {
        let items = vec![
            TabsTriggerMeta {
                value: "Size1".to_string(),
                disabled: false,
            },
            TabsTriggerMeta {
                value: "Size2".to_string(),
                disabled: true,
            },
            TabsTriggerMeta {
                value: "Size3".to_string(),
                disabled: false,
            },
        ];

        assert_eq!(next_enabled_index(&items, 0, 1, false), Some(2));
        assert_eq!(next_enabled_index(&items, 2, 1, false), None);
        assert_eq!(next_enabled_index(&items, 2, 1, true), Some(0));
    }

    #[test]
    fn next_enabled_moves_backward() {
        let items = vec![
            TabsTriggerMeta {
                value: "Size1".to_string(),
                disabled: false,
            },
            TabsTriggerMeta {
                value: "Size2".to_string(),
                disabled: false,
            },
        ];

        assert_eq!(next_enabled_index(&items, 1, -1, true), Some(0));
        assert_eq!(next_enabled_index(&items, 0, -1, true), Some(1));
    }
}
