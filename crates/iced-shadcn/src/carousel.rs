use iced::widget::{Column, Id, Row, container, scrollable};
use iced::{Element, Length, Size, Task};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button};
use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarouselOptions {
    pub looped: bool,
}

impl CarouselOptions {
    pub fn looped(mut self, looped: bool) -> Self {
        self.looped = looped;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CarouselContentProps {
    pub size: Option<Size>,
    pub spacing: f32,
    pub item_extent: f32,
}

impl Default for CarouselContentProps {
    fn default() -> Self {
        Self {
            size: None,
            spacing: 16.0,
            item_extent: 240.0,
        }
    }
}

impl CarouselContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn item_extent(mut self, item_extent: f32) -> Self {
        self.item_extent = item_extent;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CarouselState {
    pub id: Id,
    pub index: usize,
}

impl CarouselState {
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            index: 0,
        }
    }

    pub fn can_scroll_prev(&self, item_count: usize, opts: CarouselOptions) -> bool {
        item_count > 1 && (opts.looped || self.index > 0)
    }

    pub fn can_scroll_next(&self, item_count: usize, opts: CarouselOptions) -> bool {
        item_count > 1 && (opts.looped || self.index + 1 < item_count)
    }

    pub fn scroll_prev<Message>(
        &mut self,
        item_count: usize,
        opts: CarouselOptions,
        props: CarouselContentProps,
        orientation: CarouselOrientation,
    ) -> Task<Message> {
        if !self.can_scroll_prev(item_count, opts) {
            return Task::none();
        }

        let next = if self.index == 0 {
            item_count.saturating_sub(1)
        } else {
            self.index.saturating_sub(1)
        };

        self.scroll_to(next, item_count, props, orientation)
    }

    pub fn scroll_next<Message>(
        &mut self,
        item_count: usize,
        opts: CarouselOptions,
        props: CarouselContentProps,
        orientation: CarouselOrientation,
    ) -> Task<Message> {
        if !self.can_scroll_next(item_count, opts) {
            return Task::none();
        }

        let last = item_count.saturating_sub(1);
        let next = if self.index >= last {
            0
        } else {
            self.index + 1
        };

        self.scroll_to(next, item_count, props, orientation)
    }

    pub fn scroll_to<Message>(
        &mut self,
        index: usize,
        item_count: usize,
        props: CarouselContentProps,
        orientation: CarouselOrientation,
    ) -> Task<Message> {
        let index = clamp_index(index, item_count);
        self.index = index;

        let span = item_span(props);
        let offset = (index as f32) * span;
        let offset = match orientation {
            CarouselOrientation::Horizontal => scrollable::AbsoluteOffset { x: offset, y: 0.0 },
            CarouselOrientation::Vertical => scrollable::AbsoluteOffset { x: 0.0, y: offset },
        };

        iced::widget::operation::scroll_to(self.id.clone(), offset)
    }

    pub fn sync_from_viewport(
        &mut self,
        viewport: scrollable::Viewport,
        props: CarouselContentProps,
        orientation: CarouselOrientation,
        item_count: usize,
    ) -> usize {
        let span = item_span(props);
        if span <= 0.0 || item_count == 0 {
            self.index = 0;
            return 0;
        }

        let offset = viewport.absolute_offset();
        let main_offset = match orientation {
            CarouselOrientation::Horizontal => offset.x,
            CarouselOrientation::Vertical => offset.y,
        };

        let index = ((main_offset / span).round() as isize).max(0) as usize;
        let index = clamp_index(index, item_count);
        self.index = index;
        index
    }
}

impl Default for CarouselState {
    fn default() -> Self {
        Self::new(Id::unique())
    }
}

pub fn carousel_content<'a, Message: 'a>(
    state: &CarouselState,
    orientation: CarouselOrientation,
    props: CarouselContentProps,
    items: impl IntoIterator<Item = Element<'a, Message>>,
) -> scrollable::Scrollable<'a, Message> {
    let spacing = props.spacing.max(0.0);
    let extent = props.item_extent.max(1.0);
    let items: Vec<_> = items.into_iter().collect();

    let content: Element<'a, Message> = match orientation {
        CarouselOrientation::Horizontal => {
            let mut row = Row::new().spacing(spacing);
            for item in items {
                row = row.push(
                    container(item)
                        .width(Length::Fixed(extent))
                        .height(Length::Fill),
                );
            }
            row.into()
        }
        CarouselOrientation::Vertical => {
            let mut column = Column::new().spacing(spacing);
            for item in items {
                column = column.push(
                    container(item)
                        .width(Length::Fill)
                        .height(Length::Fixed(extent)),
                );
            }
            column.into()
        }
    };

    let scrollbar = scrollable::Scrollbar::hidden();
    let direction = match orientation {
        CarouselOrientation::Horizontal => scrollable::Direction::Horizontal(scrollbar),
        CarouselOrientation::Vertical => scrollable::Direction::Vertical(scrollbar),
    };

    let (width, height) = match props.size {
        Some(size) => (Length::Fixed(size.width), Length::Fixed(size.height)),
        None => (Length::Fill, Length::Fill),
    };

    scrollable(content)
        .id(state.id.clone())
        .direction(direction)
        .width(width)
        .height(height)
}

pub fn carousel_previous<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
    enabled: bool,
    orientation: CarouselOrientation,
    theme: &Theme,
) -> iced::widget::button::Button<'a, Message> {
    let label = match orientation {
        CarouselOrientation::Horizontal => "Prev",
        CarouselOrientation::Vertical => "Up",
    };

    let on_press = if enabled { on_press } else { None };
    button(
        label,
        on_press,
        ButtonProps::new()
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::One),
        theme,
    )
}

pub fn carousel_next<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
    enabled: bool,
    orientation: CarouselOrientation,
    theme: &Theme,
) -> iced::widget::button::Button<'a, Message> {
    let label = match orientation {
        CarouselOrientation::Horizontal => "Next",
        CarouselOrientation::Vertical => "Down",
    };

    let on_press = if enabled { on_press } else { None };
    button(
        label,
        on_press,
        ButtonProps::new()
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::One),
        theme,
    )
}

fn clamp_index(index: usize, item_count: usize) -> usize {
    if item_count == 0 {
        0
    } else {
        index.min(item_count - 1)
    }
}

fn item_span(props: CarouselContentProps) -> f32 {
    props.item_extent.max(1.0) + props.spacing.max(0.0)
}
