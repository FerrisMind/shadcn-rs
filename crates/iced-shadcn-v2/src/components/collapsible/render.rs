//! Slot composition, reveal animation, and indicator painting for the collapsible.

use std::slice;
use std::time::Duration;

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::widget::canvas::{self, LineCap, LineJoin, Path, Stroke};
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{Space, column, container, row, text as iced_text};
use crate::iced_compat::{
    Color, Element, Event, Length, Point, Rectangle, Size, Vector, alignment, mouse, time, window,
};
use shadcn_common::{Easing, TransitionValue};

use crate::components::button::{Button, ButtonSize};
use crate::fonts::iced_font;
use crate::theme::Theme;

use super::types::{
    CollapsibleAlignment, CollapsibleEasing, CollapsibleIndicator, CollapsibleIndicatorPlacement,
    CollapsibleOrientation,
};
use super::{CollapsibleContent, CollapsibleTrigger, TriggerContent, geometry, style};

/// Frame pacing while the panel reveals, matching the other animated components.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Reveal timing shared by the content panel and the trigger chevron.
#[derive(Debug, Clone, Copy)]
pub(super) struct Animation {
    pub(super) animated: bool,
    pub(super) duration: Duration,
    pub(super) easing: CollapsibleEasing,
}

/// Eased progress of one open/close transition, stored in the widget tree.
#[derive(Debug, Default)]
pub(super) struct Transition {
    value: TransitionValue,
}

impl Transition {
    /// Advances the transition toward `open` for the frame drawn at `now`.
    pub(super) fn advance(&mut self, open: bool, animation: Animation, now: time::Instant) {
        let target = f32::from(u8::from(open));
        self.value.advance(
            target,
            animation.animated,
            animation.duration,
            common_easing(animation.easing),
            now,
        );
    }

    /// Reveal fraction in `0.0..=1.0`.
    ///
    /// While a transition runs the eased value drives it. At rest the value is
    /// derived from `open` directly, so a panel whose state changed while no
    /// frames were requested still paints correctly.
    pub(super) fn progress(&self, open: bool) -> f32 {
        self.value
            .displayed(f32::from(u8::from(open)))
            .clamp(0.0, 1.0)
    }

    pub(super) const fn is_running(&self) -> bool {
        self.value.is_running()
    }
}

fn common_easing(easing: CollapsibleEasing) -> Easing {
    match easing {
        CollapsibleEasing::Linear => Easing::Linear,
        CollapsibleEasing::EaseOut => Easing::EaseOut,
        CollapsibleEasing::EaseInOut => Easing::EaseInOut,
    }
}

/// Size of the revealed band for a child of `natural` size.
pub(super) fn revealed_size(
    natural: Size,
    orientation: CollapsibleOrientation,
    progress: f32,
) -> Size {
    let progress = if progress.is_finite() {
        progress.clamp(0.0, 1.0)
    } else {
        0.0
    };

    match orientation {
        CollapsibleOrientation::Vertical => {
            Size::new(natural.width, (natural.height * progress).max(0.0))
        }
        CollapsibleOrientation::Horizontal => {
            Size::new((natural.width * progress).max(0.0), natural.height)
        }
    }
}

/// Cross-axis alignment of the root slots.
pub(super) fn align_x(align: CollapsibleAlignment) -> alignment::Horizontal {
    match align {
        CollapsibleAlignment::Start => alignment::Horizontal::Left,
        CollapsibleAlignment::Center => alignment::Horizontal::Center,
        CollapsibleAlignment::End => alignment::Horizontal::Right,
    }
}

/// Cross-axis alignment of the root slots on the horizontal axis.
pub(super) fn align_y(align: CollapsibleAlignment) -> alignment::Vertical {
    match align {
        CollapsibleAlignment::Start => alignment::Vertical::Top,
        CollapsibleAlignment::Center => alignment::Vertical::Center,
        CollapsibleAlignment::End => alignment::Vertical::Bottom,
    }
}

/// Builds the trigger button, wiring it to the root's toggle message.
pub(super) fn build_trigger<'a, Message>(
    trigger: CollapsibleTrigger<'a, Message>,
    open: bool,
    root_disabled: bool,
    toggle: Option<Message>,
    animation: Animation,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let CollapsibleTrigger {
        theme,
        content,
        variant,
        size,
        radius,
        color,
        width,
        full_width,
        disabled,
        indicator,
        indicator_placement,
        gap,
        height,
        padding,
        on_press,
        style_override,
    } = trigger;

    let disabled = disabled || root_disabled;
    let glyph = indicator.map(|kind| {
        indicator_canvas(Indicator {
            kind,
            open,
            color: style::trigger_text_color(theme, variant, color, disabled),
            size: geometry::indicator_size_px(theme, size),
            animation,
        })
    });

    let mut button = match (glyph, content) {
        (None, TriggerContent::Label(label)) => Button::text(label, theme),
        (None, TriggerContent::Element(element)) => Button::new(element, theme),
        (None, TriggerContent::Icon(element)) => Button::icon(element, theme),
        (None, TriggerContent::Indicator) => Button::icon(Space::new(), theme),
        (Some(glyph), TriggerContent::Indicator) => Button::icon(glyph, theme),
        (Some(glyph), content) => {
            let label = match content {
                TriggerContent::Label(label) => trigger_label(label, theme, size),
                TriggerContent::Element(element) | TriggerContent::Icon(element) => element,
                TriggerContent::Indicator => unreachable!("handled by the previous arm"),
            };
            let gap = gap.unwrap_or_else(|| geometry::trigger_gap_px(theme, size));
            let slots = match indicator_placement {
                CollapsibleIndicatorPlacement::Leading => vec![glyph, label],
                CollapsibleIndicatorPlacement::Trailing => vec![label, glyph],
            };

            Button::new(
                row(slots).spacing(gap).align_y(alignment::Vertical::Center),
                theme,
            )
        }
    };

    button = button
        .variant(variant)
        .size(size)
        .width(width)
        .disabled(disabled);

    if full_width {
        button = button.full_width();
    }

    if let Some(height) = height {
        button = button.height(height);
    }

    if let Some(padding) = padding {
        button = button.padding_resolved(padding);
    }

    if let Some(radius) = radius {
        button = button.radius(radius);
    }

    if let Some(color) = color {
        button = button.color(color);
    }

    if let Some(message) = on_press.or(toggle) {
        button = button.on_press(message);
    }

    if let Some(style_override) = style_override {
        button = button.style_override(style_override);
    }

    button.into()
}

/// Trigger label carrying the pack's `.cn-button` typography.
fn trigger_label<'a, Message: 'a>(
    label: Fragment<'a>,
    theme: &Theme,
    size: ButtonSize,
) -> Element<'a, Message> {
    let size_px = geometry::trigger_text_size_px(theme, size);
    let recipe = theme.style.button_type();
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = crate::recipes::iced_font_weight(recipe.typography.weight);

    let label = if recipe.typography.uppercase {
        label.as_ref().to_uppercase()
    } else {
        label.into_owned()
    };

    iced_text(label)
        .size(size_px)
        .font(font)
        .line_height(LineHeight::Absolute(size_px.into()))
        .into()
}

/// Builds the animated content panel.
pub(super) fn build_content<'a, Message: 'a>(
    content: CollapsibleContent<'a, Message>,
    open: bool,
    orientation: CollapsibleOrientation,
    animation: Animation,
) -> Element<'a, Message> {
    let CollapsibleContent {
        theme,
        children,
        spacing,
        padding,
        width,
        height,
        surface,
        force_mount,
        style_override,
    } = content;

    let spacing = spacing.unwrap_or(geometry::DEFAULT_SPACING);
    let body: Element<'a, Message> = match orientation {
        CollapsibleOrientation::Vertical => {
            column(children).spacing(spacing).width(Length::Fill).into()
        }
        CollapsibleOrientation::Horizontal => row(children)
            .spacing(spacing)
            .align_y(alignment::Vertical::Center)
            .into(),
    };

    let mut resolved = style::resolve_surface(theme, surface);
    if let Some(style_override) = style_override.as_ref() {
        resolved = style_override(resolved);
    }

    let panel = container(body)
        .width(width)
        .height(height)
        .padding(padding.unwrap_or_default())
        .style(move |_iced_theme| resolved);

    Element::new(Reveal {
        content: panel.into(),
        open,
        orientation,
        animation,
        force_mount,
    })
}

/// Layout widget that animates one axis of its child between zero and natural size.
struct Reveal<'a, Message> {
    content: Element<'a, Message>,
    open: bool,
    orientation: CollapsibleOrientation,
    animation: Animation,
    force_mount: bool,
}

impl<Message> Reveal<'_, Message> {
    fn child_layout<'b>(layout: Layout<'b>) -> Layout<'b> {
        layout
            .children()
            .next()
            .expect("collapsible content child layout")
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for Reveal<'_, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Transition>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Transition::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        let content = self.content.as_widget().size();

        match self.orientation {
            CollapsibleOrientation::Vertical => Size {
                width: content.width,
                height: Length::Shrink,
            },
            CollapsibleOrientation::Horizontal => Size {
                width: Length::Shrink,
                height: content.height,
            },
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let progress = tree.state.downcast_ref::<Transition>().progress(self.open);
        let child_limits = match self.orientation {
            CollapsibleOrientation::Vertical => limits.loose().height(Length::Shrink),
            CollapsibleOrientation::Horizontal => limits.loose().width(Length::Shrink),
        };

        let child =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &child_limits);
        let natural = child.size();
        let bounds = revealed_size(natural, self.orientation, progress);

        layout::Node::with_children(bounds, vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        if !self.is_mounted(tree) {
            return;
        }

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            Self::child_layout(layout),
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<Transition>();
            let was_running = state.is_running();

            state.advance(self.open, self.animation, *now);

            // The revealed size is a layout property, so every animation frame
            // has to invalidate the layout — including the one that ends the
            // transition, which snaps to the final size.
            if state.is_running() {
                shell.request_redraw_at(*now + FRAME_INTERVAL);
                shell.invalidate_layout();
            } else if was_running {
                shell.invalidate_layout();
            }
        }

        if !self.is_mounted(tree) {
            return;
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            Self::child_layout(layout),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        if !self.is_visible(tree) {
            return mouse::Interaction::None;
        }

        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            Self::child_layout(layout),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if !self.is_visible(tree) {
            return;
        }

        let Some(clipped) = layout.bounds().intersection(viewport) else {
            return;
        };

        // `overflow-hidden` of the web content wrapper: the child keeps its
        // natural size and is cropped to the revealed band.
        renderer.with_layer(clipped, |renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                Self::child_layout(layout),
                cursor,
                &clipped,
            );
        });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &crate::iced_compat::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<
        overlay::Element<'b, Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>,
    > {
        if !self.is_mounted(tree) {
            return None;
        }

        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            Self::child_layout(layout),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<Message> Reveal<'_, Message> {
    /// Whether any part of the panel is painted.
    fn is_visible(&self, tree: &Tree) -> bool {
        tree.state.downcast_ref::<Transition>().progress(self.open) > 0.0
    }

    /// Whether the child still takes part in events, focus, and overlays.
    fn is_mounted(&self, tree: &Tree) -> bool {
        self.force_mount || self.is_visible(tree)
    }
}

/// Chevron glyph rotated by the reveal progress.
#[derive(Debug, Clone, Copy)]
struct Indicator {
    kind: CollapsibleIndicator,
    open: bool,
    color: Color,
    size: f32,
    animation: Animation,
}

fn indicator_canvas<'a, Message: 'a>(indicator: Indicator) -> Element<'a, Message> {
    let size = Length::Fixed(indicator.size);

    canvas::Canvas::new(indicator)
        .width(size)
        .height(size)
        .into()
}

impl<Message> canvas::Program<Message> for Indicator {
    type State = Transition;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let canvas::Event::Window(window::Event::RedrawRequested(now)) = event else {
            return None;
        };

        state.advance(self.open, self.animation, *now);

        state
            .is_running()
            .then(|| canvas::Action::request_redraw_at(*now + FRAME_INTERVAL))
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        if !size.width.is_finite()
            || !size.height.is_finite()
            || size.width <= 0.0
            || size.height <= 0.0
        {
            return Vec::new();
        }

        let mut frame = canvas::Frame::new(renderer, size);
        let center = frame.center();
        let extent = size.width.min(size.height);
        let arm = extent * 0.30;
        let reach = extent * 0.18;
        let width = (extent * 0.125).clamp(1.0, 2.5);
        let angle = self.kind.open_angle() * state.progress(self.open);
        let chevron = Path::new(|builder| {
            builder.move_to(Point::new(-reach, -arm));
            builder.line_to(Point::new(reach, 0.0));
            builder.line_to(Point::new(-reach, arm));
        });

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            frame.rotate(angle);
            frame.stroke(
                &chevron,
                Stroke::default()
                    .with_width(width)
                    .with_color(self.color)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
            );
        });

        vec![frame.into_geometry()]
    }
}
