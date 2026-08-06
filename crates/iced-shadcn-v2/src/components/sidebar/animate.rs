//! Desktop width animation for [`super::Sidebar`] (`duration-200 ease-linear`).
//!
//! Mirrors the web `transition-[width]` / `cn-sidebar-gap` behaviour by driving
//! the in-flow layout width with [`shadcn_common::lerp_sidebar_gap`] and
//! clipping the child to that band — the iced counterpart of the absolute
//! container shrinking over a reserved gap.

use std::slice;
use std::time::Duration;

use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Operation, Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::{Element, Event, Length, Rectangle, Size, Vector, mouse, time, window};

use shadcn_common::{
    Easing, SIDEBAR_ANIMATION_MS, SidebarCollapsible, SidebarVariant, TransitionValue,
    lerp_sidebar_gap,
};

/// Frame pacing while the gap width interpolates.
const FRAME_INTERVAL: Duration = Duration::from_millis(16);

/// Open/close timing matching `duration-200 ease-linear` on the web.
#[derive(Debug, Clone, Copy)]
struct Animation {
    animated: bool,
    duration: Duration,
}

impl Animation {
    const fn desktop(animated: bool) -> Self {
        Self {
            animated,
            duration: Duration::from_millis(SIDEBAR_ANIMATION_MS),
        }
    }
}

/// Eased open progress stored in the widget tree (`0.0` = closed, `1.0` = open).
#[derive(Debug, Default)]
struct GapTransition {
    value: TransitionValue,
}

impl GapTransition {
    fn advance(&mut self, open: bool, animation: Animation, now: time::Instant) {
        let target = f32::from(u8::from(open));
        self.value.advance(
            target,
            animation.animated,
            animation.duration,
            Easing::Linear,
            now,
        );
    }

    /// Progress after [`Self::advance`] — uses the live current value so a
    /// controlled `open` flip does not snap via [`TransitionValue::displayed`].
    fn progress(&self) -> f32 {
        self.value.current().clamp(0.0, 1.0)
    }

    const fn is_running(&self) -> bool {
        self.value.is_running()
    }
}

/// Layout widget that animates the sidebar's reserved gap width.
pub(super) struct AnimatedGap<'a, Message> {
    content: Element<'a, Message>,
    open: bool,
    collapsible: SidebarCollapsible,
    variant: SidebarVariant,
    width_px: f32,
    width_icon_px: f32,
    animated: bool,
}

impl<'a, Message> AnimatedGap<'a, Message> {
    pub(super) fn new(
        content: impl Into<Element<'a, Message>>,
        open: bool,
        collapsible: SidebarCollapsible,
        variant: SidebarVariant,
        width_px: f32,
        width_icon_px: f32,
        animated: bool,
    ) -> Self {
        Self {
            content: content.into(),
            open,
            collapsible,
            variant,
            width_px,
            width_icon_px,
            animated,
        }
    }

    fn animation(&self) -> Animation {
        Animation::desktop(self.animated)
    }

    fn gap_for(&self, progress: f32) -> f32 {
        lerp_sidebar_gap(
            progress,
            self.collapsible,
            self.variant,
            self.width_px,
            self.width_icon_px,
        )
        .max(0.0)
    }

    fn child_layout<'b>(layout: Layout<'b>) -> Layout<'b> {
        layout
            .children()
            .next()
            .expect("animated sidebar child layout")
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for AnimatedGap<'_, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<GapTransition>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(GapTransition::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Fill,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Advance here so a controlled `open` change starts interpolating on
        // the same frame as the view rebuild (avoids snapping via `displayed`).
        let state = tree.state.downcast_mut::<GapTransition>();
        state.advance(self.open, self.animation(), time::Instant::now());
        let gap = self.gap_for(state.progress());

        let child_limits = limits.width(Length::Fixed(gap)).height(Length::Fill);
        let child =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &child_limits);

        let height = limits
            .resolve(Length::Shrink, Length::Fill, child.size())
            .height;
        layout::Node::with_children(Size::new(gap, height), vec![child])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        if layout.bounds().width <= 0.0 {
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
            let state = tree.state.downcast_mut::<GapTransition>();
            let was_running = state.is_running();
            state.advance(self.open, self.animation(), *now);

            if state.is_running() {
                shell.request_redraw_at(*now + FRAME_INTERVAL);
                shell.invalidate_layout();
            } else if was_running {
                shell.invalidate_layout();
            }
        } else {
            // Kick the first animation frame after a toggle without waiting for
            // a redraw event (layout may have already started the transition).
            let state = tree.state.downcast_ref::<GapTransition>();
            if state.is_running() {
                shell.request_redraw();
                shell.invalidate_layout();
            }
        }

        if layout.bounds().width <= 0.0 {
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
        if layout.bounds().width <= 0.0 {
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
        let bounds = layout.bounds();
        if bounds.width <= 0.0 {
            return;
        }

        let Some(clipped) = bounds.intersection(viewport) else {
            return;
        };

        // Overflow-hidden equivalent while the gap shrinks (offcanvas → 0,
        // icon → rail width).
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
        if layout.bounds().width <= 0.0 {
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

impl<'a, Message> From<AnimatedGap<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(value: AnimatedGap<'a, Message>) -> Self {
        Element::new(value)
    }
}

/// Watches window resize and emits the viewport width (md = 768 breakpoint).
pub(super) struct ViewportProbe<'a, Message> {
    content: Element<'a, Message>,
    on_viewport: Box<dyn Fn(f32) -> Message + 'a>,
}

impl<'a, Message> ViewportProbe<'a, Message> {
    pub(super) fn new(
        content: impl Into<Element<'a, Message>>,
        on_viewport: Box<dyn Fn(f32) -> Message + 'a>,
    ) -> Self {
        Self {
            content: content.into(),
            on_viewport,
        }
    }
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for ViewportProbe<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        if let Event::Window(window::Event::Resized(size)) = event {
            shell.publish((self.on_viewport)(size.width));
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
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
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
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
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
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
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message> From<ViewportProbe<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(value: ViewportProbe<'a, Message>) -> Self {
        Element::new(value)
    }
}
