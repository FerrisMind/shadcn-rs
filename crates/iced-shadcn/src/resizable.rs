use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::mouse;
use iced::widget::{column, container, responsive, row, stack, text};
use iced::{Background, Element, Event, Length, Point, Rectangle, Size};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::theme::Theme;

const HANDLE_VISUAL_THICKNESS: f32 = 1.0;
const HANDLE_HIT_THICKNESS: f32 = 4.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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
    pub collapsed_size: Option<f32>,
}

impl ResizablePanelProps {
    pub fn new(default_size: f32) -> Self {
        Self {
            default_size: default_size.clamp(0.0, 100.0),
            min_size: None,
            max_size: None,
            collapsible: false,
            collapsed_size: None,
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

    pub fn collapsed_size(mut self, collapsed_size: f32) -> Self {
        self.collapsed_size = Some(collapsed_size.clamp(0.0, 100.0));
        self
    }

    pub fn clamp_size(&self, size: f32) -> f32 {
        let min = if self.collapsible {
            self.collapsed_size.unwrap_or(0.0)
        } else {
            self.min_size.unwrap_or(0.0)
        };
        let max = self.max_size.unwrap_or(100.0);
        size.clamp(min, max)
    }
}

#[derive(Clone, Debug)]
pub struct ResizableHandleProps {
    pub with_handle: bool,
    pub disabled: bool,
    pub show_line: bool,
}

impl Default for ResizableHandleProps {
    fn default() -> Self {
        Self {
            with_handle: false,
            disabled: false,
            show_line: true,
        }
    }
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

    pub fn show_line(mut self, show_line: bool) -> Self {
        self.show_line = show_line;
        self
    }
}

pub struct ResizableContext<'a, Message> {
    group_id: u64,
    direction: ResizableDirection,
    sizes: &'a [f32],
    panel_space: f32,
    on_resize: Option<Rc<dyn Fn(Vec<f32>) -> Message + 'a>>,
    panel_props: Rc<RefCell<Vec<Option<ResizablePanelProps>>>>,
}

impl<'a, Message> ResizableContext<'a, Message> {
    pub fn get_size(&self, index: usize) -> f32 {
        self.sizes.get(index).copied().unwrap_or(0.0)
    }

    pub fn get_pixel_size(&self, index: usize) -> f32 {
        let percent = self.get_size(index);
        self.panel_space * percent / 100.0
    }

    pub fn direction(&self) -> ResizableDirection {
        self.direction
    }

    pub fn resize(&self, handle_index: usize, delta_percent: f32) -> Option<Vec<f32>> {
        let panel_props = self.panel_props.borrow();
        if panel_props.iter().all(Option::is_none) {
            resize_sizes(self.sizes, handle_index, delta_percent)
        } else {
            resize_sizes_with_constraints(
                self.sizes,
                handle_index,
                delta_percent,
                panel_props.as_slice(),
            )
        }
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
    let mut hasher = DefaultHasher::new();
    props.id_source.hash(&mut hasher);
    props.direction.hash(&mut hasher);
    props.auto_save_id.hash(&mut hasher);
    let group_id = hasher.finish();

    let direction = props.direction;
    let on_resize = on_resize.map(|f| Rc::new(f) as Rc<dyn Fn(Vec<f32>) -> Message + 'a>);

    responsive(move |size| {
        let on_resize = on_resize.clone();
        let panel_props = Rc::new(RefCell::new(vec![None; sizes.len()]));
        let total_size = match direction {
            ResizableDirection::Horizontal => size.width,
            ResizableDirection::Vertical => size.height,
        };
        let panel_space = available_panel_space(total_size, sizes.len());

        let ctx = ResizableContext {
            group_id,
            direction,
            sizes,
            panel_space,
            on_resize,
            panel_props,
        };

        let children = add_contents(&ctx);
        match direction {
            ResizableDirection::Horizontal => row(children)
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            ResizableDirection::Vertical => column(children)
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
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
    register_panel_props(&ctx.panel_props, index, props.clone());

    let size_percent = ctx.get_size(index);
    let clamped_percent = props.clamp_size(size_percent);
    let size_px = ctx.panel_space * clamped_percent / 100.0;

    match ctx.direction {
        ResizableDirection::Horizontal => container(content)
            .width(Length::Fixed(size_px.max(0.0)))
            .height(Length::Fill)
            .into(),
        ResizableDirection::Vertical => container(content)
            .width(Length::Fill)
            .height(Length::Fixed(size_px.max(0.0)))
            .into(),
    }
}

pub fn resizable_handle<'a, Message: Clone + 'a>(
    ctx: &ResizableContext<'a, Message>,
    props: ResizableHandleProps,
    handle_index: usize,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let (visual_width, visual_height, hit_width, hit_height, icon) = match ctx.direction {
        ResizableDirection::Horizontal => (
            Length::Fixed(HANDLE_VISUAL_THICKNESS),
            Length::Fill,
            Length::Fixed(HANDLE_HIT_THICKNESS),
            Length::Fill,
            "⋮",
        ),
        ResizableDirection::Vertical => (
            Length::Fill,
            Length::Fixed(HANDLE_VISUAL_THICKNESS),
            Length::Fill,
            Length::Fixed(HANDLE_HIT_THICKNESS),
            "⋯",
        ),
    };

    let line_color = if props.show_line {
        theme.palette.border
    } else {
        iced::Color::TRANSPARENT
    };

    let line = container(text(""))
        .width(visual_width)
        .height(visual_height)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(line_color)),
            ..Default::default()
        });

    let mut layers = vec![
        container(line)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ];

    if props.with_handle {
        let (grip_width, grip_height) = match ctx.direction {
            ResizableDirection::Horizontal => (Length::Fixed(12.0), Length::Fixed(16.0)),
            ResizableDirection::Vertical => (Length::Fixed(16.0), Length::Fixed(12.0)),
        };

        let grip = container(text(icon).size(10).style(move |_t: &iced::Theme| {
            iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }
        }))
        .width(grip_width)
        .height(grip_height)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_t| iced::widget::container::Style {
            background: Some(Background::Color(theme.palette.border)),
            border: iced::Border {
                color: theme.palette.muted,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        });

        layers.push(
            container(grip)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        );
    }

    let base = container(stack(layers))
        .width(hit_width)
        .height(hit_height)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    let widget = ResizableHandleWidget {
        content: base.into(),
        group_id: ctx.group_id,
        direction: ctx.direction,
        handle_index,
        panel_space: ctx.panel_space,
        sizes: ctx.sizes,
        on_resize: ctx.on_resize.clone(),
        panel_props: Rc::clone(&ctx.panel_props),
        disabled: props.disabled,
    };

    Element::new(widget)
}

#[derive(Default)]
struct ResizableHandleState;

#[derive(Clone, Debug)]
struct ActiveDrag {
    origin: Point,
    start_sizes: Vec<f32>,
}

thread_local! {
    static ACTIVE_DRAGS: RefCell<HashMap<(u64, usize, ResizableDirection), ActiveDrag>> =
        RefCell::new(HashMap::new());
}

struct ResizableHandleWidget<'a, Message> {
    content: Element<'a, Message>,
    group_id: u64,
    direction: ResizableDirection,
    handle_index: usize,
    panel_space: f32,
    sizes: &'a [f32],
    on_resize: Option<Rc<dyn Fn(Vec<f32>) -> Message + 'a>>,
    panel_props: Rc<RefCell<Vec<Option<ResizablePanelProps>>>>,
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
        iced::advanced::widget::tree::State::new(ResizableHandleState)
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
                if let Some(origin) = cursor.position_over(layout.bounds()) {
                    set_active_drag(
                        self.group_id,
                        self.handle_index,
                        self.direction,
                        ActiveDrag {
                            origin,
                            start_sizes: self.sizes.to_vec(),
                        },
                    );
                    trace_resize_event(
                        self.handle_index,
                        self.direction,
                        "button-pressed",
                        layout.bounds(),
                        Some(origin),
                        self.sizes,
                        None,
                    );
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if has_active_drag(self.group_id, self.handle_index, self.direction) =>
            {
                clear_active_drag(self.group_id, self.handle_index, self.direction);
                trace_resize_event(
                    self.handle_index,
                    self.direction,
                    "button-released",
                    layout.bounds(),
                    cursor.position(),
                    self.sizes,
                    None,
                );
                shell.request_redraw();
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(active_drag) =
                    active_drag(self.group_id, self.handle_index, self.direction)
                {
                    let panel_props = self.panel_props.borrow();

                    if let Some(next) = resize_from_drag(
                        active_drag.start_sizes.as_slice(),
                        self.handle_index,
                        self.direction,
                        self.panel_space,
                        panel_props.as_slice(),
                        active_drag.origin,
                        *position,
                    ) {
                        shell.capture_event();
                        trace_resize_event(
                            self.handle_index,
                            self.direction,
                            "cursor-moved",
                            layout.bounds(),
                            Some(*position),
                            self.sizes,
                            Some(&next),
                        );

                        set_active_drag(
                            self.group_id,
                            self.handle_index,
                            self.direction,
                            ActiveDrag {
                                origin: *position,
                                start_sizes: next.clone(),
                            },
                        );

                        if !sizes_match(self.sizes, &next) {
                            if let Some(on_resize) = self.on_resize.as_ref() {
                                shell.publish(on_resize(next));
                            }

                            shell.request_redraw();
                        }
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

        if has_active_drag(self.group_id, self.handle_index, self.direction)
            || cursor.is_over(layout.bounds())
        {
            match self.direction {
                ResizableDirection::Horizontal => mouse::Interaction::ResizingHorizontally,
                ResizableDirection::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::Idle
        }
    }
}

fn available_panel_space(total_size: f32, panel_count: usize) -> f32 {
    if !total_size.is_finite() || total_size <= 0.0 {
        return total_size;
    }

    let handle_count = panel_count.saturating_sub(1) as f32;
    (total_size - handle_count * HANDLE_HIT_THICKNESS).max(1.0)
}

fn resize_sizes(sizes: &[f32], handle_index: usize, delta_percent: f32) -> Option<Vec<f32>> {
    resize_sizes_unconstrained(sizes, handle_index, delta_percent)
}

fn resize_sizes_unconstrained(
    sizes: &[f32],
    handle_index: usize,
    delta_percent: f32,
) -> Option<Vec<f32>> {
    if handle_index >= sizes.len().saturating_sub(1) {
        return None;
    }

    let left_idx = handle_index;
    let right_idx = handle_index + 1;

    let left_size = sizes[left_idx];
    let right_size = sizes[right_idx];

    let total = left_size + right_size;
    let adjusted_left = (left_size + delta_percent).clamp(0.0, total);
    let adjusted_right = total - adjusted_left;

    let mut next = sizes.to_vec();
    next[left_idx] = adjusted_left;
    next[right_idx] = adjusted_right;
    Some(next)
}

fn resize_sizes_with_constraints(
    sizes: &[f32],
    handle_index: usize,
    delta_percent: f32,
    panel_props: &[Option<ResizablePanelProps>],
) -> Option<Vec<f32>> {
    if panel_props.is_empty() {
        return resize_sizes_unconstrained(sizes, handle_index, delta_percent);
    }

    if handle_index >= sizes.len().saturating_sub(1) {
        return None;
    }

    let left_idx = handle_index;
    let right_idx = handle_index + 1;

    let left_size = sizes[left_idx];
    let right_size = sizes[right_idx];
    let total = left_size + right_size;
    let target_left = left_size + delta_percent;

    let left_constraints =
        PanelConstraints::from_props(panel_props.get(left_idx).and_then(Option::as_ref));
    let right_constraints =
        PanelConstraints::from_props(panel_props.get(right_idx).and_then(Option::as_ref));

    if let Some(collapsed) = left_constraints.collapsed
        && target_left < left_constraints.expanded_min
        && (left_constraints.is_collapsed(left_size) || !right_constraints.can_collapse_to(total))
    {
        return Some(resized_pair(
            sizes,
            left_idx,
            right_idx,
            collapsed,
            total - collapsed,
        ));
    }

    let target_right = total - target_left;

    if let Some(collapsed) = right_constraints.collapsed
        && target_right < right_constraints.expanded_min
        && (right_constraints.is_collapsed(right_size) || !left_constraints.can_collapse_to(total))
    {
        let adjusted_left = total - collapsed;
        return Some(resized_pair(
            sizes,
            left_idx,
            right_idx,
            adjusted_left,
            collapsed,
        ));
    }

    let adjusted_left = solve_left_size(target_left, total, left_constraints, right_constraints)?;
    let adjusted_right = total - adjusted_left;
    Some(resized_pair(
        sizes,
        left_idx,
        right_idx,
        adjusted_left,
        adjusted_right,
    ))
}

fn resize_from_drag(
    sizes: &[f32],
    handle_index: usize,
    direction: ResizableDirection,
    total_size: f32,
    panel_props: &[Option<ResizablePanelProps>],
    origin: Point,
    current: Point,
) -> Option<Vec<f32>> {
    let delta_px = match direction {
        ResizableDirection::Horizontal => current.x - origin.x,
        ResizableDirection::Vertical => current.y - origin.y,
    };

    let delta_percent = if total_size > 0.0 {
        delta_px / total_size * 100.0
    } else {
        0.0
    };

    resize_sizes_with_constraints(sizes, handle_index, delta_percent, panel_props)
}

#[cfg(test)]
fn resize_from_cursor(
    sizes: &[f32],
    handle_index: usize,
    direction: ResizableDirection,
    panel_space: f32,
    panel_props: &[Option<ResizablePanelProps>],
    handle_bounds: Rectangle,
    current: Point,
) -> Option<Vec<f32>> {
    if handle_index >= sizes.len().saturating_sub(1) || panel_space <= 0.0 {
        return None;
    }

    let left_idx = handle_index;
    let right_idx = handle_index + 1;

    let left_px = panel_space * sizes[left_idx] / 100.0;
    let right_px = panel_space * sizes[right_idx] / 100.0;
    let pair_total_px = left_px + right_px;

    let pair_start = match direction {
        ResizableDirection::Horizontal => handle_bounds.x - left_px,
        ResizableDirection::Vertical => handle_bounds.y - left_px,
    };

    let pointer_main = match direction {
        ResizableDirection::Horizontal => current.x,
        ResizableDirection::Vertical => current.y,
    };

    let target_left_px = (pointer_main - pair_start).clamp(0.0, pair_total_px);
    let target_left = target_left_px / panel_space * 100.0;
    let delta_percent = target_left - sizes[left_idx];

    resize_sizes_with_constraints(sizes, handle_index, delta_percent, panel_props)
}

fn register_panel_props(
    panel_props: &Rc<RefCell<Vec<Option<ResizablePanelProps>>>>,
    index: usize,
    props: ResizablePanelProps,
) {
    let mut panel_props = panel_props.borrow_mut();

    if index >= panel_props.len() {
        panel_props.resize(index + 1, None);
    }

    panel_props[index] = Some(props);
}

#[derive(Clone, Copy, Debug)]
struct PanelConstraints {
    collapsed: Option<f32>,
    expanded_min: f32,
    max: f32,
}

impl PanelConstraints {
    fn from_props(props: Option<&ResizablePanelProps>) -> Self {
        let expanded_min = props
            .and_then(|props| props.min_size)
            .unwrap_or(0.0)
            .clamp(0.0, 100.0);
        let max = props
            .and_then(|props| props.max_size)
            .unwrap_or(100.0)
            .max(expanded_min);
        let collapsed = props.and_then(|props| {
            if props.collapsible {
                Some(props.collapsed_size.unwrap_or(0.0).clamp(0.0, max))
            } else {
                None
            }
        });

        Self {
            collapsed,
            expanded_min,
            max,
        }
    }

    fn segments(self) -> Vec<(f32, f32)> {
        let mut segments = Vec::with_capacity(2);

        if let Some(collapsed) = self.collapsed {
            segments.push((collapsed, collapsed));
        }

        segments.push((self.expanded_min, self.max));
        segments
    }

    fn can_collapse_to(self, total: f32) -> bool {
        self.collapsed.is_some_and(|collapsed| collapsed <= total)
    }

    fn is_collapsed(self, size: f32) -> bool {
        self.collapsed
            .is_some_and(|collapsed| (size - collapsed).abs() <= f32::EPSILON)
    }
}

fn solve_left_size(
    target_left: f32,
    total: f32,
    left: PanelConstraints,
    right: PanelConstraints,
) -> Option<f32> {
    let mut best: Option<(f32, f32)> = None;

    for (left_min, left_max) in left.segments() {
        for (right_min, right_max) in right.segments() {
            let candidate_min = left_min.max(total - right_max);
            let candidate_max = left_max.min(total - right_min);

            if candidate_min > candidate_max {
                continue;
            }

            let candidate = target_left.clamp(candidate_min, candidate_max);
            let distance = (candidate - target_left).abs();

            if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                best = Some((candidate, distance));
            }
        }
    }

    best.map(|(candidate, _)| candidate)
}

fn resized_pair(
    sizes: &[f32],
    left_idx: usize,
    right_idx: usize,
    left_size: f32,
    right_size: f32,
) -> Vec<f32> {
    let mut next = sizes.to_vec();
    next[left_idx] = left_size;
    next[right_idx] = right_size;
    next
}

fn sizes_match(left: &[f32], right: &[f32]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|(left, right)| (left - right).abs() <= 0.001)
}

fn active_drag(
    group_id: u64,
    handle_index: usize,
    direction: ResizableDirection,
) -> Option<ActiveDrag> {
    ACTIVE_DRAGS.with(|drags| {
        drags
            .borrow()
            .get(&(group_id, handle_index, direction))
            .cloned()
    })
}

fn set_active_drag(
    group_id: u64,
    handle_index: usize,
    direction: ResizableDirection,
    drag: ActiveDrag,
) {
    ACTIVE_DRAGS.with(|drags| {
        drags
            .borrow_mut()
            .insert((group_id, handle_index, direction), drag);
    });
}

fn clear_active_drag(group_id: u64, handle_index: usize, direction: ResizableDirection) {
    ACTIVE_DRAGS.with(|drags| {
        drags
            .borrow_mut()
            .remove(&(group_id, handle_index, direction));
    });
}

fn has_active_drag(group_id: u64, handle_index: usize, direction: ResizableDirection) -> bool {
    ACTIVE_DRAGS.with(|drags| {
        drags
            .borrow()
            .contains_key(&(group_id, handle_index, direction))
    })
}

fn trace_resize_event(
    handle_index: usize,
    direction: ResizableDirection,
    stage: &str,
    bounds: Rectangle,
    cursor: Option<Point>,
    sizes: &[f32],
    next: Option<&[f32]>,
) {
    if env::var_os("ICED_SHADCN_RESIZABLE_TRACE").is_none() {
        return;
    }

    eprintln!(
        "[resizable] handle={handle_index} dir={direction:?} stage={stage} bounds=({:.1},{:.1},{:.1},{:.1}) cursor={} sizes={sizes:?} next={}",
        bounds.x,
        bounds.y,
        bounds.width,
        bounds.height,
        format_trace_point(cursor),
        format_trace_sizes(next),
    );
}

fn format_trace_point(cursor: Option<Point>) -> String {
    match cursor {
        Some(cursor) => format!("({:.1},{:.1})", cursor.x, cursor.y),
        None => "none".to_owned(),
    }
}

fn format_trace_sizes(sizes: Option<&[f32]>) -> String {
    match sizes {
        Some(sizes) => format!("{sizes:?}"),
        None => "none".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ResizableDirection, ResizablePanelProps, available_panel_space, resize_from_cursor,
        resize_from_drag, resize_sizes, resize_sizes_with_constraints,
    };
    use iced::{Point, Rectangle};

    #[test]
    fn resize_sizes_moves_space_between_adjacent_panels() {
        let next = resize_sizes(&[20.0, 60.0, 20.0], 0, 10.0).expect("valid handle");

        assert_eq!(next, vec![30.0, 50.0, 20.0]);
    }

    #[test]
    fn resize_sizes_allows_panels_to_reach_zero_by_default() {
        let next = resize_sizes(&[20.0, 60.0, 20.0], 0, -30.0).expect("valid handle");

        assert_eq!(next, vec![0.0, 80.0, 20.0]);
    }

    #[test]
    fn resize_sizes_rejects_out_of_bounds_handle() {
        assert!(resize_sizes(&[50.0, 50.0], 1, 10.0).is_none());
    }

    #[test]
    fn available_panel_space_excludes_handle_hit_areas() {
        assert_eq!(available_panel_space(300.0, 3), 292.0);
    }

    #[test]
    fn resize_from_drag_accumulates_against_drag_origin() {
        let next = resize_from_drag(
            &[20.0, 60.0, 20.0],
            0,
            ResizableDirection::Horizontal,
            100.0,
            &[],
            Point::new(0.0, 0.0),
            Point::new(20.0, 0.0),
        )
        .expect("valid drag");

        assert_eq!(next, vec![40.0, 40.0, 20.0]);
    }

    #[test]
    fn resize_from_drag_supports_vertical_axis() {
        let next = resize_from_drag(
            &[68.0, 32.0],
            0,
            ResizableDirection::Vertical,
            200.0,
            &[],
            Point::new(0.0, 50.0),
            Point::new(0.0, 10.0),
        )
        .expect("valid drag");

        assert_eq!(next, vec![48.0, 52.0]);
    }

    #[test]
    fn resize_from_drag_recovers_from_zero_without_handle_deadzone() {
        let next = resize_from_drag(
            &[0.0, 100.0],
            0,
            ResizableDirection::Horizontal,
            100.0,
            &[],
            Point::new(46.0, 0.0),
            Point::new(56.0, 0.0),
        )
        .expect("valid drag");

        assert_eq!(next, vec![10.0, 90.0]);
    }

    #[test]
    fn resize_from_drag_recovers_vertically_from_zero_without_deadzone() {
        let next = resize_from_drag(
            &[0.0, 100.0],
            0,
            ResizableDirection::Vertical,
            100.0,
            &[],
            Point::new(0.0, 46.0),
            Point::new(0.0, 56.0),
        )
        .expect("valid drag");

        assert_eq!(next, vec![10.0, 90.0]);
    }

    #[test]
    fn resize_from_cursor_recovers_immediately_from_zero_edge() {
        let clamped_sizes = resize_from_cursor(
            &[25.0, 75.0],
            0,
            ResizableDirection::Horizontal,
            100.0,
            &[],
            Rectangle {
                x: 25.0,
                y: 0.0,
                width: 4.0,
                height: 100.0,
            },
            Point::new(0.0, 0.0),
        )
        .expect("valid drag");

        assert_eq!(clamped_sizes, vec![0.0, 100.0]);

        let expanded_sizes = resize_from_cursor(
            &clamped_sizes,
            0,
            ResizableDirection::Horizontal,
            100.0,
            &[],
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 4.0,
                height: 100.0,
            },
            Point::new(20.0, 0.0),
        )
        .expect("valid reverse drag");

        assert_eq!(expanded_sizes, vec![20.0, 80.0]);
    }

    #[test]
    fn resize_from_cursor_supports_vertical_zero_edge_recovery() {
        let clamped_sizes = resize_from_cursor(
            &[25.0, 75.0],
            0,
            ResizableDirection::Vertical,
            100.0,
            &[],
            Rectangle {
                x: 0.0,
                y: 25.0,
                width: 100.0,
                height: 4.0,
            },
            Point::new(0.0, 0.0),
        )
        .expect("valid drag");

        assert_eq!(clamped_sizes, vec![0.0, 100.0]);

        let expanded_sizes = resize_from_cursor(
            &clamped_sizes,
            0,
            ResizableDirection::Vertical,
            100.0,
            &[],
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 4.0,
            },
            Point::new(0.0, 20.0),
        )
        .expect("valid reverse drag");

        assert_eq!(expanded_sizes, vec![20.0, 80.0]);
    }

    #[test]
    fn resize_sizes_snaps_collapsible_panel_to_collapsed_size() {
        let panel_props = [
            Some(
                ResizablePanelProps::new(50.0)
                    .min_size(25.0)
                    .collapsible(true)
                    .collapsed_size(0.0),
            ),
            Some(ResizablePanelProps::new(50.0).min_size(25.0)),
        ];

        let next = resize_sizes_with_constraints(&[30.0, 70.0], 0, -20.0, &panel_props)
            .expect("valid handle");

        assert_eq!(next, vec![0.0, 100.0]);
    }

    #[test]
    fn resize_sizes_keeps_collapsed_panel_collapsed_until_min_size_threshold() {
        let panel_props = [
            Some(
                ResizablePanelProps::new(0.0)
                    .min_size(25.0)
                    .collapsible(true)
                    .collapsed_size(0.0),
            ),
            Some(ResizablePanelProps::new(100.0).min_size(25.0)),
        ];

        let next = resize_sizes_with_constraints(&[0.0, 100.0], 0, 20.0, &panel_props)
            .expect("valid handle");
        let expanded = resize_sizes_with_constraints(&[0.0, 100.0], 0, 30.0, &panel_props)
            .expect("valid handle");

        assert_eq!(next, vec![0.0, 100.0]);
        assert_eq!(expanded, vec![30.0, 70.0]);
    }
}
