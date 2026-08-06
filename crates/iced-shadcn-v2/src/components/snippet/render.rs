//! Rendering for the Snippet component.
//!
//! Mirrors the reference Svelte component: a padded `<pre>` frame with an
//! absolutely positioned copy button (`absolute top-1/2 right-2`). A custom
//! [`SnippetView`] widget layers the text rows and the copy button, paints the
//! variant surface (`bg-card` / `bg-accent` / `bg-destructive` / `bg-primary`)
//! and clips rows that overflow the frame.

use std::time::Duration;

use crate::components::copy_button::CopyButtonIcon;
use crate::components::copy_button::CopyButtonStatus;
use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout;
use crate::iced_compat::advanced::renderer;
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::Operation;
use crate::iced_compat::advanced::widget::tree::{self, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget};
use crate::iced_compat::widget::button::{self, Status as ButtonStatus};
use crate::iced_compat::widget::text::{LineHeight, Wrapping};
use crate::iced_compat::widget::{column, container, text as iced_text};
use crate::iced_compat::{
    Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Size, mouse,
};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

use shadcn_common::ComponentRadius;

use super::types::{SnippetRadius, SnippetText, SnippetVariant};

// ---------------------------------------------------------------------------
// Frame style
// ---------------------------------------------------------------------------

/// Resolved colors of the snippet frame for one variant.
#[derive(Debug, Clone, Copy)]
pub(super) struct SnippetFrame {
    /// `bg-*` of the variant.
    pub(super) background: Color,
    /// `border-*` of the variant (1 px).
    pub(super) border_color: Color,
    /// Inherited text color of the variant.
    pub(super) text_color: Color,
    /// Corner radius in px (`rounded-md` by default).
    pub(super) radius: f32,
}

/// Maps a [`SnippetVariant`] to frame colors, mirroring the `tv()` variants:
///
/// * `default` -> `border-border bg-card` (text inherits `foreground`)
/// * `secondary` -> `border-border bg-accent`
/// * `destructive` -> `border-destructive bg-destructive`
/// * `primary` -> `border-primary bg-primary text-primary-foreground`
pub(super) fn frame_style(theme: &Theme, variant: SnippetVariant, radius: f32) -> SnippetFrame {
    let palette = &theme.palette;
    let base = |background, border_color| SnippetFrame {
        background,
        border_color,
        text_color: palette.foreground,
        radius,
    };

    match variant {
        SnippetVariant::Default => base(palette.card, palette.border),
        SnippetVariant::Secondary => base(palette.accent, palette.border),
        SnippetVariant::Destructive => base(palette.destructive, palette.destructive),
        SnippetVariant::Primary => SnippetFrame {
            background: palette.primary,
            border_color: palette.primary,
            text_color: palette.primary_foreground,
            radius,
        },
    }
}

/// Resolves a [`SnippetRadius`] against the style pack.
pub(super) fn radius_px(theme: &Theme, radius: SnippetRadius) -> f32 {
    let recipe = theme.style.snippet();
    match radius {
        SnippetRadius::None => 0.0,
        SnippetRadius::Small => component_radius_px(theme, ComponentRadius::Sm),
        SnippetRadius::Medium => component_radius_px(theme, recipe.default_radius),
        SnippetRadius::Large => component_radius_px(theme, ComponentRadius::Lg),
        SnippetRadius::Full => 9999.0,
    }
}

// ---------------------------------------------------------------------------
// Content rows
// ---------------------------------------------------------------------------

/// Builds one iced `text` row per displayed line (`whitespace-nowrap`,
/// `text-left`, mono `text-sm font-light`), forcing the `leading-5` line box
/// like the code component does to dodge iced's 2-run paragraph measure.
pub(super) fn build_rows<'a, Message: 'a>(
    text: &SnippetText,
    theme: &'a Theme,
    text_color: Color,
) -> Element<'a, Message> {
    let recipe = theme.style.snippet();
    let mut font = iced_font(theme.style.font_pack.mono);
    font.weight = crate::recipes::iced_font_weight(recipe.typography.weight);

    let rows: Vec<Element<'a, Message>> = text
        .lines()
        .into_iter()
        .map(|line| {
            iced_text(line.to_owned())
                .font(font)
                .size(recipe.typography.size_px)
                .line_height(LineHeight::Absolute(
                    recipe.typography.line_height_px.into(),
                ))
                .height(Length::Fixed(recipe.typography.line_height_px))
                .wrapping(Wrapping::None)
                .color(text_color)
                .into()
        })
        .collect();

    column(rows).width(Length::Fill).into()
}

// ---------------------------------------------------------------------------
// Copy button
// ---------------------------------------------------------------------------

/// Builds the floating copy button (`size-7`, ghost, transparent hover with a
/// faded icon, matching the reference `hover:text-opacity-80
/// hover:bg-transparent` overrides).
///
/// When `on_copy` is `None` the button stays visually active but does not
/// publish anything — the same contract as a [`super::super::Button`] without
/// `on_press`, minus the disabled look.
pub(super) fn build_copy_button<'a, Message: 'a + Clone>(
    theme: &'a Theme,
    status: CopyButtonStatus,
    animation_duration: Duration,
    on_copy: Option<Message>,
    text_color: Color,
) -> Element<'a, Message> {
    let recipe = theme.style.snippet();
    let size = recipe.copy_button_px;
    let icon_size = recipe.copy_icon_px;
    let radius = component_radius_px(theme, recipe.default_radius);

    let icon: Element<'a, Message> = CopyButtonIcon {
        status,
        color: text_color,
        hover_color: Color {
            a: text_color.a * 0.8,
            ..text_color
        },
        size: icon_size,
        animation_duration,
    }
    .element();

    let content: Element<'a, Message> = container(icon)
        .width(Length::Fixed(icon_size))
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();

    let mut widget = button::Button::new(content)
        .padding(Padding::ZERO)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size));

    if let Some(message) = on_copy {
        widget = widget.on_press(message);
    }

    widget
        .style(move |_iced_theme, status| button::Style {
            background: None,
            text_color: match status {
                ButtonStatus::Hovered | ButtonStatus::Pressed => Color {
                    a: text_color.a * 0.8,
                    ..text_color
                },
                _ => text_color,
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: radius.into(),
            },
            ..Default::default()
        })
        .into()
}

// ---------------------------------------------------------------------------
// SnippetView custom widget
// ---------------------------------------------------------------------------

/// Index of the text rows child of [`SnippetView`].
const CONTENT: usize = 0;
/// Index of the copy button child of [`SnippetView`].
const COPY: usize = 1;
/// Fixed number of children of [`SnippetView`].
const CHILD_COUNT: usize = 2;

/// The widget behind the [`Snippet`](super::Snippet) builder.
///
/// Children:
///
/// 0. the text rows (a plain column, or a [`ScrollArea`](crate::ScrollArea)
///    when the caller bound the height — `overflow-y-auto` is opt-in, exactly
///    like the web component),
/// 1. the copy button overlay, centered on the right edge (`top-1/2 right-2`).
pub(super) struct SnippetView<'a, Message> {
    pub(super) content: Element<'a, Message>,
    pub(super) copy: Element<'a, Message>,
    pub(super) copy_active: bool,
    pub(super) frame: SnippetFrame,
    pub(super) width: Length,
    pub(super) height: Option<Length>,
    pub(super) max_width: Option<f32>,
    /// `right-2` inset of the copy button from the frame edge.
    pub(super) copy_offset: f32,
}

impl<'a, Message> SnippetView<'a, Message> {
    fn child_mut(&mut self, index: usize) -> &mut Element<'a, Message> {
        match index {
            CONTENT => &mut self.content,
            COPY => &mut self.copy,
            _ => unreachable!("SnippetView has exactly {CHILD_COUNT} children"),
        }
    }
}

impl<'a, Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for SnippetView<'a, Message>
{
    fn children(&self) -> Vec<Tree> {
        [&self.content, &self.copy]
            .into_iter()
            .map(|child| Tree::new(child.as_widget()))
            .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let children: Vec<_> = [&self.content, &self.copy]
            .into_iter()
            .map(Element::as_widget)
            .collect();

        tree.diff_children(&children);
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<()>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height.unwrap_or(Length::Shrink),
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &crate::iced_compat::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // `max-w-[...]` from the `class` prop: cap the frame width.
        let limits = match self.max_width {
            Some(max_width) => layout::Limits::new(
                limits.min(),
                Size::new(max_width.min(limits.max().width), limits.max().height),
            ),
            None => *limits,
        };

        // Bounded height opts into vertical scrolling (`overflow-y-auto`);
        // otherwise the frame grows with its rows.
        let height = self.height.unwrap_or(Length::Shrink);
        let mut size = limits.resolve(self.width, height, Size::new(limits.max().width, 0.0));

        let content_max = match self.height {
            Some(_) => size,
            None => Size::new(size.width, f32::INFINITY),
        };
        let content = self.content.as_widget_mut().layout(
            &mut tree.children[CONTENT],
            renderer,
            &layout::Limits::new(Size::ZERO, content_max),
        );

        if self.height.is_none() {
            size = limits.resolve(self.width, Length::Shrink, content.size());
        }

        let copy = self.copy.as_widget_mut().layout(
            &mut tree.children[COPY],
            renderer,
            &layout::Limits::new(Size::ZERO, size),
        );
        let copy_size = copy.bounds().size();

        // `absolute top-1/2 right-2 -translate-y-1/2`.
        let copy = copy.move_to(Point::new(
            size.width - self.copy_offset - copy_size.width,
            ((size.height - copy_size.height) * 0.5).max(0.0),
        ));

        layout::Node::with_children(size, vec![content, copy])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        for index in 0..CHILD_COUNT {
            self.child_mut(index).as_widget_mut().operate(
                &mut tree.children[index],
                layout.children().nth(index).unwrap_or_else(|| {
                    unreachable!("SnippetView always lays out {CHILD_COUNT} children")
                }),
                renderer,
                operation,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &crate::iced_compat::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // Copy button first: it sits above the rows (`absolute`).
        for index in (0..CHILD_COUNT).rev() {
            self.child_mut(index).as_widget_mut().update(
                &mut tree.children[index],
                event,
                layout.children().nth(index).unwrap_or_else(|| {
                    unreachable!("SnippetView always lays out {CHILD_COUNT} children")
                }),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            if shell.is_event_captured() {
                return;
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut crate::iced_compat::Renderer,
        theme: &crate::iced_compat::Theme,
        style: &renderer::Style,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let Some(clipped) = layout.bounds().intersection(viewport) else {
            return;
        };

        // Frame surface (`bg-*` + 1 px `border-*` + `rounded-md`).
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: self.frame.border_color,
                    width: 1.0,
                    radius: self.frame.radius.into(),
                },
                ..Default::default()
            },
            Background::Color(self.frame.background),
        );

        let children = [&self.content, &self.copy];
        let active = [true, self.copy_active];

        for index in 0..CHILD_COUNT {
            if !active[index] {
                continue;
            }
            let child_layout = layout.children().nth(index).unwrap_or_else(|| {
                unreachable!("SnippetView always lays out {CHILD_COUNT} children")
            });
            renderer.with_layer(clipped, |renderer| {
                children[index].as_widget().draw(
                    &tree.children[index],
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    &clipped,
                );
            });
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &crate::iced_compat::Renderer,
    ) -> mouse::Interaction {
        let children = [&self.content, &self.copy];
        let active = [true, self.copy_active];

        for index in (0..CHILD_COUNT).rev() {
            if !active[index] {
                continue;
            }
            let interaction = children[index].as_widget().mouse_interaction(
                &tree.children[index],
                layout.children().nth(index).unwrap_or_else(|| {
                    unreachable!("SnippetView always lays out {CHILD_COUNT} children")
                }),
                cursor,
                viewport,
                renderer,
            );
            if interaction != mouse::Interaction::default() {
                return interaction;
            }
        }

        mouse::Interaction::default()
    }
}

impl<'a, Message> From<SnippetView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(view: SnippetView<'a, Message>) -> Self {
        Self::new(view)
    }
}
