//! Iced layout and text rendering for the field component.

use std::cell::Cell;

use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout::{self, Layout};
use crate::iced_compat::advanced::widget::{Operation, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, overlay, renderer};
use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{self, column, container, row, text as iced_text};
use crate::iced_compat::{Element, Event, Length, Padding, Rectangle, Size, Vector, mouse};
use crate::recipes::iced_font_weight;
use crate::theme::Theme;
use iced_core::Alignment;
use shadcn_common::FontWeight;

use super::FieldTextContent;
use super::geometry;
use super::types::{FieldLegendVariant, FieldOrientation};

/// Builds a field root with a layout that can react to the available width.
pub(super) fn build_field<'a, Message>(
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    options: FieldOptions,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let FieldOptions {
        orientation,
        width,
        spacing,
        responsive_breakpoint,
        invalid,
        disabled,
    } = options;

    let spacing = spacing.unwrap_or(geometry::FIELD_GAP_PX);
    let field = FieldWidget {
        children,
        orientation,
        width,
        spacing,
        responsive_breakpoint,
        active_axis: Cell::new(FieldAxis::Vertical),
    };

    let content: Element<'a, Message> = field.into();
    if !invalid && !disabled {
        return content;
    }

    let mut color = if invalid {
        theme.palette.destructive
    } else {
        theme.palette.foreground
    };
    if disabled {
        color = with_alpha(color, 0.5);
    }

    container(content)
        .width(width)
        .style(move |_| container::Style {
            text_color: Some(color),
            ..container::Style::default()
        })
        .into()
}

/// Private render-time configuration collected from the public [`Field`]
/// builder before it becomes an iced widget.
pub(super) struct FieldOptions {
    pub(super) orientation: FieldOrientation,
    pub(super) width: Length,
    pub(super) spacing: Option<f32>,
    pub(super) responsive_breakpoint: f32,
    pub(super) invalid: bool,
    pub(super) disabled: bool,
}

/// Builds a vertical group of fields.
pub(super) fn build_group<'a, Message>(
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: Option<f32>,
    checkbox_group: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let default_spacing = if checkbox_group {
        geometry::CHECKBOX_GROUP_GAP_PX
    } else {
        geometry::FIELD_GROUP_GAP_PX
    };

    column(children)
        .spacing(spacing.unwrap_or(default_spacing))
        .width(width)
        .into()
}

/// Builds a semantic field set.
pub(super) fn build_set<'a, Message>(
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: Option<f32>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    column(children)
        .spacing(spacing.unwrap_or(geometry::FIELD_SET_GAP_PX))
        .width(width)
        .into()
}

/// Builds the label/description content column used by horizontal fields.
pub(super) fn build_content<'a, Message>(
    children: Vec<Element<'a, Message>>,
    width: Length,
    spacing: Option<f32>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    column(children)
        .spacing(spacing.unwrap_or(geometry::FIELD_CONTENT_GAP_PX))
        .width(width)
        .into()
}

/// Text metrics for one of the field family text slots.
#[derive(Clone, Copy, Debug)]
pub(super) struct TextOptions {
    pub(super) size: f32,
    pub(super) line_height: f32,
    pub(super) color: crate::iced_compat::Color,
    pub(super) weight: FontWeight,
    pub(super) font_heading: bool,
    pub(super) width: Length,
}

/// Builds a field text slot from either a text fragment or arbitrary content.
pub(super) fn build_text<'a, Message>(
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    options: TextOptions,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let font_face = if options.font_heading {
        theme.font_pack().heading
    } else {
        theme.font_pack().sans
    };
    let mut font = iced_font(font_face);
    font.weight = iced_font_weight(options.weight);

    match content {
        FieldTextContent::Text(fragment) => iced_text(fragment)
            .size(options.size)
            .line_height(LineHeight::Absolute(options.line_height.into()))
            .font(font)
            .color(options.color)
            .width(options.width)
            .into(),
        FieldTextContent::Element(element) => container(element)
            .width(options.width)
            .style(move |_| container::Style {
                text_color: Some(options.color),
                ..container::Style::default()
            })
            .into(),
    }
}

/// Builds a field legend using the two variants from shadcn-svelte.
pub(super) fn build_legend<'a, Message>(
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    variant: FieldLegendVariant,
    width: Length,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let (size, line_height) = match variant {
        FieldLegendVariant::Legend => (16.0, 24.0),
        FieldLegendVariant::Label => (14.0, geometry::TITLE_LINE_HEIGHT_PX),
    };

    build_text(
        content,
        theme,
        TextOptions {
            size,
            line_height,
            color: color.unwrap_or(theme.palette.foreground),
            weight: FontWeight::Medium,
            font_heading: false,
            width,
        },
    )
}

/// Builds a field title.
pub(super) fn build_title<'a, Message>(
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    width: Length,
    disabled: bool,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut color = color.unwrap_or(theme.palette.foreground);
    if disabled {
        color = with_alpha(color, 0.5);
    }

    build_text(
        content,
        theme,
        TextOptions {
            size: 14.0,
            line_height: geometry::TITLE_LINE_HEIGHT_PX,
            color,
            weight: FontWeight::Medium,
            font_heading: false,
            width,
        },
    )
}

/// Builds muted helper text.
pub(super) fn build_description<'a, Message>(
    content: FieldTextContent<'a, Message>,
    theme: &'a Theme,
    width: Length,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    build_text(
        content,
        theme,
        TextOptions {
            size: 14.0,
            line_height: 21.0,
            color: color.unwrap_or(theme.palette.muted_foreground),
            weight: FontWeight::Normal,
            font_heading: false,
            width,
        },
    )
}

/// Builds a single error, an error list, or an empty zero-sized element.
pub(super) fn build_error<'a, Message>(
    content: Option<FieldTextContent<'a, Message>>,
    errors: Vec<super::types::FieldErrorItem>,
    theme: &'a Theme,
    width: Length,
) -> Element<'a, Message>
where
    Message: 'a,
{
    if let Some(content) = content {
        return build_text(
            content,
            theme,
            TextOptions {
                size: 14.0,
                line_height: 20.0,
                color: theme.palette.destructive,
                weight: FontWeight::Normal,
                font_heading: false,
                width,
            },
        );
    }

    let messages: Vec<String> = errors
        .iter()
        .filter_map(super::types::FieldErrorItem::message)
        .filter(|message| !message.is_empty())
        .map(str::to_owned)
        .collect();

    match messages.as_slice() {
        [] => widget::Space::new().into(),
        [message] => build_text(
            FieldTextContent::Text(message.clone().into()),
            theme,
            TextOptions {
                size: 14.0,
                line_height: 20.0,
                color: theme.palette.destructive,
                weight: FontWeight::Normal,
                font_heading: false,
                width,
            },
        ),
        _ => {
            let items = messages.into_iter().map(|message| {
                row![
                    iced_text("•")
                        .size(14.0)
                        .line_height(LineHeight::Absolute(20.0.into()))
                        .font(iced_font(theme.font_pack().sans))
                        .color(theme.palette.destructive)
                        .width(8.0),
                    iced_text(message)
                        .size(14.0)
                        .line_height(LineHeight::Absolute(20.0.into()))
                        .font(iced_font(theme.font_pack().sans))
                        .color(theme.palette.destructive)
                        .width(Length::Fill),
                ]
                .spacing(4)
                .align_y(Vertical::Top)
                .width(Length::Fill)
                .into()
            });

            container(column(items).spacing(4).width(Length::Fill))
                .padding(Padding {
                    left: 16.0,
                    ..Padding::ZERO
                })
                .width(width)
                .into()
        }
    }
}

/// Builds the optional centered separator content.
pub(super) fn build_separator<'a, Message>(
    content: Option<FieldTextContent<'a, Message>>,
    theme: &'a Theme,
    width: Length,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let line = || {
        container(crate::components::separator::separator(
            crate::components::separator::Separator::new(theme),
        ))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .into()
    };

    let Some(content) = content else {
        return line();
    };

    let content = build_text(
        content,
        theme,
        TextOptions {
            size: 14.0,
            line_height: 20.0,
            color: theme.palette.muted_foreground,
            weight: FontWeight::Normal,
            font_heading: false,
            width: Length::Shrink,
        },
    );

    container(
        row![
            line(),
            container(content).padding(Padding {
                left: 8.0,
                right: 8.0,
                ..Padding::ZERO
            }),
            line()
        ]
        .spacing(0)
        .align_y(Vertical::Center)
        .width(width),
    )
    .height(Length::Fixed(20.0))
    .width(width)
    .align_y(Vertical::Center)
    .into()
}

#[derive(Clone, Copy, Debug)]
enum FieldAxis {
    Vertical,
    Horizontal,
}

struct FieldWidget<'a, Message> {
    children: Vec<Element<'a, Message>>,
    orientation: FieldOrientation,
    width: Length,
    spacing: f32,
    responsive_breakpoint: f32,
    active_axis: Cell<FieldAxis>,
}

impl<Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for FieldWidget<'_, Message>
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let axis = self.axis_for_width(limits.max().width);
        self.active_axis.set(axis);

        layout::flex::resolve(
            match axis {
                FieldAxis::Vertical => layout::flex::Axis::Vertical,
                FieldAxis::Horizontal => layout::flex::Axis::Horizontal,
            },
            renderer,
            limits,
            self.width,
            Length::Shrink,
            Padding::ZERO,
            self.spacing,
            match axis {
                FieldAxis::Vertical => Alignment::Start,
                FieldAxis::Horizontal => Alignment::Center,
            },
            &mut self.children,
            &mut tree.children,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((child, state), child_layout) in self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
            }
        });
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
        for ((child, state), child_layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                state,
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

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), child_layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, child_layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
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
        if !layout.bounds().intersects(viewport) {
            return;
        }

        for ((child, state), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            if child_layout.bounds().intersects(viewport) {
                child.as_widget().draw(
                    state,
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }
        }
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
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message> From<FieldWidget<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(field: FieldWidget<'a, Message>) -> Self {
        Element::new(field)
    }
}

impl<Message> FieldWidget<'_, Message> {
    fn axis_for_width(&self, width: f32) -> FieldAxis {
        match self.orientation {
            FieldOrientation::Vertical => FieldAxis::Vertical,
            FieldOrientation::Horizontal => FieldAxis::Horizontal,
            FieldOrientation::Responsive => {
                if width >= self.responsive_breakpoint {
                    FieldAxis::Horizontal
                } else {
                    FieldAxis::Vertical
                }
            }
        }
    }
}

fn with_alpha(color: crate::iced_compat::Color, alpha: f32) -> crate::iced_compat::Color {
    crate::iced_compat::Color {
        a: (color.a * alpha).clamp(0.0, 1.0),
        ..color
    }
}
