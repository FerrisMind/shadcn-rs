use iced::advanced::layout;
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Layout, Shell};
use iced::border::Border;
use iced::mouse;
use iced::time::{Duration, Instant};
use iced::widget::{container, tooltip as tooltip_widget};
use iced::window;
use iced::{Background, Element, Event, Length, Padding, Point, Rectangle, Size, Vector};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
    FollowCursor,
}

impl From<TooltipPosition> for tooltip_widget::Position {
    fn from(value: TooltipPosition) -> Self {
        match value {
            TooltipPosition::Top => tooltip_widget::Position::Top,
            TooltipPosition::Bottom => tooltip_widget::Position::Bottom,
            TooltipPosition::Left => tooltip_widget::Position::Left,
            TooltipPosition::Right => tooltip_widget::Position::Right,
            TooltipPosition::FollowCursor => tooltip_widget::Position::FollowCursor,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TooltipProps {
    pub position: TooltipPosition,
    pub gap: f32,
    pub delay_ms: u64,
    pub snap_within_viewport: bool,
    pub max_width: u32,
    pub show_shadow: bool,
    pub show_border: bool,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            position: TooltipPosition::Top,
            gap: 4.0,
            delay_ms: 0,
            snap_within_viewport: true,
            max_width: 360,
            show_shadow: true,
            show_border: true,
        }
    }
}

impl TooltipProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width.max(1);
        self
    }

    pub fn show_shadow(mut self, show: bool) -> Self {
        self.show_shadow = show;
        self
    }

    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }
}

pub fn tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tip: impl Into<Element<'a, Message>>,
    props: TooltipProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let theme = theme.clone();
    let tooltip_content: Element<'a, Message> = container(tip)
        .padding(8)
        .max_width(props.max_width)
        .style(
            move |_iced_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(Background::Color(
                    if crate::tokens::is_dark(&theme.palette) {
                        theme.palette.popover
                    } else {
                        theme.palette.secondary
                    },
                )),
                text_color: Some(if crate::tokens::is_dark(&theme.palette) {
                    theme.palette.popover_foreground
                } else {
                    theme.palette.secondary_foreground
                }),
                border: Border {
                    color: if props.show_border {
                        theme.palette.border
                    } else {
                        iced::Color::TRANSPARENT
                    },
                    width: if props.show_border { 1.0 } else { 0.0 },
                    radius: theme.radius.sm.into(),
                },
                shadow: if props.show_shadow {
                    iced::Shadow {
                        color: iced::Color {
                            a: 0.18,
                            ..iced::Color::BLACK
                        },
                        offset: iced::Vector::new(0.0, 6.0),
                        blur_radius: 18.0,
                    }
                } else {
                    iced::Shadow::default()
                },
                snap: true,
            },
        )
        .into();

    if props.position == TooltipPosition::FollowCursor {
        FollowCursorTooltip::new(
            content.into(),
            tooltip_content,
            props.gap.max(10.0),
            Duration::from_millis(props.delay_ms),
            props.snap_within_viewport,
        )
        .into()
    } else {
        tooltip_widget::Tooltip::new(content, tooltip_content, props.position.into())
            .gap(props.gap)
            .padding(0)
            .delay(Duration::from_millis(props.delay_ms))
            .snap_within_viewport(props.snap_within_viewport)
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum FollowCursorState {
    #[default]
    Idle,
    Hovered {
        at: Instant,
    },
    Open {
        cursor_position: Point,
    },
}

struct FollowCursorTooltip<'a, Message> {
    content: Element<'a, Message>,
    tooltip: Element<'a, Message>,
    gap: f32,
    padding: f32,
    snap_within_viewport: bool,
    delay: Duration,
}

impl<'a, Message> FollowCursorTooltip<'a, Message> {
    const DEFAULT_PADDING: f32 = 5.0;

    fn new(
        content: Element<'a, Message>,
        tooltip: Element<'a, Message>,
        gap: f32,
        delay: Duration,
        snap_within_viewport: bool,
    ) -> Self {
        Self {
            content,
            tooltip,
            gap,
            padding: Self::DEFAULT_PADDING,
            snap_within_viewport,
            delay,
        }
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for FollowCursorTooltip<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content), Tree::new(&self.tooltip)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget(), self.tooltip.as_widget()]);
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(FollowCursorState::default())
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<FollowCursorState>()
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
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
        if let Event::Mouse(_) | Event::Window(window::Event::RedrawRequested(_)) = event {
            let state = tree.state.downcast_mut::<FollowCursorState>();
            let now = Instant::now();
            let cursor_position = cursor.position_over(layout.bounds());

            match (*state, cursor_position) {
                (FollowCursorState::Idle, Some(cursor_position)) => {
                    if self.delay == Duration::ZERO {
                        *state = FollowCursorState::Open { cursor_position };
                        shell.invalidate_layout();
                    } else {
                        *state = FollowCursorState::Hovered { at: now };
                    }

                    shell.request_redraw_at(now + self.delay);
                }
                (FollowCursorState::Hovered { .. }, None) => {
                    *state = FollowCursorState::Idle;
                }
                (FollowCursorState::Hovered { at }, _) if at.elapsed() < self.delay => {
                    shell.request_redraw_at(now + self.delay - at.elapsed());
                }
                (FollowCursorState::Hovered { .. }, Some(cursor_position)) => {
                    *state = FollowCursorState::Open { cursor_position };
                    shell.invalidate_layout();
                }
                (
                    FollowCursorState::Open {
                        cursor_position: last_position,
                    },
                    Some(cursor_position),
                ) if last_position != cursor_position => {
                    *state = FollowCursorState::Open { cursor_position };
                    shell.request_redraw();
                }
                (FollowCursorState::Open { .. }, None) => {
                    *state = FollowCursorState::Idle;
                    shell.invalidate_layout();

                    if !matches!(event, Event::Window(window::Event::RedrawRequested(_)),) {
                        shell.request_redraw();
                    }
                }
                (FollowCursorState::Open { .. }, Some(_)) | (FollowCursorState::Idle, None) => {}
            }
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
        renderer: &iced::Renderer,
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
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            inherited_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        let state = tree.state.downcast_ref::<FollowCursorState>();
        let mut children = tree.children.iter_mut();

        let content_overlay = self.content.as_widget_mut().overlay(
            children.next().expect("content tree"),
            layout,
            renderer,
            viewport,
            translation,
        );

        let tooltip_overlay = if let FollowCursorState::Open { cursor_position } = *state {
            Some(overlay::Element::new(Box::new(FollowCursorOverlay {
                position: layout.position() + translation,
                tooltip: &mut self.tooltip,
                tree: children.next().expect("tooltip tree"),
                cursor_position,
                content_bounds: layout.bounds(),
                snap_within_viewport: self.snap_within_viewport,
                gap: self.gap,
                padding: self.padding,
            })))
        } else {
            None
        };

        if content_overlay.is_some() || tooltip_overlay.is_some() {
            Some(
                overlay::Group::with_children(
                    content_overlay.into_iter().chain(tooltip_overlay).collect(),
                )
                .overlay(),
            )
        } else {
            None
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }
}

impl<'a, Message: 'a> From<FollowCursorTooltip<'a, Message>> for Element<'a, Message> {
    fn from(tooltip: FollowCursorTooltip<'a, Message>) -> Self {
        Element::new(tooltip)
    }
}

struct FollowCursorOverlay<'a, 'b, Message> {
    position: Point,
    tooltip: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    cursor_position: Point,
    content_bounds: Rectangle,
    snap_within_viewport: bool,
    gap: f32,
    padding: f32,
}

impl<Message> overlay::Overlay<Message, iced::Theme, iced::Renderer>
    for FollowCursorOverlay<'_, '_, Message>
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let viewport = Rectangle::with_size(bounds);

        let tooltip_layout = self.tooltip.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(
                Size::ZERO,
                if self.snap_within_viewport {
                    viewport.size()
                } else {
                    Size::INFINITE
                },
            )
            .shrink(Padding::new(self.padding)),
        );

        let text_bounds = tooltip_layout.bounds();
        let translation = self.position - self.content_bounds.position();

        let mut tooltip_bounds = Rectangle {
            x: self.cursor_position.x - text_bounds.width / 2.0 + translation.x - self.padding,
            y: self.cursor_position.y + self.gap - text_bounds.height / 2.0 + translation.y
                - self.padding,
            width: text_bounds.width + self.padding * 2.0,
            height: text_bounds.height + self.padding * 2.0,
        };

        if self.snap_within_viewport {
            if tooltip_bounds.x < viewport.x {
                tooltip_bounds.x = viewport.x;
            } else if viewport.x + viewport.width < tooltip_bounds.x + tooltip_bounds.width {
                tooltip_bounds.x = viewport.x + viewport.width - tooltip_bounds.width;
            }

            if tooltip_bounds.y < viewport.y {
                tooltip_bounds.y = viewport.y;
            } else if viewport.y + viewport.height < tooltip_bounds.y + tooltip_bounds.height {
                tooltip_bounds.y = viewport.y + viewport.height - tooltip_bounds.height;
            }
        }

        layout::Node::with_children(
            tooltip_bounds.size(),
            vec![tooltip_layout.translate(Vector::new(self.padding, self.padding))],
        )
        .translate(Vector::new(tooltip_bounds.x, tooltip_bounds.y))
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.tooltip.as_widget().draw(
            self.tree,
            renderer,
            theme,
            inherited_style,
            layout.children().next().expect("tooltip child layout"),
            cursor,
            &Rectangle::with_size(Size::INFINITE),
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.tooltip.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().expect("tooltip child layout"),
            cursor,
            &Rectangle::with_size(Size::INFINITE),
            renderer,
        )
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
        self.tooltip.as_widget_mut().update(
            self.tree,
            event,
            layout.children().next().expect("tooltip child layout"),
            cursor,
            renderer,
            clipboard,
            shell,
            &Rectangle::with_size(Size::INFINITE),
        );
    }
}
