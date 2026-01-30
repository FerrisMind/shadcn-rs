use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::keyboard;
use iced::mouse;
use iced::touch;
use iced::{Background, Color, Element, Event, Length, Point, Rectangle, Shadow, Size, Vector};

use crate::overlay::keyboard as overlay_keyboard;
use crate::theme::Theme as ShadcnTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverSize {
    One,
    Two,
    Three,
    Four,
}

#[derive(Clone, Copy, Debug)]
pub struct PopoverProps {
    pub size: PopoverSize,
    pub max_width: u32,
    pub offset: f32,
    pub disabled: bool,
}

impl Default for PopoverProps {
    fn default() -> Self {
        Self {
            size: PopoverSize::Two,
            max_width: 480,
            offset: 8.0,
            disabled: false,
        }
    }
}

impl PopoverProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: PopoverSize) -> Self {
        self.size = size;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width.max(1);
        self
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset.max(0.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

fn padding_px(theme: &ShadcnTheme, size: PopoverSize) -> u16 {
    let px = match size {
        PopoverSize::One => theme.spacing.md,
        PopoverSize::Two => theme.spacing.lg,
        PopoverSize::Three => theme.spacing.lg + theme.spacing.xs,
        PopoverSize::Four => theme.spacing.lg + theme.spacing.sm,
    };
    px.round().max(0.0) as u16
}

fn radius_px(theme: &ShadcnTheme, size: PopoverSize) -> f32 {
    match size {
        PopoverSize::One | PopoverSize::Two => theme.radius.md,
        PopoverSize::Three | PopoverSize::Four => theme.radius.lg,
    }
}

#[derive(Debug, Default)]
struct PopoverState {
    is_open: bool,
    overlay_bounds: Option<Rectangle>,
    keyboard_modifiers: keyboard::Modifiers,
}

pub fn popover<'a, Message: Clone + 'a>(
    trigger: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    props: PopoverProps,
    theme: &ShadcnTheme,
) -> Popover<'a, Message> {
    let padding = padding_px(theme, props.size);
    Popover {
        trigger: trigger.into(),
        content: iced::widget::container(content)
            .padding(padding)
            .width(Length::Shrink)
            .max_width(props.max_width)
            .into(),
        props,
        theme: theme.clone(),
    }
}

pub struct Popover<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    props: PopoverProps,
    theme: ShadcnTheme,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for Popover<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.trigger), Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget(), self.content.as_widget()]);
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(PopoverState::default())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<PopoverState>()
    }

    fn size(&self) -> Size<Length> {
        self.trigger.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.trigger
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
        let state = tree.state.downcast_mut::<PopoverState>();

        if self.props.disabled && state.is_open {
            state.is_open = false;
        }

        self.trigger.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if self.props.disabled {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let over_trigger = cursor.is_over(layout.bounds());
                let over_overlay = state
                    .overlay_bounds
                    .map(|b| cursor.is_over(b))
                    .unwrap_or(false);

                if state.is_open {
                    if over_trigger || !over_overlay {
                        state.is_open = false;
                        shell.capture_event();
                    }
                } else if over_trigger {
                    state.is_open = true;
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { .. })
                if matches!(
                    overlay_keyboard::command(event),
                    Some(overlay_keyboard::OverlayCommand::Close)
                ) =>
            {
                if state.is_open {
                    state.is_open = false;
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            _ => {}
        }

        if !state.is_open {
            state.overlay_bounds = None;
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
        let state = tree.state.downcast_mut::<PopoverState>();
        if !state.is_open {
            return None;
        }

        let bounds = layout.bounds();
        let anchor_position = layout.position() + translation;

        Some(iced::overlay::Element::new(Box::new(PopoverOverlay {
            content: &mut self.content,
            tree: &mut tree.children[1],
            theme: self.theme.clone(),
            props: self.props,
            overlay_bounds: &mut state.overlay_bounds,
            anchor_position,
            target_size: Size::new(bounds.width, bounds.height),
            viewport: *viewport,
        })))
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.trigger.as_widget().mouse_interaction(
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
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.trigger.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }
}

struct PopoverOverlay<'a, 'b, Message> {
    content: &'a mut Element<'b, Message>,
    tree: &'a mut Tree,
    theme: ShadcnTheme,
    props: PopoverProps,
    overlay_bounds: &'a mut Option<Rectangle>,
    anchor_position: Point,
    target_size: Size,
    viewport: Rectangle,
}

impl<Message> iced::advanced::Overlay<Message, iced::Theme, iced::Renderer>
    for PopoverOverlay<'_, '_, Message>
where
    Message: Clone,
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let max_width = bounds.width.max(0.0);
        let max_height = bounds.height.max(0.0);

        let limits = layout::Limits::new(Size::ZERO, Size::new(max_width, max_height));

        let node = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let size = node.size();

        let collision_padding = 10.0;
        let space_below = bounds.height - (self.anchor_position.y + self.target_size.height);
        let space_above = self.anchor_position.y;

        let available_x = (bounds.width - size.width - collision_padding).max(collision_padding);
        let clamped_x = self.anchor_position.x.clamp(collision_padding, available_x);

        let position = if space_below >= space_above {
            Point::new(
                clamped_x,
                self.anchor_position.y + self.target_size.height + self.props.offset,
            )
        } else {
            Point::new(
                clamped_x,
                self.anchor_position.y - size.height - self.props.offset,
            )
        };

        let node = node.move_to(position);
        *self.overlay_bounds = Some(node.bounds());
        node
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
        let bounds = layout.bounds();
        self.content.as_widget_mut().update(
            self.tree, event, layout, cursor, renderer, clipboard, shell, &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout,
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
        let bounds = layout.bounds();
        let palette = self.theme.palette;
        let radius = radius_px(&self.theme, self.props.size);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: palette.border,
                    width: 1.0,
                    radius: radius.into(),
                },
                shadow: Shadow {
                    color: Color {
                        a: 0.18,
                        ..Color::BLACK
                    },
                    offset: iced::Vector::new(0.0, 8.0),
                    blur_radius: 22.0,
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
            layout,
            cursor,
            &self.viewport,
        );
    }
}

impl<'a, Message: Clone + 'a> From<Popover<'a, Message>> for Element<'a, Message> {
    fn from(widget: Popover<'a, Message>) -> Element<'a, Message> {
        Element::new(widget)
    }
}
