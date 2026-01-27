use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::widget::{container, responsive, row, text};
use iced::{Background, Element, Event, Length, Point, Rectangle, Size};
use iced::mouse;
use std::hash::Hash;
use std::rc::Rc;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub struct ResizablePanelGroupProps<Id: Hash> {
    pub id_source: Id,
    pub direction: ResizableDirection,
    pub auto_save_id: Option<String>,
}

impl<IdType: Hash> ResizablePanelGroupProps<IdType> {
    pub fn new(id_source: IdType) -> Self {
        Self {
            id_source,
            direction: ResizableDirection::Horizontal,
            auto_save_id: None,
        }
    }

    pub fn direction(mut self, direction: ResizableDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn auto_save_id(mut self, id: impl Into<String>) -> Self {
        self.auto_save_id = Some(id.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct ResizablePanelProps {
    pub default_size: f32,
    pub min_size: Option<f32>,
    pub max_size: Option<f32>,
    pub collapsible: bool,
}

impl ResizablePanelProps {
    pub fn new(default_size: f32) -> Self {
        Self {
            default_size: default_size.clamp(0.0, 100.0),
            min_size: None,
            max_size: None,
            collapsible: false,
        }
    }

    pub fn min_size(mut self, min: f32) -> Self {
        self.min_size = Some(min.clamp(0.0, 100.0));
        self
    }

    pub fn max_size(mut self, max: f32) -> Self {
        self.max_size = Some(max.clamp(0.0, 100.0));
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn clamp_size(&self, size: f32) -> f32 {
        let min = self.min_size.unwrap_or(0.0);
        let max = self.max_size.unwrap_or(100.0);
        size.clamp(min, max)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResizableHandleProps {
    pub with_handle: bool,
    pub disabled: bool,
}

impl ResizableHandleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_handle(mut self, with_handle: bool) -> Self {
        self.with_handle = with_handle;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct ResizableContext<'a, Message> {
    direction: ResizableDirection,
    sizes: &'a [f32],
    total_size: f32,
    on_resize: Option<Rc<dyn Fn(Vec<f32>) -> Message + 'a>>,
}

impl<'a, Message> ResizableContext<'a, Message> {
    pub fn get_size(&self, index: usize) -> f32 {
        self.sizes.get(index).copied().unwrap_or(0.0)
    }

    pub fn get_pixel_size(&self, index: usize) -> f32 {
        let percent = self.get_size(index);
        self.total_size * percent / 100.0
    }

    pub fn direction(&self) -> ResizableDirection {
        self.direction
    }

    pub fn resize(&self, handle_index: usize, delta_percent: f32) -> Option<Vec<f32>> {
        resize_sizes(self.sizes, handle_index, delta_percent)
    }
}

pub fn resizable_panel_group<'a, Message: Clone + 'a, IdType: Hash, F, C>(
    props: ResizablePanelGroupProps<IdType>,
    sizes: &'a [f32],
    on_resize: Option<F>,
    _theme: &'a Theme,
    add_contents: C,
) -> Element<'a, Message>
where
    F: Fn(Vec<f32>) -> Message + 'a,
    C: Fn(&ResizableContext<'a, Message>) -> Vec<Element<'a, Message>> + 'a,
{
    let direction = props.direction;
    let on_resize = on_resize.map(|f| Rc::new(f) as Rc<dyn Fn(Vec<f32>) -> Message + 'a>);

    responsive(move |size| {
        let on_resize = on_resize.clone();
        let total_size = match direction {
            ResizableDirection::Horizontal => size.width,
            ResizableDirection::Vertical => size.height,
        };

        let ctx = ResizableContext {
            direction,
            sizes,
            total_size,
            on_resize,
        };

        let children = add_contents(&ctx);
        match direction {
            ResizableDirection::Horizontal => row(children).spacing(0).into(),
            ResizableDirection::Vertical => iced::widget::column(children).spacing(0).into(),
        }
    })
    .into()
}

pub fn resizable_panel<'a, Message: Clone + 'a>(
    ctx: &ResizableContext<'a, Message>,
    props: ResizablePanelProps,
    index: usize,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let size_percent = ctx.get_size(index);
    let clamped_percent = props.clamp_size(size_percent);

    let size_px = ctx.total_size * clamped_percent / 100.0;

    match ctx.direction {
        ResizableDirection::Horizontal => container(content)
            .width(Length::Fixed(size_px.max(1.0)))
            .height(Length::Fill)
            .into(),
        ResizableDirection::Vertical => container(content)
            .width(Length::Fill)
            .height(Length::Fixed(size_px.max(1.0)))
            .into(),
    }
}

pub fn resizable_handle<'a, Message: Clone + 'a>(
    ctx: &ResizableContext<'a, Message>,
    props: ResizableHandleProps,
    handle_index: usize,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let (width, height, icon) = match ctx.direction {
        ResizableDirection::Horizontal => (Length::Fixed(4.0), Length::Fill, "⋮"),
        ResizableDirection::Vertical => (Length::Fill, Length::Fixed(4.0), "⋯"),
    };

    let grip: Element<'a, Message> = if props.with_handle {
        text(icon)
            .size(10)
            .style(move |_t: &iced::Theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            })
            .into()
    } else {
        container(text("")).into()
    };

    let base = container(grip)
        .width(width)
        .height(height)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.border)),
            ..Default::default()
        });

    let widget = ResizableHandleWidget {
        content: base.into(),
        direction: ctx.direction,
        handle_index,
        total_size: ctx.total_size,
        sizes: ctx.sizes,
        on_resize: ctx.on_resize.clone(),
        disabled: props.disabled,
    };

    Element::new(widget)
}

#[derive(Default)]
struct ResizableHandleState {
    dragging: bool,
    last_position: Option<Point>,
}

struct ResizableHandleWidget<'a, Message> {
    content: Element<'a, Message>,
    direction: ResizableDirection,
    handle_index: usize,
    total_size: f32,
    sizes: &'a [f32],
    on_resize: Option<Rc<dyn Fn(Vec<f32>) -> Message + 'a>>,
    disabled: bool,
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for ResizableHandleWidget<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<ResizableHandleState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(ResizableHandleState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
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
        let state = tree.state.downcast_mut::<ResizableHandleState>();

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

        if self.disabled {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds()) {
                    state.dragging = true;
                    state.last_position = cursor.position();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.dragging {
                    state.dragging = false;
                    state.last_position = None;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.dragging {
                    if let Some(last) = state.last_position {
                        let delta_px = match self.direction {
                            ResizableDirection::Horizontal => position.x - last.x,
                            ResizableDirection::Vertical => position.y - last.y,
                        };

                        let delta_percent = if self.total_size > 0.0 {
                            delta_px / self.total_size * 100.0
                        } else {
                            0.0
                        };

                        if delta_percent.abs() > 0.0
                            && let Some(on_resize) = self.on_resize.as_ref()
                            && let Some(next) =
                                resize_sizes(self.sizes, self.handle_index, delta_percent)
                        {
                            shell.publish(on_resize(next));
                            shell.capture_event();
                        }
                        state.last_position = Some(*position);
                    } else {
                        state.last_position = Some(*position);
                    }
                }
            }
            _ => {}
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

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.disabled {
            return mouse::Interaction::Idle;
        }

        if cursor.is_over(layout.bounds()) {
            match self.direction {
                ResizableDirection::Horizontal => mouse::Interaction::ResizingHorizontally,
                ResizableDirection::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::Idle
        }
    }
}

fn resize_sizes(sizes: &[f32], handle_index: usize, delta_percent: f32) -> Option<Vec<f32>> {
    if handle_index >= sizes.len().saturating_sub(1) {
        return None;
    }

    let left_idx = handle_index;
    let right_idx = handle_index + 1;

    let left_size = sizes[left_idx];
    let right_size = sizes[right_idx];

    let new_left = (left_size + delta_percent).clamp(5.0, 95.0);
    let total = left_size + right_size;
    let adjusted_left = new_left.min(total - 5.0);
    let adjusted_right = total - adjusted_left;

    let mut next = sizes.to_vec();
    next[left_idx] = adjusted_left;
    next[right_idx] = adjusted_right;
    Some(next)
}
