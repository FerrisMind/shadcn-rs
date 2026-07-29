//! Layout assembly, corner flattening, and border merging for the
//! button-group component.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::widget::{Column, Row, container, text as iced_text};
use crate::iced_compat::{Border, Element, Length};

use shadcn_common::ControlSize;

use super::types::ButtonGroupOrientation;
use super::{ButtonGroup, ButtonGroupItem, ButtonGroupText, ItemKind, TextContent};
use crate::components::button::{self, CornerFlatten};
use crate::components::separator::{Separator, SeparatorOrientation, separator};
use crate::fonts::iced_font;
use crate::theme::Theme;

/// `gap-2` between children of a group that nests other groups.
pub(super) const DEFAULT_NESTED_GAP: f32 = 8.0;

/// Border width shared by every mergeable child (buttons, text cells, and
/// typical bordered widgets all paint 1 px borders).
const MERGED_BORDER_WIDTH: f32 = 1.0;

/// Default horizontal padding of a text cell (`px-4`).
const TEXT_PADDING_X: f32 = 16.0;

pub(super) fn build_group<'a, Message>(group: ButtonGroup<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let ButtonGroup {
        theme: _,
        orientation,
        items,
        width,
        height,
        nested_gap,
        aria_label: _,
    } = group;

    let has_nested = items
        .iter()
        .any(|item| matches!(item.kind, ItemKind::Group(_)));

    if has_nested {
        return assemble(
            orientation,
            items
                .into_iter()
                .map(|item| build_standalone_item(item, orientation))
                .collect(),
            nested_gap,
            width,
            height,
        );
    }

    build_merged(orientation, items, width, height)
}

/// Child rendered on its own (nesting mode): full rounding, no merging.
fn build_standalone_item<'a, Message>(
    item: ButtonGroupItem<'a, Message>,
    orientation: ButtonGroupOrientation,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    match item.kind {
        ItemKind::Button(button) => (*button).into(),
        ItemKind::Text(text) => build_text(*text, CornerFlatten::default(), sizing_standalone()),
        ItemKind::Separator(rule) => build_separator(rule, orientation),
        ItemKind::Element(element) => element,
        ItemKind::Group(group) => build_group(*group),
    }
}

/// Children merged into one visual control (no nested groups present).
fn build_merged<'a, Message>(
    orientation: ButtonGroupOrientation,
    items: Vec<ButtonGroupItem<'a, Message>>,
    width: Length,
    height: Length,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let count = items.len();
    let stretch = orientation == ButtonGroupOrientation::Vertical && width != Length::Shrink;
    let has_rigid = items
        .iter()
        .any(|item| matches!(item.kind, ItemKind::Button(_) | ItemKind::Element(_)));

    let joints: Vec<f32> = (1..count)
        .map(|index| -joint_overlap(&items[index - 1], &items[index]))
        .collect();

    let elements: Vec<Element<'a, Message>> = items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let corners = corner_mask(orientation, index == 0, index + 1 == count);
            build_merged_item(item, orientation, corners, stretch, has_rigid)
        })
        .collect();

    let uniform = joints.windows(2).all(|pair| pair[0] == pair[1]);

    if uniform {
        let spacing = joints.first().copied().unwrap_or(0.0);
        return assemble(orientation, elements, spacing, width, height);
    }

    // Per-joint spacing differs (e.g. bordered and borderless children are
    // mixed with separators): fold children pairwise so every joint gets its
    // exact overlap.
    let mut children = elements.into_iter();
    let Some(mut chain) = children.next() else {
        return assemble(orientation, Vec::new(), 0.0, width, height);
    };

    for (element, joint) in children.zip(joints) {
        chain = assemble_axis(orientation, vec![chain, element], joint);
    }

    container(chain).width(width).height(height).into()
}

fn build_merged_item<'a, Message>(
    item: ButtonGroupItem<'a, Message>,
    orientation: ButtonGroupOrientation,
    corners: CornerFlatten,
    stretch: bool,
    has_rigid: bool,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    match item.kind {
        ItemKind::Button(button) => {
            let mut button = (*button).flatten_corners(corners);
            if stretch {
                button = button.full_width();
            }
            button.into()
        }
        ItemKind::Text(text) => build_text(
            *text,
            corners,
            sizing_merged(orientation, stretch, has_rigid),
        ),
        ItemKind::Separator(rule) => build_separator(rule, orientation),
        ItemKind::Element(element) => element,
        // Unreachable by construction (`build_group` routes nesting groups
        // through the standalone path), but stays total instead of panicking.
        ItemKind::Group(group) => build_group(*group),
    }
}

/// Overlap in px between two adjacent children.
///
/// Mirrors the web `border-l-0` / `border-t-0` rules: a child that paints a
/// resting border is pulled over the previous child's trailing border so the
/// pair shares one 1 px divider. Separators keep their own 1 px column and
/// never overlap.
fn joint_overlap<Message>(
    previous: &ButtonGroupItem<'_, Message>,
    current: &ButtonGroupItem<'_, Message>,
) -> f32 {
    if matches!(previous.kind, ItemKind::Separator(_)) {
        return 0.0;
    }

    let bordered = match &current.kind {
        ItemKind::Button(button) => button.has_resting_border(),
        // Text cells always carry a 1 px border; opaque elements (inputs,
        // pick lists, …) are assumed bordered, matching the web rule that
        // strips their leading border inside a group.
        ItemKind::Text(_) | ItemKind::Element(_) => true,
        ItemKind::Separator(_) | ItemKind::Group(_) => false,
    };

    if bordered { MERGED_BORDER_WIDTH } else { 0.0 }
}

/// Corners flattened for a child at the given position.
pub(super) fn corner_mask(
    orientation: ButtonGroupOrientation,
    first: bool,
    last: bool,
) -> CornerFlatten {
    match orientation {
        ButtonGroupOrientation::Horizontal => CornerFlatten {
            top_left: !first,
            bottom_left: !first,
            top_right: !last,
            bottom_right: !last,
        },
        ButtonGroupOrientation::Vertical => CornerFlatten {
            top_left: !first,
            top_right: !first,
            bottom_left: !last,
            bottom_right: !last,
        },
    }
}

/// Orientation of a separator inside a group: always the cross axis.
pub(super) const fn cross_orientation(orientation: ButtonGroupOrientation) -> SeparatorOrientation {
    match orientation {
        ButtonGroupOrientation::Horizontal => SeparatorOrientation::Vertical,
        ButtonGroupOrientation::Vertical => SeparatorOrientation::Horizontal,
    }
}

fn build_separator<'a, Message: 'a>(
    rule: Separator,
    orientation: ButtonGroupOrientation,
) -> Element<'a, Message> {
    separator(rule.orientation(cross_orientation(orientation))).into()
}

fn assemble<'a, Message: 'a>(
    orientation: ButtonGroupOrientation,
    elements: Vec<Element<'a, Message>>,
    spacing: f32,
    width: Length,
    height: Length,
) -> Element<'a, Message> {
    match orientation {
        ButtonGroupOrientation::Horizontal => Row::with_children(elements)
            .spacing(spacing)
            .align_y(Vertical::Center)
            .width(width)
            .height(height)
            .into(),
        ButtonGroupOrientation::Vertical => Column::with_children(elements)
            .spacing(spacing)
            .align_x(Horizontal::Left)
            .width(width)
            .height(height)
            .into(),
    }
}

fn assemble_axis<'a, Message: 'a>(
    orientation: ButtonGroupOrientation,
    elements: Vec<Element<'a, Message>>,
    spacing: f32,
) -> Element<'a, Message> {
    match orientation {
        ButtonGroupOrientation::Horizontal => Row::with_children(elements)
            .spacing(spacing)
            .align_y(Vertical::Center)
            .into(),
        ButtonGroupOrientation::Vertical => Column::with_children(elements)
            .spacing(spacing)
            .align_x(Horizontal::Left)
            .into(),
    }
}

/// Footprint of a text cell for its layout context.
struct TextSizing {
    width: Length,
    height: Length,
}

/// Standalone text cells mimic a default-size button footprint.
fn sizing_standalone() -> TextSizing {
    TextSizing {
        width: Length::Shrink,
        height: Length::Shrink,
    }
}

fn sizing_merged(
    orientation: ButtonGroupOrientation,
    stretch: bool,
    has_rigid: bool,
) -> TextSizing {
    match orientation {
        // `items-stretch`: fill the row height set by sibling controls.
        // Without a rigid sibling the fill would collapse, so fall back to
        // the intrinsic control height.
        ButtonGroupOrientation::Horizontal => TextSizing {
            width: Length::Shrink,
            height: if has_rigid {
                Length::Fill
            } else {
                Length::Shrink
            },
        },
        ButtonGroupOrientation::Vertical => TextSizing {
            width: if stretch {
                Length::Fill
            } else {
                Length::Shrink
            },
            height: Length::Shrink,
        },
    }
}

pub(super) fn build_standalone_text<'a, Message: 'a>(
    text: ButtonGroupText<'a, Message>,
) -> Element<'a, Message> {
    build_text(text, CornerFlatten::default(), sizing_standalone())
}

fn build_text<'a, Message: 'a>(
    text: ButtonGroupText<'a, Message>,
    corners: CornerFlatten,
    sizing: TextSizing,
) -> Element<'a, Message> {
    let ButtonGroupText {
        content,
        theme,
        padding_x,
        text_size,
        style_override,
    } = text;

    let recipe = theme.style.button_size(ControlSize::Md);
    let text_size = text_size.unwrap_or(recipe.text_size_px);
    let padding_x = padding_x.unwrap_or(TEXT_PADDING_X);
    let control_height = recipe.height_px;

    let body: Element<'a, Message> = match content {
        TextContent::Label(label) => {
            let type_recipe = theme.style.button_type();
            let mut font = iced_font(theme.font_pack().sans);
            font.weight = crate::recipes::iced_font_weight(type_recipe.typography.weight);

            iced_text(label.into_owned())
                .size(text_size)
                .font(font)
                .line_height(LineHeight::Absolute(text_size.into()))
                .into()
        }
        TextContent::Element(element) => element,
    };

    let height = match sizing.height {
        Length::Shrink => Length::Fixed(control_height),
        other => other,
    };

    let radius = text_radius(theme, corners);

    container(body)
        .padding(crate::iced_compat::Padding {
            top: 0.0,
            right: padding_x,
            bottom: 0.0,
            left: padding_x,
        })
        .width(sizing.width)
        .height(height)
        .align_y(Vertical::Center)
        .style(move |_iced_theme| {
            let mut resolved = container::Style {
                background: Some(theme.palette.muted.into()),
                text_color: Some(theme.palette.foreground),
                border: Border {
                    color: theme.palette.border,
                    width: MERGED_BORDER_WIDTH,
                    radius,
                },
                ..container::Style::default()
            };

            if let Some(override_fn) = style_override.as_ref() {
                resolved = override_fn(resolved);
            }

            resolved
        })
        .into()
}

fn text_radius(theme: &Theme, corners: CornerFlatten) -> crate::iced_compat::border::Radius {
    let base = button::default_radius_px(theme);

    crate::iced_compat::border::Radius {
        top_left: if corners.top_left { 0.0 } else { base },
        top_right: if corners.top_right { 0.0 } else { base },
        bottom_right: if corners.bottom_right { 0.0 } else { base },
        bottom_left: if corners.bottom_left { 0.0 } else { base },
    }
}
