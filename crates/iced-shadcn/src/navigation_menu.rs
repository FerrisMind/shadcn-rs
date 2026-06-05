use iced::time::{Duration, Instant};

use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text as advanced_text;
use iced::advanced::text::Renderer as TextRenderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::alignment;
use iced::border::Border;
use iced::keyboard;
use iced::keyboard::key::{self, Key};
use iced::mouse;
use iced::touch;
use iced::widget::{button as iced_button, container, text};
use iced::{
    Background, Color, Element, Event, Font, Length, Padding, Point, Rectangle, Shadow, Size,
    Vector,
};
use lucide_icons::Icon as LucideIcon;

use crate::button::ButtonRadius;
use crate::theme::Theme;
use crate::tokens::{AccentColor, accent_high, accent_soft, accent_text};

const INDICATOR_ANIM_MS: u64 = 160;
const MOTION_ANIM_MS: u64 = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuJustify {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuSize {
    Size1,
    Size2,
}

impl NavigationMenuSize {
    fn padding(self) -> [f32; 2] {
        match self {
            NavigationMenuSize::Size1 => [6.0, 10.0],
            NavigationMenuSize::Size2 => [8.0, 14.0],
        }
    }

    fn text_size(self) -> u32 {
        match self {
            NavigationMenuSize::Size1 => 12,
            NavigationMenuSize::Size2 => 13,
        }
    }

    fn icon_size(self) -> u32 {
        match self {
            NavigationMenuSize::Size1 => 10,
            NavigationMenuSize::Size2 => 12,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationMenuLinkVariant {
    Default,
    Trigger,
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuProps {
    pub orientation: NavigationMenuOrientation,
    pub delay_duration_ms: u64,
    pub skip_delay_duration_ms: u64,
    pub close_delay_ms: u64,
    pub viewport: bool,
    pub indicator: bool,
    pub default_value: Option<&'static str>,
}

impl Default for NavigationMenuProps {
    fn default() -> Self {
        Self {
            orientation: NavigationMenuOrientation::Horizontal,
            delay_duration_ms: 200,
            skip_delay_duration_ms: 300,
            close_delay_ms: 0,
            viewport: true,
            indicator: false,
            default_value: None,
        }
    }
}

impl NavigationMenuProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn delay_duration_ms(mut self, delay: u64) -> Self {
        self.delay_duration_ms = delay;
        self
    }

    pub fn skip_delay_duration_ms(mut self, delay: u64) -> Self {
        self.skip_delay_duration_ms = delay;
        self
    }

    pub fn close_delay_ms(mut self, delay: u64) -> Self {
        self.close_delay_ms = delay;
        self
    }

    pub fn viewport(mut self, viewport: bool) -> Self {
        self.viewport = viewport;
        self
    }

    pub fn indicator(mut self, indicator: bool) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn viewport_component(mut self, viewport: NavigationMenuViewport) -> Self {
        self.viewport = viewport.enabled;
        self
    }

    pub fn indicator_component(mut self, indicator: NavigationMenuIndicator) -> Self {
        self.indicator = indicator.enabled;
        self
    }

    pub fn default_value(mut self, value: &'static str) -> Self {
        self.default_value = Some(value);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuViewport {
    pub enabled: bool,
}

impl NavigationMenuViewport {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl Default for NavigationMenuViewport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuIndicator {
    pub enabled: bool,
}

impl NavigationMenuIndicator {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl Default for NavigationMenuIndicator {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuListProps {
    pub size: NavigationMenuSize,
    pub wrap: NavigationMenuWrap,
    pub justify: NavigationMenuJustify,
    pub gap: f32,
    pub color: AccentColor,
    pub high_contrast: bool,
    pub full_width: bool,
    pub padding: f32,
}

impl Default for NavigationMenuListProps {
    fn default() -> Self {
        Self {
            size: NavigationMenuSize::Size2,
            wrap: NavigationMenuWrap::NoWrap,
            justify: NavigationMenuJustify::Center,
            gap: 4.0,
            color: AccentColor::Gray,
            high_contrast: false,
            full_width: false,
            padding: 0.0,
        }
    }
}

impl NavigationMenuListProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: NavigationMenuSize) -> Self {
        self.size = size;
        self
    }

    pub fn wrap(mut self, wrap: NavigationMenuWrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn justify(mut self, justify: NavigationMenuJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
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

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavigationMenuContentProps {
    pub width: Option<f32>,
    pub max_height: Option<f32>,
    pub side: NavigationMenuSide,
    pub align: NavigationMenuAlign,
    pub side_offset: f32,
    pub align_offset: f32,
    pub padding: f32,
    pub collision_padding: f32,
}

impl Default for NavigationMenuContentProps {
    fn default() -> Self {
        Self {
            width: None,
            max_height: None,
            side: NavigationMenuSide::Bottom,
            align: NavigationMenuAlign::Start,
            side_offset: 6.0,
            align_offset: 0.0,
            padding: 8.0,
            collision_padding: 8.0,
        }
    }
}

impl NavigationMenuContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn side(mut self, side: NavigationMenuSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: NavigationMenuAlign) -> Self {
        self.align = align;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn collision_padding(mut self, padding: f32) -> Self {
        self.collision_padding = padding.max(0.0);
        self
    }
}

pub struct NavigationMenuContent<'a, Message> {
    pub content: Element<'a, Message>,
    pub props: NavigationMenuContentProps,
}

impl<'a, Message> NavigationMenuContent<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            props: NavigationMenuContentProps::new(),
        }
    }

    pub fn props(mut self, props: NavigationMenuContentProps) -> Self {
        self.props = props;
        self
    }
}

impl<'a, Message> From<Element<'a, Message>> for NavigationMenuContent<'a, Message> {
    fn from(content: Element<'a, Message>) -> Self {
        NavigationMenuContent::new(content)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuLinkProps {
    pub variant: NavigationMenuLinkVariant,
    pub size: NavigationMenuSize,
    pub padding: f32,
    pub rounding: Option<ButtonRadius>,
    pub full_width: bool,
    pub active: bool,
    pub disabled: bool,
}

impl Default for NavigationMenuLinkProps {
    fn default() -> Self {
        Self {
            variant: NavigationMenuLinkVariant::Default,
            size: NavigationMenuSize::Size2,
            padding: 6.0,
            rounding: Some(ButtonRadius::Small),
            full_width: false,
            active: false,
            disabled: false,
        }
    }
}

impl NavigationMenuLinkProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: NavigationMenuLinkVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: NavigationMenuSize) -> Self {
        self.size = size;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    pub fn rounding(mut self, rounding: ButtonRadius) -> Self {
        self.rounding = Some(rounding);
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn navigation_menu_trigger_style() -> NavigationMenuLinkProps {
    NavigationMenuLinkProps::new().variant(NavigationMenuLinkVariant::Trigger)
}

pub fn navigation_menu_viewport() -> NavigationMenuViewport {
    NavigationMenuViewport::new()
}

pub fn navigation_menu_indicator() -> NavigationMenuIndicator {
    NavigationMenuIndicator::new()
}

pub fn navigation_menu_content<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> NavigationMenuContent<'a, Message> {
    NavigationMenuContent::new(content)
}

pub enum NavigationMenuTriggerContent<'a, Message> {
    Text(String),
    Element(Element<'a, Message>),
}
pub struct NavigationMenuTriggerItem<'a, Message> {
    pub value: String,
    pub content: NavigationMenuTriggerContent<'a, Message>,
    pub content_props: NavigationMenuContentProps,
    pub disabled: bool,
    pub show_chevron: bool,
}

impl<'a, Message> NavigationMenuTriggerItem<'a, Message> {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            content: NavigationMenuTriggerContent::Text(label.into()),
            content_props: NavigationMenuContentProps::new(),
            disabled: false,
            show_chevron: true,
        }
    }

    pub fn with_content(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            value: value.into(),
            content: NavigationMenuTriggerContent::Element(content.into()),
            content_props: NavigationMenuContentProps::new(),
            disabled: false,
            show_chevron: true,
        }
    }

    pub fn content_props(mut self, props: NavigationMenuContentProps) -> Self {
        self.content_props = props;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_chevron(mut self, show: bool) -> Self {
        self.show_chevron = show;
        self
    }
}

pub type NavigationMenuTrigger<'a, Message> = NavigationMenuTriggerItem<'a, Message>;

pub fn navigation_menu_trigger<'a, Message>(
    value: impl Into<String>,
    label: impl Into<String>,
) -> NavigationMenuTriggerItem<'a, Message> {
    NavigationMenuTriggerItem::new(value, label)
}

pub fn navigation_menu_trigger_with<'a, Message>(
    value: impl Into<String>,
    content: impl Into<Element<'a, Message>>,
) -> NavigationMenuTriggerItem<'a, Message> {
    NavigationMenuTriggerItem::with_content(value, content)
}

pub struct NavigationMenuLinkItem<'a, Message> {
    pub value: String,
    pub content: Element<'a, Message>,
    pub on_press: Option<Message>,
    pub props: NavigationMenuLinkProps,
}

impl<'a, Message> NavigationMenuLinkItem<'a, Message> {
    pub fn new(
        value: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
        on_press: Option<Message>,
    ) -> Self {
        Self {
            value: value.into(),
            content: content.into(),
            on_press,
            props: NavigationMenuLinkProps::new().variant(NavigationMenuLinkVariant::Trigger),
        }
    }

    pub fn props(mut self, props: NavigationMenuLinkProps) -> Self {
        self.props = props;
        self
    }
}

pub type NavigationMenuLink<'a, Message> = NavigationMenuLinkItem<'a, Message>;

pub enum NavigationMenuItem<'a, Message> {
    Trigger {
        trigger: NavigationMenuTriggerItem<'a, Message>,
        content: NavigationMenuContent<'a, Message>,
    },
    Link(NavigationMenuLinkItem<'a, Message>),
}

pub type NavigationMenuList<'a, Message> = Vec<NavigationMenuItem<'a, Message>>;
pub type NavigationMenuRoot<'a, Message> = Element<'a, Message>;

pub fn navigation_menu_list<'a, Message>(
    items: Vec<NavigationMenuItem<'a, Message>>,
) -> NavigationMenuList<'a, Message> {
    items
}

pub fn navigation_menu_item<'a, Message>(
    trigger: NavigationMenuTriggerItem<'a, Message>,
    content: impl Into<NavigationMenuContent<'a, Message>>,
) -> NavigationMenuItem<'a, Message> {
    NavigationMenuItem::Trigger {
        trigger,
        content: content.into(),
    }
}

pub fn navigation_menu_link_item<'a, Message>(
    value: impl Into<String>,
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> NavigationMenuItem<'a, Message> {
    NavigationMenuItem::Link(NavigationMenuLinkItem::new(value, content, on_press))
}

pub fn navigation_menu_link<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
    props: NavigationMenuLinkProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let theme = theme.clone();
    let element: Element<'a, Message> = content.into();
    let size_hint = element.as_widget().size_hint();
    let width = if props.full_width {
        Length::Fill
    } else {
        size_hint.width
    };

    NavigationMenuLinkWidget {
        content: element,
        on_press,
        props,
        theme,
        width,
        height: size_hint.height,
    }
    .into()
}
#[derive(Clone, Debug)]
struct NavItemMeta<Message> {
    value: String,
    kind: NavItemKind,
    disabled: bool,
    content_index: Option<usize>,
    content_props: NavigationMenuContentProps,
    link_message: Option<Message>,
}

pub fn navigation_menu_root<'a, Message: Clone + 'a, F>(
    items: NavigationMenuList<'a, Message>,
    value: Option<&'a str>,
    on_value_change: Option<F>,
    root_props: NavigationMenuProps,
    list_props: NavigationMenuListProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    navigation_menu(items, value, on_value_change, root_props, list_props, theme)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NavItemKind {
    Trigger,
    Link,
}

#[derive(Clone, Copy, Debug)]
struct NavigationMenuMetrics {
    list_padding: f32,
    gap: f32,
    line_gap: f32,
    indicator_size: f32,
    indicator_offset: f32,
    radius: f32,
}

fn navigation_menu_metrics(props: NavigationMenuListProps, theme: &Theme) -> NavigationMenuMetrics {
    NavigationMenuMetrics {
        list_padding: props.padding,
        gap: props.gap,
        line_gap: props.gap,
        indicator_size: 8.0,
        indicator_offset: 6.0,
        radius: theme.radius.md,
    }
}

fn apply_opacity(color: Color, opacity: f32) -> Color {
    Color {
        a: color.a * opacity,
        ..color
    }
}

fn link_style(
    theme: &Theme,
    props: NavigationMenuLinkProps,
    status: iced_button::Status,
) -> iced_button::Style {
    let palette = theme.palette;
    let accent_bg = palette.accent;
    let accent_fg = palette.accent_foreground;
    let active_bg = apply_opacity(accent_bg, 0.5);
    let (mut background, mut text_color) = match props.variant {
        NavigationMenuLinkVariant::Trigger => (palette.background, palette.foreground),
        NavigationMenuLinkVariant::Default => (Color::TRANSPARENT, palette.foreground),
    };

    let hovered = matches!(
        status,
        iced_button::Status::Hovered | iced_button::Status::Pressed
    );
    if hovered {
        background = accent_bg;
        text_color = accent_fg;
    }

    if props.active {
        background = active_bg;
        text_color = accent_fg;
    }

    if props.disabled {
        background = if props.variant == NavigationMenuLinkVariant::Trigger {
            palette.background
        } else {
            Color::TRANSPARENT
        };
        text_color = apply_opacity(text_color, 0.6);
    }

    iced_button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: props.rounding.unwrap_or(ButtonRadius::Small).value().into(),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

#[derive(Debug, Default)]
struct NavigationMenuLinkState {
    is_pressed: bool,
}

struct NavigationMenuLinkWidget<'a, Message> {
    content: Element<'a, Message>,
    on_press: Option<Message>,
    props: NavigationMenuLinkProps,
    theme: Theme,
    width: Length,
    height: Length,
}

#[derive(Debug, Default)]
struct NavigationMenuTriggerState {
    is_pressed: bool,
    is_open: bool,
}

struct NavigationMenuTriggerWidget<'a, Message> {
    content: Element<'a, Message>,
    show_chevron: bool,
    icon_size: u32,
    size: NavigationMenuSize,
    disabled: bool,
    list_props: NavigationMenuListProps,
    theme: Theme,
}

impl<Message: Clone> Widget<Message, iced::Theme, iced::Renderer>
    for NavigationMenuLinkWidget<'_, Message>
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(NavigationMenuLinkState::default())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<NavigationMenuLinkState>()
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = Padding {
            top: self.props.padding,
            right: self.props.padding,
            bottom: self.props.padding,
            left: self.props.padding,
        };

        layout::padded(limits, self.width, self.height, padding, |limits| {
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        })
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
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap_or(layout),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        if self.props.disabled {
            let state = tree.state.downcast_mut::<NavigationMenuLinkState>();
            state.is_pressed = false;
            return;
        }

        if self.on_press.is_none() {
            let state = tree.state.downcast_mut::<NavigationMenuLinkState>();
            state.is_pressed = false;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if cursor.is_over(layout.bounds()) =>
            {
                let state = tree.state.downcast_mut::<NavigationMenuLinkState>();
                state.is_pressed = true;
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<NavigationMenuLinkState>();
                if state.is_pressed {
                    state.is_pressed = false;
                    if cursor.is_over(layout.bounds())
                        && let Some(message) = self.on_press.clone()
                    {
                        shell.publish(message);
                    }
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<NavigationMenuLinkState>();
                state.is_pressed = false;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.props.disabled {
            return mouse::Interaction::default();
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
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap_or(layout);

        let status = if self.props.disabled {
            iced_button::Status::Disabled
        } else if cursor.is_over(bounds) {
            let state = tree.state.downcast_ref::<NavigationMenuLinkState>();
            if state.is_pressed {
                iced_button::Status::Pressed
            } else {
                iced_button::Status::Hovered
            }
        } else {
            iced_button::Status::Active
        };

        let style = link_style(&self.theme, self.props, status);

        if style.background.is_some() || style.border.width > 0.0 || style.shadow.color.a > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                    snap: style.snap,
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            content_layout,
            cursor,
            viewport,
        );
    }
}

impl<'a, Message: Clone + 'a> From<NavigationMenuLinkWidget<'a, Message>> for Element<'a, Message> {
    fn from(widget: NavigationMenuLinkWidget<'a, Message>) -> Self {
        Element::new(widget)
    }
}

impl<Message: Clone> Widget<Message, iced::Theme, iced::Renderer>
    for NavigationMenuTriggerWidget<'_, Message>
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(NavigationMenuTriggerState::default())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<NavigationMenuTriggerState>()
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = self.size.padding();
        let padding = Padding {
            top: padding[0],
            right: padding[1],
            bottom: padding[0],
            left: padding[1],
        };
        let caret_width = if self.show_chevron {
            self.icon_size as f32 + 4.0
        } else {
            0.0
        };

        layout::padded(limits, Length::Shrink, Length::Shrink, padding, |limits| {
            let max = limits.max();
            let content_limits = layout::Limits::new(
                Size::ZERO,
                Size::new((max.width - caret_width).max(0.0), max.height),
            );
            let content_node = self.content.as_widget_mut().layout(
                &mut tree.children[0],
                renderer,
                &content_limits,
            );
            let size = content_node.size();

            layout::Node::with_children(
                Size::new(size.width + caret_width, size.height),
                vec![content_node],
            )
        })
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
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap_or(layout),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.disabled {
            let state = tree.state.downcast_mut::<NavigationMenuTriggerState>();
            state.is_pressed = false;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if cursor.is_over(layout.bounds()) =>
            {
                let state = tree.state.downcast_mut::<NavigationMenuTriggerState>();
                state.is_pressed = true;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<NavigationMenuTriggerState>();
                if state.is_pressed {
                    state.is_pressed = false;
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<NavigationMenuTriggerState>();
                state.is_pressed = false;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.disabled {
            return mouse::Interaction::default();
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
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap_or(layout);
        let state = tree.state.downcast_ref::<NavigationMenuTriggerState>();

        let palette = self.theme.palette;
        let accent_bg = if self.list_props.color == AccentColor::Gray {
            palette.accent
        } else {
            accent_soft(&palette, self.list_props.color)
        };
        let accent_fg = if self.list_props.color == AccentColor::Gray {
            palette.accent_foreground
        } else {
            accent_text(&palette, self.list_props.color)
        };

        let is_hovered = cursor.is_over(bounds);
        let mut background = palette.background;
        let mut text_color = palette.foreground;

        if self.disabled {
            text_color = apply_opacity(text_color, 0.6);
        } else if state.is_open {
            background = apply_opacity(accent_bg, 0.5);
            text_color = accent_fg;
        } else if is_hovered || state.is_pressed {
            background = accent_bg;
            text_color = accent_fg;
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: self.theme.radius.md.into(),
                },
                shadow: Shadow::default(),
                snap: true,
            },
            Background::Color(background),
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style { text_color },
            content_layout,
            cursor,
            viewport,
        );

        if self.show_chevron {
            let caret_icon = if state.is_open {
                LucideIcon::ChevronUp
            } else {
                LucideIcon::ChevronDown
            };
            let icon_size = self.icon_size as f32;
            let caret_bounds = Rectangle {
                x: bounds.x + bounds.width - self.size.padding()[1] - icon_size,
                y: bounds.y,
                width: icon_size,
                height: bounds.height,
            };
            let center = caret_bounds.center();

            renderer.fill_text(
                advanced_text::Text {
                    content: char::from(caret_icon).to_string(),
                    font: Font::with_name("lucide"),
                    size: self.icon_size.into(),
                    line_height: advanced_text::LineHeight::Absolute(self.icon_size.into()),
                    bounds: caret_bounds.size(),
                    align_x: advanced_text::Alignment::Center,
                    align_y: alignment::Vertical::Center,
                    shaping: advanced_text::Shaping::Basic,
                    wrapping: advanced_text::Wrapping::default(),
                },
                Point::new(center.x, center.y),
                text_color,
                *viewport,
            );
        }
    }
}

impl<'a, Message: Clone + 'a> From<NavigationMenuTriggerWidget<'a, Message>>
    for Element<'a, Message>
{
    fn from(widget: NavigationMenuTriggerWidget<'a, Message>) -> Self {
        Element::new(widget)
    }
}

#[derive(Debug, Default)]
struct NavigationMenuState {
    open_value: Option<String>,
    open_index: Option<usize>,
    focused: bool,
    focus_visible: bool,
    focused_index: Option<usize>,
    hovered_index: Option<usize>,
    trigger_bounds: Vec<Rectangle>,
    indicator_from: Option<Rectangle>,
    indicator_to: Option<Rectangle>,
    indicator_started: Option<Instant>,
    motion: Option<Motion>,
    pending_open: Option<PendingOpen>,
    pending_close: Option<Instant>,
    last_close_at: Option<Instant>,
    viewport_bounds: Option<Rectangle>,
    viewport_size: Option<Size>,
    viewport_hovered: bool,
    last_redraw: Option<Instant>,
    initialized: bool,
}

#[derive(Clone, Copy, Debug)]
struct PendingOpen {
    index: usize,
    started_at: Instant,
}

#[derive(Clone, Copy, Debug)]
struct Motion {
    direction: i32,
    started_at: Instant,
}
pub fn navigation_menu<'a, Message: Clone + 'a, F>(
    items: NavigationMenuList<'a, Message>,
    value: Option<&'a str>,
    on_value_change: Option<F>,
    root_props: NavigationMenuProps,
    list_props: NavigationMenuListProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let theme = theme.clone();
    let on_value_change =
        on_value_change.map(|f| Box::new(f) as Box<dyn Fn(String) -> Message + 'a>);

    let current_value = value.and_then(|val| (!val.is_empty()).then_some(val.to_string()));

    let mut triggers = Vec::new();
    let mut contents = Vec::new();
    let mut metas = Vec::new();

    for item in items {
        match item {
            NavigationMenuItem::Trigger { trigger, content } => {
                let NavigationMenuTriggerItem {
                    value,
                    content: trigger_content,
                    content_props,
                    disabled,
                    show_chevron,
                } = trigger;
                let content_index = contents.len();
                let resolved_props = if content.props == NavigationMenuContentProps::default() {
                    content_props
                } else {
                    content.props
                };
                let content_widget = apply_content_props(content.content, resolved_props);
                contents.push(content_widget);

                let trigger_element = build_trigger_element(
                    trigger_content,
                    show_chevron,
                    disabled,
                    list_props,
                    &theme,
                );
                triggers.push(trigger_element);

                metas.push(NavItemMeta {
                    value,
                    kind: NavItemKind::Trigger,
                    disabled,
                    content_index: Some(content_index),
                    content_props: resolved_props,
                    link_message: None,
                });
            }
            NavigationMenuItem::Link(link) => {
                let link_props = link.props;
                let trigger_element =
                    navigation_menu_link(link.content, link.on_press.clone(), link_props, &theme);
                triggers.push(trigger_element);

                metas.push(NavItemMeta {
                    value: link.value,
                    kind: NavItemKind::Link,
                    disabled: link_props.disabled,
                    content_index: None,
                    content_props: NavigationMenuContentProps::new(),
                    link_message: link.on_press.clone(),
                });
            }
        }
    }

    NavigationMenuWidget {
        triggers,
        contents,
        items: metas,
        value: current_value,
        on_value_change,
        root_props,
        list_props,
        theme,
    }
    .into()
}

fn apply_content_props<'a, Message: 'a>(
    content: Element<'a, Message>,
    props: NavigationMenuContentProps,
) -> Element<'a, Message> {
    let mut wrapper = container(content).padding(props.padding);

    if let Some(width) = props.width {
        wrapper = wrapper.width(Length::Fixed(width.max(0.0)));
    }
    if let Some(max_height) = props.max_height {
        wrapper = wrapper.max_height(max_height.max(0.0));
    }

    wrapper.into()
}

struct NavigationMenuWidget<'a, Message> {
    triggers: Vec<Element<'a, Message>>,
    contents: Vec<Element<'a, Message>>,
    items: Vec<NavItemMeta<Message>>,
    value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    root_props: NavigationMenuProps,
    list_props: NavigationMenuListProps,
    theme: Theme,
}

impl<'a, Message> NavigationMenuWidget<'a, Message> {
    fn is_controlled(&self) -> bool {
        self.value.is_some()
    }

    fn current_value<'b>(&'b self, state: &'b NavigationMenuState) -> Option<&'b str> {
        self.value
            .as_deref()
            .or(state.open_value.as_deref())
            .filter(|val| !val.is_empty())
    }

    fn open_index(&self, state: &NavigationMenuState) -> Option<usize> {
        resolve_open_index(&self.items, self.current_value(state))
    }

    fn set_open_value(
        &self,
        state: &mut NavigationMenuState,
        shell: &mut Shell<'_, Message>,
        next: Option<String>,
    ) {
        let next_value = next.clone().unwrap_or_default();
        let current = self.current_value(state).unwrap_or("");
        if current == next_value {
            return;
        }

        if !self.is_controlled() {
            state.open_value = next.clone();
        }

        if let Some(on_change) = self.on_value_change.as_ref() {
            shell.publish((on_change)(next_value));
        }
    }
}
impl<Message> Widget<Message, iced::Theme, iced::Renderer> for NavigationMenuWidget<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        let mut children: Vec<Tree> = self.triggers.iter().map(|child| Tree::new(child)).collect();
        children.extend(self.contents.iter().map(|child| Tree::new(child)));
        children
    }

    fn diff(&self, tree: &mut Tree) {
        let mut children: Vec<&Element<'_, Message>> = self.triggers.iter().collect();
        children.extend(self.contents.iter());
        tree.diff_children(&children);
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(NavigationMenuState::default())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<NavigationMenuState>()
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
        let metrics = navigation_menu_metrics(self.list_props, &self.theme);
        let max = limits.max();
        let count = self.triggers.len();
        let mut child_nodes = Vec::with_capacity(count);

        let full_width_each = if self.list_props.full_width
            && matches!(
                self.root_props.orientation,
                NavigationMenuOrientation::Horizontal
            )
            && matches!(self.list_props.wrap, NavigationMenuWrap::NoWrap)
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
            NavigationMenuOrientation::Horizontal => {
                let mut current = Line::default();
                for (index, node) in child_nodes.iter().enumerate() {
                    let size = node.size();
                    let proposed = if current.indices.is_empty() {
                        size.width
                    } else {
                        current.width + metrics.gap + size.width
                    };

                    let should_wrap = !matches!(self.list_props.wrap, NavigationMenuWrap::NoWrap)
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
            NavigationMenuOrientation::Vertical => {
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
            NavigationMenuOrientation::Horizontal => {
                lines.iter().map(|line| line.height).sum::<f32>()
                    + metrics.line_gap * (lines.len().saturating_sub(1) as f32)
            }
            NavigationMenuOrientation::Vertical => {
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
        let wrap_reverse = matches!(self.list_props.wrap, NavigationMenuWrap::WrapReverse);
        if wrap_reverse
            && matches!(
                self.root_props.orientation,
                NavigationMenuOrientation::Horizontal
            )
        {
            let total_lines_height = content_height;
            y = size.height - metrics.list_padding - total_lines_height;
        }

        let mut trigger_bounds = Vec::with_capacity(child_nodes.len());
        trigger_bounds.resize(child_nodes.len(), Rectangle::default());

        for (line_index, line) in lines.iter().enumerate() {
            let line_space = (size.width - metrics.list_padding * 2.0 - line.width).max(0.0);
            let offset = match self.list_props.justify {
                NavigationMenuJustify::Start => 0.0,
                NavigationMenuJustify::Center => line_space / 2.0,
                NavigationMenuJustify::End => line_space,
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
                    NavigationMenuOrientation::Horizontal => metrics.line_gap,
                    NavigationMenuOrientation::Vertical => metrics.gap,
                };
                y += line.height + gap;
            }
        }

        let state = tree.state.downcast_mut::<NavigationMenuState>();
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
        let state = tree.state.downcast_mut::<NavigationMenuState>();

        if !state.initialized {
            state.initialized = true;
            if !self.is_controlled()
                && state.open_value.is_none()
                && let Some(default) = self.root_props.default_value
            {
                state.open_value = Some(default.to_string());
            }
        }

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
        let open_index = self.open_index(state);
        if open_index.is_none() {
            state.viewport_hovered = false;
            state.viewport_bounds = None;
            state.viewport_size = None;
        }

        let hovered_index = hit_test_trigger(&state.trigger_bounds, bounds, cursor);
        state.hovered_index = hovered_index;
        let resolved_viewport_bounds =
            resolve_viewport_bounds(self, state, bounds, *viewport).or(state.viewport_bounds);
        let over_viewport = resolved_viewport_bounds
            .map(|rect| cursor.is_over(rect))
            .unwrap_or(false)
            || state.viewport_hovered;
        let over_bridge = resolved_viewport_bounds
            .map(|viewport| {
                let list_bottom = bounds.y + bounds.height;
                let viewport_top = viewport.y;
                let (y1, y2) = if viewport_top >= list_bottom {
                    (list_bottom, viewport_top)
                } else {
                    (viewport_top, list_bottom)
                };
                let x1 = bounds.x.min(viewport.x);
                let x2 = (bounds.x + bounds.width).max(viewport.x + viewport.width);
                let bridge = Rectangle {
                    x: x1,
                    y: y1,
                    width: (x2 - x1).max(0.0),
                    height: (y2 - y1).max(0.0),
                };
                cursor.is_over(bridge)
            })
            .unwrap_or(false);

        if over_viewport || over_bridge {
            state.pending_close = None;
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                handle_hover(
                    self,
                    state,
                    hovered_index,
                    bounds,
                    resolved_viewport_bounds,
                    cursor,
                    shell,
                );
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) {
                    state.focused = true;
                    state.focus_visible = false;
                    state.focused_index = hovered_index.or(open_index);
                } else if state.focused {
                    state.focused = false;
                }

                if let Some(index) = hovered_index {
                    let item = self.items.get(index);
                    if let Some(item) = item {
                        match item.kind {
                            NavItemKind::Trigger if !item.disabled => {
                                let current = self.current_value(state).unwrap_or("");
                                if current == item.value {
                                    state.pending_open = None;
                                    self.set_open_value(state, shell, None);
                                    state.last_close_at = Some(Instant::now());
                                } else {
                                    state.pending_open = None;
                                    self.set_open_value(state, shell, Some(item.value.clone()));
                                }
                                shell.capture_event();
                            }
                            NavItemKind::Link => {
                                state.pending_open = None;
                                self.set_open_value(state, shell, None);
                                state.last_close_at = Some(Instant::now());
                            }
                            NavItemKind::Trigger => {}
                        }
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if !state.focused {
                    return;
                }

                state.focus_visible = true;

                let current = state.focused_index.or(open_index);
                let next = match key {
                    Key::Named(key::Named::ArrowRight)
                        if matches!(
                            self.root_props.orientation,
                            NavigationMenuOrientation::Horizontal
                        ) =>
                    {
                        current.and_then(|idx| next_enabled_index(&self.items, idx, 1))
                    }
                    Key::Named(key::Named::ArrowLeft)
                        if matches!(
                            self.root_props.orientation,
                            NavigationMenuOrientation::Horizontal
                        ) =>
                    {
                        current.and_then(|idx| next_enabled_index(&self.items, idx, -1))
                    }
                    Key::Named(key::Named::ArrowDown)
                        if matches!(
                            self.root_props.orientation,
                            NavigationMenuOrientation::Vertical
                        ) =>
                    {
                        current.and_then(|idx| next_enabled_index(&self.items, idx, 1))
                    }
                    Key::Named(key::Named::ArrowUp)
                        if matches!(
                            self.root_props.orientation,
                            NavigationMenuOrientation::Vertical
                        ) =>
                    {
                        current.and_then(|idx| next_enabled_index(&self.items, idx, -1))
                    }
                    Key::Named(key::Named::Home) => first_enabled_index(&self.items),
                    Key::Named(key::Named::End) => last_enabled_index(&self.items),
                    Key::Named(key::Named::Escape) => {
                        self.set_open_value(state, shell, None);
                        state.last_close_at = Some(Instant::now());
                        shell.capture_event();
                        None
                    }
                    Key::Named(key::Named::Enter) | Key::Named(key::Named::Space) => current,
                    _ => None,
                };

                if let Some(next_index) = next {
                    state.focused_index = Some(next_index);

                    if matches!(
                        key,
                        Key::Named(key::Named::Enter) | Key::Named(key::Named::Space)
                    ) {
                        if let Some(item) = self.items.get(next_index) {
                            match item.kind {
                                NavItemKind::Trigger if !item.disabled => {
                                    let current_value = self.current_value(state).unwrap_or("");
                                    if current_value == item.value {
                                        self.set_open_value(state, shell, None);
                                        state.last_close_at = Some(Instant::now());
                                    } else {
                                        self.set_open_value(state, shell, Some(item.value.clone()));
                                    }
                                }
                                NavItemKind::Link => {
                                    if let Some(message) = item.link_message.clone() {
                                        shell.publish(message);
                                    }
                                }
                                NavItemKind::Trigger => {}
                            }
                        }
                        shell.capture_event();
                    }
                }
            }
            Event::Window(iced::window::Event::RedrawRequested(now)) => {
                state.last_redraw = Some(*now);

                if let Some(pending) = state.pending_open {
                    let elapsed = now.saturating_duration_since(pending.started_at);
                    let delay_ms = derived_delay_ms(*now, state, self.root_props);
                    if elapsed >= Duration::from_millis(delay_ms) {
                        if let Some(item) = self.items.get(pending.index)
                            && item.kind == NavItemKind::Trigger
                            && !item.disabled
                        {
                            self.set_open_value(state, shell, Some(item.value.clone()));
                        }
                        state.pending_open = None;
                    } else {
                        shell.request_redraw();
                    }
                }

                if let Some(pending_close) = state.pending_close {
                    let elapsed = now.saturating_duration_since(pending_close);
                    let delay_ms = close_delay_ms(*now, state, self.root_props);
                    if elapsed >= Duration::from_millis(delay_ms) {
                        self.set_open_value(state, shell, None);
                        state.pending_close = None;
                        state.last_close_at = Some(*now);
                    } else {
                        shell.request_redraw();
                    }
                }

                let over_viewport = state
                    .viewport_bounds
                    .map(|rect| cursor.is_over(rect))
                    .unwrap_or(false)
                    || state.viewport_hovered;
                if (state.hovered_index.is_some() || over_viewport) && state.pending_close.is_some()
                {
                    state.pending_close = None;
                }

                if let Some(started) = state.indicator_started {
                    let elapsed = now.saturating_duration_since(started);
                    if elapsed < Duration::from_millis(INDICATOR_ANIM_MS) {
                        shell.request_redraw();
                    } else {
                        state.indicator_started = None;
                        state.indicator_from = None;
                    }
                }

                if let Some(motion) = state.motion {
                    let elapsed = now.saturating_duration_since(motion.started_at);
                    if elapsed < Duration::from_millis(MOTION_ANIM_MS) {
                        shell.request_redraw();
                    } else {
                        state.motion = None;
                    }
                }
            }
            _ => {}
        }

        let next_open_index = self.open_index(state);
        if next_open_index != state.open_index {
            let now = state.last_redraw.unwrap_or_else(Instant::now);
            state.indicator_from = state
                .open_index
                .and_then(|idx| state.trigger_bounds.get(idx).copied());
            state.indicator_to =
                next_open_index.and_then(|idx| state.trigger_bounds.get(idx).copied());
            state.indicator_started =
                if state.indicator_from.is_some() && state.indicator_to.is_some() {
                    Some(now)
                } else {
                    None
                };

            if let (Some(prev), Some(next)) = (state.open_index, next_open_index)
                && prev != next
            {
                let direction = if next > prev { 1 } else { -1 };
                state.motion = Some(Motion {
                    direction,
                    started_at: now,
                });
            }

            state.open_index = next_open_index;
            state.focused_index = state.focused_index.or(next_open_index);
            if state.indicator_started.is_some() {
                shell.request_redraw();
            }
        }

        for (index, child_tree) in tree
            .children
            .iter_mut()
            .take(self.triggers.len())
            .enumerate()
        {
            if let Some(item) = self.items.get(index)
                && item.kind == NavItemKind::Trigger
            {
                let trigger_state = child_tree
                    .state
                    .downcast_mut::<NavigationMenuTriggerState>();
                trigger_state.is_open = state.open_index == Some(index);
            }
        }
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<iced::overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        let state = tree.state.downcast_mut::<NavigationMenuState>();
        let open_index = state.open_index?;
        let item = self.items.get(open_index)?;
        let content_index = item.content_index?;
        let content_tree_index = self.triggers.len() + content_index;
        let content = &mut self.contents[content_index];
        let content_tree = &mut tree.children[content_tree_index];
        let anchor_position = layout.position() + translation;
        let trigger_bounds = state
            .trigger_bounds
            .get(open_index)
            .copied()
            .map(|rect| Rectangle {
                x: rect.x + anchor_position.x,
                y: rect.y + anchor_position.y,
                width: rect.width,
                height: rect.height,
            })
            .unwrap_or(Rectangle::default());

        Some(iced::overlay::Element::new(Box::new(
            NavigationMenuOverlay {
                content,
                tree: content_tree,
                theme: self.theme.clone(),
                root_props: self.root_props,
                content_props: item.content_props,
                state,
                trigger_bounds,
                viewport: *viewport,
            },
        )))
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

        let metrics = navigation_menu_metrics(self.list_props, &self.theme);
        let state = tree.state.downcast_ref::<NavigationMenuState>();

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

        if self.root_props.indicator
            && state.open_index.is_some()
            && let Some(rect) = indicator_rect(state, metrics, Vector::new(bounds.x, bounds.y))
        {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: rect,
                    border: Border {
                        radius: (metrics.radius * 0.4).into(),
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(self.theme.palette.border),
            );
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
struct NavigationMenuOverlay<'a, 'b, Message> {
    content: &'a mut Element<'b, Message>,
    tree: &'a mut Tree,
    theme: Theme,
    root_props: NavigationMenuProps,
    content_props: NavigationMenuContentProps,
    state: &'a mut NavigationMenuState,
    trigger_bounds: Rectangle,
    viewport: Rectangle,
}

impl<Message> iced::advanced::Overlay<Message, iced::Theme, iced::Renderer>
    for NavigationMenuOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);
        let content_node = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let content_size = content_node.size();

        let placement = if self.root_props.viewport {
            place_viewport(
                self.trigger_bounds,
                content_size,
                self.content_props.align,
                self.content_props.align_offset,
                self.content_props.side_offset,
                self.content_props.collision_padding,
                bounds,
            )
        } else {
            place_content(
                self.trigger_bounds,
                content_size,
                self.content_props,
                bounds,
            )
        };

        let motion_offset = motion_offset(self.state, self.root_props);
        let position = Point::new(
            placement.position.x + motion_offset.x,
            placement.position.y + motion_offset.y,
        );

        // Wrap the content node to ensure child layout is always present for draw/update paths.
        let mut root = layout::Node::with_children(content_node.size(), vec![content_node]);
        root = root.move_to(position);
        self.state.viewport_bounds = Some(root.bounds());
        self.state.viewport_size = Some(content_size);
        root
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let content_layout = layout.children().next().unwrap_or(layout);
        let bounds = content_layout.bounds();
        self.state.viewport_bounds = Some(bounds);
        self.state.viewport_hovered = cursor.is_over(bounds);
        if self.state.viewport_hovered {
            self.state.pending_close = None;
        }
        self.content.as_widget_mut().update(
            self.tree,
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let content_layout = layout.children().next().unwrap_or(layout);
        self.content.as_widget().mouse_interaction(
            self.tree,
            content_layout,
            cursor,
            &self.viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let content_layout = layout.children().next().unwrap_or(layout);
        let bounds = content_layout.bounds();
        let palette = self.theme.palette;
        let radius = self.theme.radius.md;

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: palette.border,
                    width: self.theme.styles.navigation_menu.border_width,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: Color {
                        a: self.theme.styles.navigation_menu.shadow.opacity,
                        ..palette.foreground
                    },
                    offset: Vector::new(0.0, self.theme.styles.navigation_menu.shadow.offset_y),
                    blur_radius: self.theme.styles.navigation_menu.shadow.blur_radius,
                },
                ..renderer::Quad::default()
            },
            Background::Color(palette.popover),
        );

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &self.viewport,
        );
    }
}

impl<'a, Message: Clone + 'a> From<NavigationMenuWidget<'a, Message>> for Element<'a, Message> {
    fn from(widget: NavigationMenuWidget<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}

#[derive(Clone, Copy, Debug)]
struct OverlayPlacement {
    position: Point,
}

fn place_viewport(
    trigger_bounds: Rectangle,
    content_size: Size,
    align: NavigationMenuAlign,
    align_offset: f32,
    side_offset: f32,
    collision_padding: f32,
    window: Size,
) -> OverlayPlacement {
    let base_x = match align {
        NavigationMenuAlign::Start => trigger_bounds.x,
        NavigationMenuAlign::Center => {
            trigger_bounds.x + (trigger_bounds.width - content_size.width) / 2.0
        }
        NavigationMenuAlign::End => trigger_bounds.x + trigger_bounds.width - content_size.width,
    } + align_offset;

    let x = base_x.clamp(
        collision_padding,
        (window.width - content_size.width - collision_padding).max(collision_padding),
    );

    let y = (trigger_bounds.y + trigger_bounds.height + side_offset).clamp(
        collision_padding,
        (window.height - content_size.height - collision_padding).max(collision_padding),
    );

    OverlayPlacement {
        position: Point::new(x, y),
    }
}

fn place_content(
    trigger_bounds: Rectangle,
    content_size: Size,
    props: NavigationMenuContentProps,
    window: Size,
) -> OverlayPlacement {
    let (mut x, mut y) = match props.side {
        NavigationMenuSide::Top => (
            trigger_bounds.x,
            trigger_bounds.y - content_size.height - props.side_offset,
        ),
        NavigationMenuSide::Bottom => (
            trigger_bounds.x,
            trigger_bounds.y + trigger_bounds.height + props.side_offset,
        ),
        NavigationMenuSide::Left => (
            trigger_bounds.x - content_size.width - props.side_offset,
            trigger_bounds.y,
        ),
        NavigationMenuSide::Right => (
            trigger_bounds.x + trigger_bounds.width + props.side_offset,
            trigger_bounds.y,
        ),
    };

    match props.side {
        NavigationMenuSide::Top | NavigationMenuSide::Bottom => {
            x = match props.align {
                NavigationMenuAlign::Start => trigger_bounds.x,
                NavigationMenuAlign::Center => {
                    trigger_bounds.x + (trigger_bounds.width - content_size.width) / 2.0
                }
                NavigationMenuAlign::End => {
                    trigger_bounds.x + trigger_bounds.width - content_size.width
                }
            } + props.align_offset;
        }
        NavigationMenuSide::Left | NavigationMenuSide::Right => {
            y = match props.align {
                NavigationMenuAlign::Start => trigger_bounds.y,
                NavigationMenuAlign::Center => {
                    trigger_bounds.y + (trigger_bounds.height - content_size.height) / 2.0
                }
                NavigationMenuAlign::End => {
                    trigger_bounds.y + trigger_bounds.height - content_size.height
                }
            } + props.align_offset;
        }
    }

    x = x.clamp(
        props.collision_padding,
        (window.width - content_size.width - props.collision_padding).max(props.collision_padding),
    );
    y = y.clamp(
        props.collision_padding,
        (window.height - content_size.height - props.collision_padding)
            .max(props.collision_padding),
    );

    OverlayPlacement {
        position: Point::new(x, y),
    }
}

fn motion_offset(state: &NavigationMenuState, props: NavigationMenuProps) -> Vector {
    let Some(motion) = state.motion else {
        return Vector::default();
    };
    let now = state.last_redraw.unwrap_or_else(Instant::now);
    let elapsed = now.saturating_duration_since(motion.started_at);
    let t = (elapsed.as_secs_f32() / (MOTION_ANIM_MS as f32 / 1000.0)).clamp(0.0, 1.0);
    let distance = if props.viewport { 48.0 } else { 32.0 };
    let offset = (1.0 - t) * distance * motion.direction as f32;
    Vector::new(offset, 0.0)
}

fn hit_test_trigger(
    bounds: &[Rectangle],
    list_bounds: Rectangle,
    cursor: mouse::Cursor,
) -> Option<usize> {
    bounds.iter().position(|rect| {
        let global = Rectangle {
            x: rect.x + list_bounds.x,
            y: rect.y + list_bounds.y,
            width: rect.width,
            height: rect.height,
        };
        cursor.is_over(global)
    })
}

fn should_skip_delay(
    now: Instant,
    state: &NavigationMenuState,
    props: NavigationMenuProps,
) -> bool {
    if props.skip_delay_duration_ms == 0 {
        return false;
    }
    state
        .last_close_at
        .map(|last| {
            now.saturating_duration_since(last)
                <= Duration::from_millis(props.skip_delay_duration_ms)
        })
        .unwrap_or(false)
}

fn derived_delay_ms(now: Instant, state: &NavigationMenuState, props: NavigationMenuProps) -> u64 {
    if state.open_index.is_some() || should_skip_delay(now, state, props) {
        100
    } else {
        props.delay_duration_ms
    }
}

fn close_delay_ms(now: Instant, state: &NavigationMenuState, props: NavigationMenuProps) -> u64 {
    if props.close_delay_ms > 0 {
        props.close_delay_ms
    } else {
        derived_delay_ms(now, state, props)
    }
}

fn resolve_viewport_bounds<Message>(
    menu: &NavigationMenuWidget<'_, Message>,
    state: &NavigationMenuState,
    list_bounds: Rectangle,
    viewport: Rectangle,
) -> Option<Rectangle> {
    let open_index = menu.open_index(state)?;
    let trigger = state.trigger_bounds.get(open_index).copied()?;
    let content_size = state.viewport_size?;
    let item = menu.items.get(open_index)?;

    let trigger_bounds = Rectangle {
        x: trigger.x + list_bounds.x,
        y: trigger.y + list_bounds.y,
        width: trigger.width,
        height: trigger.height,
    };

    let viewport_size = Size::new(viewport.width, viewport.height);
    let placement = if menu.root_props.viewport {
        place_viewport(
            trigger_bounds,
            content_size,
            item.content_props.align,
            item.content_props.align_offset,
            item.content_props.side_offset,
            item.content_props.collision_padding,
            viewport_size,
        )
    } else {
        place_content(
            trigger_bounds,
            content_size,
            item.content_props,
            viewport_size,
        )
    };

    let motion = motion_offset(state, menu.root_props);
    let position = Point::new(
        placement.position.x + motion.x,
        placement.position.y + motion.y,
    );

    Some(Rectangle {
        x: position.x,
        y: position.y,
        width: content_size.width,
        height: content_size.height,
    })
}

fn handle_hover<Message: Clone>(
    menu: &NavigationMenuWidget<'_, Message>,
    state: &mut NavigationMenuState,
    hovered_index: Option<usize>,
    list_bounds: Rectangle,
    viewport_bounds: Option<Rectangle>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) {
    let now = Instant::now();
    let open_index = menu.open_index(state);
    let over_viewport = viewport_bounds
        .map(|rect| cursor.is_over(rect))
        .unwrap_or(false)
        || state.viewport_hovered;
    let over_bridge = viewport_bounds
        .map(|viewport| {
            let list_bottom = list_bounds.y + list_bounds.height;
            let viewport_top = viewport.y;
            let (y1, y2) = if viewport_top >= list_bottom {
                (list_bottom, viewport_top)
            } else {
                (viewport_top, list_bottom)
            };
            let x1 = list_bounds.x.min(viewport.x);
            let x2 = (list_bounds.x + list_bounds.width).max(viewport.x + viewport.width);
            let bridge = Rectangle {
                x: x1,
                y: y1,
                width: (x2 - x1).max(0.0),
                height: (y2 - y1).max(0.0),
            };
            cursor.is_over(bridge)
        })
        .unwrap_or(false);

    if let Some(index) = hovered_index
        && let Some(item) = menu.items.get(index)
        && item.kind == NavItemKind::Trigger
        && !item.disabled
    {
        state.pending_close = None;

        if open_index != Some(index)
            && state
                .pending_open
                .map(|pending| pending.index != index)
                .unwrap_or(true)
        {
            state.pending_open = Some(PendingOpen {
                index,
                started_at: now,
            });
            shell.request_redraw();
        }
    } else {
        state.pending_open = None;
        if open_index.is_some() && !(over_viewport || over_bridge) {
            if state.pending_close.is_none() {
                state.pending_close = Some(now);
                shell.request_redraw();
            }
        } else if open_index.is_some() && (over_viewport || over_bridge) {
            state.pending_close = None;
        }
    }
}

#[derive(Default)]
struct Line {
    indices: Vec<usize>,
    width: f32,
    height: f32,
}

fn resolve_open_index<Message>(
    items: &[NavItemMeta<Message>],
    value: Option<&str>,
) -> Option<usize> {
    let value = value?;
    items
        .iter()
        .position(|item| item.kind == NavItemKind::Trigger && !item.disabled && item.value == value)
}

fn first_enabled_index<Message>(items: &[NavItemMeta<Message>]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

fn last_enabled_index<Message>(items: &[NavItemMeta<Message>]) -> Option<usize> {
    items.iter().rposition(|item| !item.disabled)
}

fn next_enabled_index<Message>(
    items: &[NavItemMeta<Message>],
    start: usize,
    direction: i32,
) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    let mut index = start as i32;
    for _ in 0..items.len() {
        index += direction;
        if index < 0 || index >= items.len() as i32 {
            index = if direction > 0 {
                0
            } else {
                items.len() as i32 - 1
            };
        }

        let idx = index as usize;
        if !items[idx].disabled {
            return Some(idx);
        }
    }

    None
}

fn indicator_rect(
    state: &NavigationMenuState,
    metrics: NavigationMenuMetrics,
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

    let size = metrics.indicator_size;
    let x = rect.x + offset.x + (rect.width - size) / 2.0;
    let y = rect.y + offset.y + rect.height + metrics.indicator_offset;

    Some(Rectangle {
        x,
        y,
        width: size,
        height: size,
    })
}

fn lerp_rect(from: Rectangle, to: Rectangle, t: f32) -> Rectangle {
    Rectangle {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
        width: from.width + (to.width - from.width) * t,
        height: from.height + (to.height - from.height) * t,
    }
}
fn build_trigger_element<'a, Message: Clone + 'a>(
    content: NavigationMenuTriggerContent<'a, Message>,
    show_chevron: bool,
    disabled: bool,
    list_props: NavigationMenuListProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let size = list_props.size;
    let text_size = size.text_size();
    let icon_size = size.icon_size();

    let label = match content {
        NavigationMenuTriggerContent::Text(label) => text(label).size(text_size).into(),
        NavigationMenuTriggerContent::Element(element) => element,
    };

    NavigationMenuTriggerWidget {
        content: label,
        show_chevron,
        icon_size,
        size,
        disabled,
        list_props,
        theme: theme.clone(),
    }
    .into()
}

impl ButtonRadius {
    fn value(self) -> f32 {
        match self {
            ButtonRadius::None => 0.0,
            ButtonRadius::Small => 6.0,
            ButtonRadius::Medium => 8.0,
            ButtonRadius::Large => 12.0,
            ButtonRadius::Full => 999.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_open_index_matches_trigger() {
        let items = vec![
            NavItemMeta::<()> {
                value: "home".to_string(),
                kind: NavItemKind::Trigger,
                disabled: false,
                content_index: Some(0),
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
            NavItemMeta::<()> {
                value: "docs".to_string(),
                kind: NavItemKind::Link,
                disabled: false,
                content_index: None,
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
        ];

        assert_eq!(resolve_open_index(&items, Some("home")), Some(0));
        assert_eq!(resolve_open_index(&items, Some("docs")), None);
    }

    #[test]
    fn next_enabled_wraps() {
        let items = vec![
            NavItemMeta::<()> {
                value: "Size1".to_string(),
                kind: NavItemKind::Trigger,
                disabled: false,
                content_index: Some(0),
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
            NavItemMeta::<()> {
                value: "Size2".to_string(),
                kind: NavItemKind::Trigger,
                disabled: true,
                content_index: Some(1),
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
            NavItemMeta::<()> {
                value: "Size3".to_string(),
                kind: NavItemKind::Trigger,
                disabled: false,
                content_index: Some(2),
                content_props: NavigationMenuContentProps::new(),
                link_message: None,
            },
        ];

        assert_eq!(next_enabled_index(&items, 0, 1), Some(2));
        assert_eq!(next_enabled_index(&items, 2, 1), Some(0));
    }
}
