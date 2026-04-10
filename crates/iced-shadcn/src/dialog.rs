use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::keyboard;
use iced::mouse;
use iced::touch;
use iced::{Background, Color, Element, Event, Length, Point, Rectangle, Shadow, Size};

use crate::overlay::keyboard as overlay_keyboard;
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAlign {
    Start,
    Center,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogSize {
    Size1,
    Size2,
    Size3,
    Size4,
}

#[derive(Clone, Copy, Debug)]
pub struct DialogProps {
    pub align: DialogAlign,
    pub size: DialogSize,
    pub max_width: u32,
    pub overlay_opacity: f32,
    pub close_on_blur: bool,
    pub padding: Option<u16>,
    pub draggable: bool,
    pub viewport_margin: Option<(f32, f32)>,
}

impl Default for DialogProps {
    fn default() -> Self {
        Self {
            align: DialogAlign::Center,
            size: DialogSize::Size3,
            max_width: 600,
            overlay_opacity: 0.8,
            close_on_blur: true,
            padding: None,
            draggable: false,
            viewport_margin: None,
        }
    }
}

impl DialogProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn align(mut self, align: DialogAlign) -> Self {
        self.align = align;
        self
    }

    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width.max(1);
        self
    }

    pub fn overlay_opacity(mut self, overlay_opacity: f32) -> Self {
        self.overlay_opacity = overlay_opacity.clamp(0.0, 1.0);
        self
    }

    pub fn close_on_blur(mut self, close_on_blur: bool) -> Self {
        self.close_on_blur = close_on_blur;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    pub fn viewport_margin(mut self, margin_w_pct: f32, margin_h_pct: f32) -> Self {
        self.viewport_margin = Some((margin_w_pct.clamp(0.0, 0.5), margin_h_pct.clamp(0.0, 0.5)));
        self
    }
}

fn dialog_padding(theme: &Theme, size: DialogSize, override_padding: Option<u16>) -> u16 {
    if let Some(padding) = override_padding {
        return padding;
    }
    let px = match size {
        DialogSize::Size1 => theme.spacing.md,
        DialogSize::Size2 => theme.spacing.lg,
        DialogSize::Size3 => theme.spacing.lg + theme.spacing.xs,
        DialogSize::Size4 => theme.spacing.lg + theme.spacing.sm,
    };
    px.round().max(0.0) as u16
}

fn dialog_radius(theme: &Theme, size: DialogSize) -> f32 {
    match size {
        DialogSize::Size1 | DialogSize::Size2 => theme.radius.md,
        DialogSize::Size3 | DialogSize::Size4 => theme.radius.lg,
    }
}

#[derive(Debug, Default)]
struct DialogOverlayState {
    content_bounds: Option<Rectangle>,
    keyboard_modifiers: keyboard::Modifiers,
    drag_anchor: Option<Point>,
    drag_start_offset: Point,
    drag_offset: Point,
}

struct DialogOverlay<'a, Message> {
    content: Element<'a, Message>,
    props: DialogProps,
    on_close: Message,
    theme: Theme,
}

impl<'a, Message: Clone + 'a> DialogOverlay<'a, Message> {
    fn new(
        content: Element<'a, Message>,
        props: DialogProps,
        on_close: Message,
        theme: Theme,
    ) -> Self {
        Self {
            content,
            props,
            on_close,
            theme,
        }
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for DialogOverlay<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.content.as_widget()]);
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(DialogOverlayState::default())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<DialogOverlayState>()
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<DialogOverlayState>();

        let max = limits.max();
        let size = Size::new(max.width.max(0.0), max.height.max(0.0));

        let (padding_x, padding_top, padding_bottom) =
            if let Some((mw, mh)) = self.props.viewport_margin {
                let px = (size.width * mw).max(self.theme.spacing.lg);
                let py = (size.height * mh).max(self.theme.spacing.lg + self.theme.spacing.sm);
                (px, py, py)
            } else {
                (
                    self.theme.spacing.lg.max(0.0),
                    (self.theme.spacing.lg + self.theme.spacing.sm).max(0.0),
                    (self.theme.spacing.lg + self.theme.spacing.sm).max(0.0),
                )
            };

        let available_w = (size.width - padding_x * 2.0).max(0.0);
        let available_h = (size.height - padding_top - padding_bottom).max(0.0);

        let limits = layout::Limits::new(Size::ZERO, Size::new(available_w, available_h));

        let mut content =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &limits);

        let content_size = content.size();

        let base_x = padding_x + (available_w - content_size.width).max(0.0) / 2.0;

        let desired_y = match self.props.align {
            DialogAlign::Start => padding_top,
            DialogAlign::Center => (size.height - content_size.height).max(0.0) / 2.0,
        };
        let max_y = (size.height - padding_bottom - content_size.height).max(padding_top);
        let base_y = desired_y.clamp(padding_top, max_y);

        let min_x = padding_x;
        let max_x = (size.width - padding_x - content_size.width).max(min_x);
        let min_y = padding_top;
        let max_y = (size.height - padding_bottom - content_size.height).max(min_y);

        let x = (base_x + state.drag_offset.x).clamp(min_x, max_x);
        let y = (base_y + state.drag_offset.y).clamp(min_y, max_y);

        content = content.move_to(Point::new(x, y));
        state.content_bounds = Some(content.bounds());

        layout::Node::with_children(size, vec![content])
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
        let state = tree.state.downcast_mut::<DialogOverlayState>();

        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let over_content = state
                    .content_bounds
                    .map(|bounds| cursor.is_over(bounds))
                    .unwrap_or(false);

                if self.props.draggable
                    && over_content
                    && let Some(position) = cursor.position()
                {
                    state.drag_anchor = Some(position);
                    state.drag_start_offset = state.drag_offset;
                }

                if self.props.close_on_blur && !over_content {
                    shell.publish(self.on_close.clone());
                }

                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if self.props.draggable
                    && let Some(anchor) = state.drag_anchor
                {
                    state.drag_offset = Point::new(
                        state.drag_start_offset.x + position.x - anchor.x,
                        state.drag_start_offset.y + position.y - anchor.y,
                    );
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.drag_anchor = None;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { .. })
                if matches!(
                    overlay_keyboard::command(event),
                    Some(overlay_keyboard::OverlayCommand::Close)
                ) =>
            {
                shell.publish(self.on_close.clone());
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
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
        let state = tree.state.downcast_ref::<DialogOverlayState>();
        let over_content = state
            .content_bounds
            .map(|bounds| cursor.is_over(bounds))
            .unwrap_or(false);

        if over_content {
            if let Some(content_layout) = layout.children().next() {
                return self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content_layout,
                    cursor,
                    viewport,
                    renderer,
                );
            }
        } else if cursor.is_over(layout.bounds()) {
            return mouse::Interaction::Pointer;
        }

        mouse::Interaction::default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let overlay_color = Color {
            a: self.props.overlay_opacity,
            ..Color::BLACK
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            Background::Color(overlay_color),
        );

        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                _theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget_mut().overlay(
                &mut tree.children[0],
                content_layout,
                renderer,
                viewport,
                translation,
            )
        } else {
            None
        }
    }
}

impl<'a, Message: Clone + 'a> From<DialogOverlay<'a, Message>> for Element<'a, Message> {
    fn from(widget: DialogOverlay<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}

pub fn dialog<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    open: bool,
    content: impl Into<Element<'a, Message>>,
    on_close: Message,
    props: DialogProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let base: Element<'a, Message> = base.into();
    if !open {
        return base;
    }

    let theme = theme.clone();
    let padding = dialog_padding(&theme, props.size, props.padding);
    let radius = dialog_radius(&theme, props.size);

    let (w, h) = if props.viewport_margin.is_some() {
        (Length::Fill, Length::Fill)
    } else {
        (Length::Shrink, Length::Shrink)
    };

    let dialog_content = iced::widget::container(content)
        .padding(padding)
        .width(w)
        .height(h)
        .max_width(props.max_width)
        .style(move |_t: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.card)),
            text_color: Some(theme.palette.card_foreground),
            border: Border {
                color: theme.palette.border,
                width: 1.0,
                radius: radius.into(),
            },
            shadow: Shadow {
                color: Color {
                    a: 0.22,
                    ..Color::BLACK
                },
                offset: iced::Vector::new(0.0, 16.0),
                blur_radius: 32.0,
            },
            snap: true,
        })
        .into();

    let overlay: Element<'a, Message> =
        DialogOverlay::new(dialog_content, props, on_close, theme).into();
    iced::widget::stack![base, overlay].into()
}
