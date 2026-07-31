//! Rendering for the Code component.
//!
//! Converts `shadcn_common` syntax-highlighting tokens into a per-line row of
//! iced `text` widgets (one fragment per token) and hosts a custom
//! [`CodeView`] widget that layers the code block, the overflow fade + expand
//! button, and the copy button — matching the `pre.shiki` + `CodeOverflow` +
//! `CodeCopyButton` markup of the reference Svelte component.

use crate::fonts::iced_font;
use crate::iced_compat::advanced::layout;
use crate::iced_compat::advanced::renderer;
use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::Operation;
use crate::iced_compat::advanced::widget::tree::{self, Tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget};
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::LineHeight;
use crate::iced_compat::{
    Background, Color, Element, Event, Font, Length, Padding, Pixels, Radians, Rectangle, Size,
    mouse, touch,
};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

use shadcn_common::{CodePalette, CodeToken, SyntaxKind};

use super::types::CodeVariant;

// ---------------------------------------------------------------------------
// Constants that mirror the Tailwind classes used by the Svelte component
// ---------------------------------------------------------------------------

/// `text-sm` line-height = 1.25 rem = 20 px; each `.line` box is 20 px tall.
pub(super) const LINE_TEXT_HEIGHT: f32 = 20.0;

/// `py-0.5` = 0.125 rem = 2 px on each side of a line.
pub(super) const LINE_VERTICAL_PAD: f32 = 2.0;

/// Total height of one rendered line: 20 px line box + 4 px padding.
pub(super) const LINE_HEIGHT: f32 = LINE_TEXT_HEIGHT + 2.0 * LINE_VERTICAL_PAD;

/// `px-4` when no line numbers, `px-2` when line numbers are shown.
pub(super) const LINE_PAD_WITH_NUMBERS: f32 = 8.0;
pub(super) const LINE_PAD_WITHOUT_NUMBERS: f32 = 16.0;

/// `width: 1.8rem` for the line-number pseudo-element.
pub(super) const LINE_NUMBER_WIDTH: f32 = 28.8;

/// `margin-right: 1.4rem` gap after the line-number pseudo-element.
pub(super) const LINE_NUMBER_GAP: f32 = 22.4;

/// Default code font size (`text-sm` = 0.875 rem = 14 px).
pub(super) const CODE_FONT_SIZE: f32 = 14.0;

/// `py-4` = 1 rem = 16 px vertical padding on the `<pre>` block.
pub(super) const PRE_VERTICAL_PAD: f32 = 16.0;

/// `max-h-[300px]` for the collapsed overflow state.
pub(super) const OVERFLOW_MAX_HEIGHT: f32 = 300.0;

/// `bottom-2` offset of the expand button.
pub(super) const EXPAND_BOTTOM_OFFSET: f32 = 8.0;

/// `top-2 right-2` offset of the copy button.
pub(super) const COPY_OFFSET: f32 = 8.0;

// ---------------------------------------------------------------------------
// Color helpers
// ---------------------------------------------------------------------------

/// Converts a `shadcn_common` `Rgba` (channels in 0..255, alpha in 0..1) to an
/// iced `Color` (all channels in 0..1).
pub(super) fn rgba_to_iced(rgba: shadcn_common::color_space::Rgba) -> Color {
    Color::from_rgba(
        rgba.red / 255.0,
        rgba.green / 255.0,
        rgba.blue / 255.0,
        rgba.alpha,
    )
}

/// Returns the monospace iced `Font` for the current theme.
pub(super) fn mono_font(theme: &Theme) -> Font {
    iced_font(theme.style.font_pack.mono)
}

/// `text-sm` / `leading-5` absolute line box (20 px at the 16 px root).
pub(super) fn line_height() -> LineHeight {
    LineHeight::Absolute(Pixels(LINE_TEXT_HEIGHT))
}

// ---------------------------------------------------------------------------
// Token -> colored text fragments
// ---------------------------------------------------------------------------

/// Maps a [`SyntaxKind`] to a [`Color`] from the given [`CodePalette`].
fn token_color(kind: SyntaxKind, palette: &CodePalette) -> Color {
    let (rgba, _bg) = palette.token_color(kind);
    rgba_to_iced(rgba)
}

/// Builds the highlighted code for one source line as a horizontal row of
/// plain `text` widgets.
///
/// Each fragment is forced to the `text-sm` / `leading-5` line box. iced's
/// paragraph measurer was reporting two layout runs (~40 px) per source line,
/// which made rows look double-spaced and caused `clip` to hide the glyphs.
fn build_code_fragments<'a, Message: 'a>(
    tokens: &[CodeToken],
    source: &'a str,
    line_start: usize,
    line_end: usize,
    palette: &CodePalette,
    font: Font,
) -> Element<'a, Message> {
    use crate::iced_compat::widget::text::Wrapping;
    use crate::iced_compat::widget::{Space, row, text as iced_text};

    let parts: Vec<Element<'a, Message>> = tokens
        .iter()
        .filter(|token| token.start < line_end && token.end > line_start)
        .filter_map(|token| {
            let start = token.start.max(line_start);
            let end = token.end.min(line_end);
            let text = source[start..end].trim_end_matches(['\r', '\n']);
            if text.is_empty() {
                return None;
            }
            Some(
                iced_text(text)
                    .font(font)
                    .size(CODE_FONT_SIZE)
                    .line_height(line_height())
                    .height(Length::Fixed(LINE_TEXT_HEIGHT))
                    .wrapping(Wrapping::None)
                    .color(token_color(token.kind, palette))
                    .into(),
            )
        })
        .collect();

    if parts.is_empty() {
        Space::new()
            .width(Length::Shrink)
            .height(Length::Fixed(LINE_TEXT_HEIGHT))
            .into()
    } else {
        row(parts)
            .height(Length::Fixed(LINE_TEXT_HEIGHT))
            .align_y(crate::iced_compat::alignment::Vertical::Center)
            .into()
    }
}

// ---------------------------------------------------------------------------
// Line splitting
// ---------------------------------------------------------------------------

/// A single logical line of source code together with its 1-based number.
pub(super) struct SourceLine {
    /// 1-based line number (for display and for the `highlight` prop).
    pub number: u32,
    /// Byte offset of the first character of this line in the source.
    pub start: usize,
    /// Exclusive byte offset (one past the last character, usually `\n`).
    pub end: usize,
}

/// Splits `source` into lines, trimming the trailing newline of each line
/// (matching the Svelte component's `code.trimEnd()`).
pub(super) fn split_lines(source: &str) -> Vec<SourceLine> {
    let mut lines = Vec::new();
    let mut start = 0usize;
    let mut number = 1u32;

    for (idx, ch) in source.char_indices() {
        if ch == '\n' {
            // trimEnd: skip trailing \r
            let mut end = idx;
            if end > start && source.as_bytes()[end - 1] == b'\r' {
                end -= 1;
            }
            lines.push(SourceLine { number, start, end });
            start = idx + 1;
            number += 1;
        }
    }

    // Last line (may or may not end with a newline).
    if start < source.len() {
        let mut end = source.len();
        if end > start && source.as_bytes()[end - 1] == b'\r' {
            end -= 1;
        }
        lines.push(SourceLine { number, start, end });
    }

    // Empty source -> single empty line.
    if lines.is_empty() {
        lines.push(SourceLine {
            number: 1,
            start: 0,
            end: 0,
        });
    }

    lines
}

// ---------------------------------------------------------------------------
// Per-line widget
// ---------------------------------------------------------------------------

/// Builds the iced element for a single source line.
///
/// When `hide_lines` is `false`, a right-aligned line number is prepended.
/// When the line is highlighted, the container gets a secondary background.
#[allow(clippy::too_many_arguments)]
pub(super) fn build_line<'a, Message: 'a>(
    line: &SourceLine,
    tokens: &[CodeToken],
    source: &'a str,
    palette: &CodePalette,
    font: Font,
    hide_lines: bool,
    highlighted: bool,
    theme: &'a Theme,
) -> Element<'a, Message> {
    use crate::iced_compat::widget::{Space, row, text as iced_text};

    let pad_x = if hide_lines {
        LINE_PAD_WITHOUT_NUMBERS
    } else {
        LINE_PAD_WITH_NUMBERS
    };

    let code_part: Element<'a, Message> =
        build_code_fragments(tokens, source, line.start, line.end, palette, font);

    let inner: Element<'a, Message> = if hide_lines {
        code_part
    } else {
        let line_num_text = iced_text(format!("{:>4}", line.number))
            .font(font)
            .size(CODE_FONT_SIZE)
            .line_height(line_height())
            .height(Length::Fixed(LINE_TEXT_HEIGHT))
            .wrapping(crate::iced_compat::widget::text::Wrapping::None)
            .color(theme.palette.muted_foreground);

        let line_num_cell = container(line_num_text)
            .width(Length::Fixed(LINE_NUMBER_WIDTH))
            .height(Length::Fixed(LINE_TEXT_HEIGHT))
            .align_x(crate::iced_compat::alignment::Horizontal::Right)
            .align_y(crate::iced_compat::alignment::Vertical::Center);

        let gap = Space::new()
            .width(Length::Fixed(LINE_NUMBER_GAP))
            .height(Length::Fixed(LINE_TEXT_HEIGHT));

        row![line_num_cell, gap, code_part]
            .height(Length::Fixed(LINE_TEXT_HEIGHT))
            .align_y(crate::iced_compat::alignment::Vertical::Center)
            .into()
    };

    // `pre .line` → `inline-block min-h-4 w-full px-4 py-0.5` (and `px-2` with
    // line numbers). Height is forced so a buggy 2-run paragraph measure
    // cannot stretch the row; clip keeps overflow ink inside the line box.
    container(inner)
        .width(Length::Fill)
        .height(Length::Fixed(LINE_HEIGHT))
        .padding(
            Padding::default()
                .top(LINE_VERTICAL_PAD)
                .bottom(LINE_VERTICAL_PAD)
                .left(pad_x)
                .right(pad_x),
        )
        .align_y(crate::iced_compat::alignment::Vertical::Center)
        .clip(true)
        .style(move |_| {
            if highlighted {
                container::Style {
                    background: Some(Background::Color(theme.palette.secondary)),
                    ..Default::default()
                }
            } else {
                container::Style::default()
            }
        })
        .into()
}

// ---------------------------------------------------------------------------
// Outer container style
// ---------------------------------------------------------------------------

/// Styles for the root code block container, matching `codeVariants` from the
/// Svelte component:
///
/// * `default` -> `rounded-lg border-border bg-card`
/// * `secondary` -> `bg-secondary/50 border-transparent`
///
/// The reference has no style-specific code tokens, so the frame radius
/// follows the embedded button recipe (copy / expand) via the code recipe.
pub(super) fn code_container_style<'a>(
    variant: CodeVariant,
    theme: &'a Theme,
) -> impl Fn(&iced_core::Theme) -> container::Style + 'a {
    move |_| {
        let palette = &theme.palette;
        let radius = component_radius_px(theme, theme.style.code().default_radius);
        match variant {
            CodeVariant::Default => container::Style {
                background: Some(Background::Color(palette.card)),
                text_color: Some(palette.card_foreground),
                border: crate::iced_compat::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: radius.into(),
                },
                ..Default::default()
            },
            CodeVariant::Secondary => container::Style {
                background: Some(Background::Color(Color {
                    a: 0.5,
                    ..palette.secondary
                })),
                text_color: Some(palette.secondary_foreground),
                border: crate::iced_compat::Border {
                    color: Color::TRANSPARENT,
                    width: 1.0,
                    radius: radius.into(),
                },
                ..Default::default()
            },
        }
    }
}

/// Gradient fade overlay for the overflow state.
///
/// Returns a `container::Style` with a bottom-to-top linear gradient that
/// fades from the theme background to transparent, matching the Svelte
/// component's `from-background bg-linear-to-t to-transparent`.
pub(super) fn fade_gradient_style<'a>(
    theme: &'a Theme,
) -> impl Fn(&iced_core::Theme) -> container::Style + 'a {
    let bg = theme.palette.background;

    let gradient = crate::iced_compat::gradient::Linear::new(Radians(0.0))
        .add_stop(0.0, bg)
        .add_stop(1.0, Color { a: 0.0, ..bg });

    move |_| container::Style {
        background: Some(Background::Gradient(
            crate::iced_compat::gradient::Gradient::Linear(gradient),
        )),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// CodeView custom widget
// ---------------------------------------------------------------------------

/// Index of the collapsed (clipped, non-scrolling) content child.
const CONTENT_COLLAPSED: usize = 0;
/// Index of the expanded content child (plain pre, or ScrollArea if height set).
const CONTENT_EXPANDED: usize = 1;
/// Index of the fade overlay child of [`CodeView`].
const FADE: usize = 2;
/// Index of the expand-button overlay child of [`CodeView`].
const EXPAND: usize = 3;
/// Index of the copy-button overlay child of [`CodeView`].
const COPY: usize = 4;
/// Fixed number of children of [`CodeView`].
const CHILD_COUNT: usize = 5;

/// Tree state of [`CodeView`]: the uncontrolled `collapsed` flag.
///
/// The flag is only read when no [`OverflowView::collapsed_override`] is set,
/// matching the `$bindable` semantics of the reference Svelte component.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct CodeViewState {
    pub(super) collapsed: bool,
}

/// Overflow decoration configuration, present only when `overflow` was
/// attached to the [`Code`](super::Code) builder.
pub(super) struct OverflowView<'a, Message> {
    pub(super) collapsed_override: Option<bool>,
    pub(super) default_collapsed: bool,
    pub(super) max_height: f32,
    pub(super) on_collapse_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

/// The widget behind the [`Code`](super::Code) builder.
///
/// Children (fixed count so each content tree keeps its own `Rich` caches):
///
/// 0. collapsed content — full pre, clipped with `overflow-y: hidden` (no scroll),
/// 1. expanded content — plain pre, or a [`ScrollArea`] when the caller set
///    an explicit height (scrolling is opt-in),
/// 2. bottom fade gradient (collapsed only),
/// 3. expand button overlay (collapsed only),
/// 4. copy button overlay (`absolute top-2 right-2`, drawn above the code).
pub(super) struct CodeView<'a, Message> {
    pub(super) content_collapsed: Element<'a, Message>,
    pub(super) content_expanded: Element<'a, Message>,
    pub(super) fade: Element<'a, Message>,
    pub(super) expand: Element<'a, Message>,
    pub(super) copy: Element<'a, Message>,
    pub(super) copy_active: bool,
    pub(super) overflow: Option<OverflowView<'a, Message>>,
    pub(super) content_height: f32,
    pub(super) width: Length,
    pub(super) height: Option<Length>,
}

impl<'a, Message> CodeView<'a, Message> {
    /// Returns the collapsed state to display this frame.
    fn effective_collapsed(&self, tree: &Tree) -> bool {
        match &self.overflow {
            Some(overflow) => overflow
                .collapsed_override
                .unwrap_or_else(|| tree.state.downcast_ref::<CodeViewState>().collapsed),
            None => false,
        }
    }

    /// Index of the content child that is currently visible.
    fn content_index(collapsed: bool) -> usize {
        if collapsed {
            CONTENT_COLLAPSED
        } else {
            CONTENT_EXPANDED
        }
    }

    /// Returns the child at `index`, mutably.
    fn child_mut(&mut self, index: usize) -> &mut Element<'a, Message> {
        match index {
            CONTENT_COLLAPSED => &mut self.content_collapsed,
            CONTENT_EXPANDED => &mut self.content_expanded,
            FADE => &mut self.fade,
            EXPAND => &mut self.expand,
            COPY => &mut self.copy,
            _ => unreachable!("CodeView has exactly {CHILD_COUNT} children"),
        }
    }

    /// Whether the fade overlay is visible (collapsed with overflow config).
    fn fade_active(&self, collapsed: bool) -> bool {
        self.overflow.is_some() && collapsed
    }

    /// Whether the expand button is visible and interactive.
    fn expand_active(&self, collapsed: bool) -> bool {
        self.overflow.is_some() && collapsed
    }

    /// Toggles the collapsed state, notifying `on_collapse_change` in both
    /// controlled and uncontrolled mode.
    fn toggle_collapsed(&self, tree: &mut Tree, shell: &mut Shell<'_, Message>, collapsed: bool) {
        let Some(overflow) = &self.overflow else {
            return;
        };

        let next = !collapsed;

        if overflow.collapsed_override.is_none() {
            tree.state.downcast_mut::<CodeViewState>().collapsed = next;
            shell.request_redraw();
        }

        if let Some(callback) = &overflow.on_collapse_change {
            shell.publish(callback(next));
        }
    }
}

impl<'a, Message> Widget<Message, crate::iced_compat::Theme, crate::iced_compat::Renderer>
    for CodeView<'a, Message>
{
    fn children(&self) -> Vec<Tree> {
        [
            &self.content_collapsed,
            &self.content_expanded,
            &self.fade,
            &self.expand,
            &self.copy,
        ]
        .into_iter()
        .map(|child| Tree::new(child.as_widget()))
        .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let children: Vec<_> = [
            &self.content_collapsed,
            &self.content_expanded,
            &self.fade,
            &self.expand,
            &self.copy,
        ]
        .into_iter()
        .map(Element::as_widget)
        .collect();

        tree.diff_children(&children);
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<CodeViewState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(CodeViewState {
            collapsed: self
                .overflow
                .as_ref()
                .is_some_and(|overflow| overflow.default_collapsed),
        })
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
        let collapsed = self.effective_collapsed(tree);

        // Collapsed overflow → fixed max-height (Svelte `max-h-[300px]`).
        // Explicit `.height(...)` → that viewport (optional ScrollArea).
        // Otherwise → shrink to content (no scroll).
        let height: Length = if collapsed {
            Length::Fixed(
                self.overflow
                    .as_ref()
                    .map_or(OVERFLOW_MAX_HEIGHT, |overflow| overflow.max_height),
            )
        } else if let Some(height) = self.height {
            height
        } else {
            Length::Shrink
        };

        let preferred = Size::new(limits.max().width, self.content_height);
        let mut size = limits.resolve(self.width, height, preferred);

        // Content is laid out at its natural height when collapsed (clipped at
        // draw time, like `overflow-y: hidden`) or when no explicit height is
        // set. Only an explicit height constrains the expanded content box
        // (ScrollArea fills that viewport).
        let content_max = if collapsed || self.height.is_none() {
            Size::new(size.width, f32::INFINITY)
        } else {
            size
        };
        let content_limits = layout::Limits::new(Size::ZERO, content_max);

        let content_collapsed = self.content_collapsed.as_widget_mut().layout(
            &mut tree.children[CONTENT_COLLAPSED],
            renderer,
            &content_limits,
        );
        let content_expanded = self.content_expanded.as_widget_mut().layout(
            &mut tree.children[CONTENT_EXPANDED],
            renderer,
            &content_limits,
        );

        if !collapsed && self.height.is_none() {
            size = limits.resolve(self.width, Length::Shrink, content_expanded.size());
        }

        let overlay_limits = layout::Limits::new(Size::ZERO, size);
        let fade =
            self.fade
                .as_widget_mut()
                .layout(&mut tree.children[FADE], renderer, &overlay_limits);
        let expand = self.expand.as_widget_mut().layout(
            &mut tree.children[EXPAND],
            renderer,
            &overlay_limits,
        );
        let copy =
            self.copy
                .as_widget_mut()
                .layout(&mut tree.children[COPY], renderer, &overlay_limits);

        layout::Node::with_children(
            size,
            vec![content_collapsed, content_expanded, fade, expand, copy],
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: crate::iced_compat::advanced::layout::Layout<'_>,
        renderer: &crate::iced_compat::Renderer,
        operation: &mut dyn Operation,
    ) {
        let collapsed = self.effective_collapsed(tree);
        let active_content = Self::content_index(collapsed);

        for index in 0..CHILD_COUNT {
            if (index == CONTENT_COLLAPSED || index == CONTENT_EXPANDED) && index != active_content
            {
                continue;
            }

            self.child_mut(index).as_widget_mut().operate(
                &mut tree.children[index],
                layout.children().nth(index).unwrap_or_else(|| {
                    unreachable!("CodeView always lays out {CHILD_COUNT} children")
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
        let collapsed = self.effective_collapsed(tree);
        let active_content = Self::content_index(collapsed);

        // Expand / copy first (they sit above the code).
        if self.expand_active(collapsed) {
            let press = matches!(
                event,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    | Event::Touch(touch::Event::FingerPressed { .. })
            );
            let button_bounds = layout
                .children()
                .nth(EXPAND)
                .and_then(|node| node.children().next())
                .map(|node| node.bounds());

            if press && button_bounds.is_some_and(|bounds| cursor.is_over(bounds)) {
                self.toggle_collapsed(tree, shell, collapsed);
                shell.capture_event();
                return;
            }
        }

        for index in (0..CHILD_COUNT).rev() {
            if (index == CONTENT_COLLAPSED || index == CONTENT_EXPANDED) && index != active_content
            {
                continue;
            }

            // Collapsed content outside the clipped viewport is not interactive.
            if index == active_content && collapsed && !cursor.is_over(layout.bounds()) {
                continue;
            }

            self.child_mut(index).as_widget_mut().update(
                &mut tree.children[index],
                event,
                layout.children().nth(index).unwrap_or_else(|| {
                    unreachable!("CodeView always lays out {CHILD_COUNT} children")
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
        let collapsed = self.effective_collapsed(tree);
        let active_content = Self::content_index(collapsed);

        let children = [
            &self.content_collapsed,
            &self.content_expanded,
            &self.fade,
            &self.expand,
            &self.copy,
        ];
        let active = [
            active_content == CONTENT_COLLAPSED,
            active_content == CONTENT_EXPANDED,
            self.fade_active(collapsed),
            self.expand_active(collapsed),
            self.copy_active,
        ];

        let Some(clipped) = layout.bounds().intersection(viewport) else {
            return;
        };

        // Content first, clipped to the code frame (`overflow-y: hidden` when
        // collapsed; also keeps nested page-scroll compositing correct).
        {
            let content_layout = layout
                .children()
                .nth(active_content)
                .unwrap_or_else(|| unreachable!("CodeView always lays out {CHILD_COUNT} children"));
            renderer.with_layer(clipped, |renderer| {
                children[active_content].as_widget().draw(
                    &tree.children[active_content],
                    renderer,
                    theme,
                    style,
                    content_layout,
                    cursor,
                    &clipped,
                );
            });
        }

        // Overlays above the code (fade → expand → copy), each in its own
        // layer so ScrollArea/content layers cannot paint over the copy button.
        for index in [FADE, EXPAND, COPY] {
            if !active[index] {
                continue;
            }
            let child_layout = layout
                .children()
                .nth(index)
                .unwrap_or_else(|| unreachable!("CodeView always lays out {CHILD_COUNT} children"));
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
        let collapsed = self.effective_collapsed(tree);
        let active_content = Self::content_index(collapsed);

        let children = [
            &self.content_collapsed,
            &self.content_expanded,
            &self.fade,
            &self.expand,
            &self.copy,
        ];
        let active = [
            active_content == CONTENT_COLLAPSED,
            active_content == CONTENT_EXPANDED,
            self.fade_active(collapsed),
            self.expand_active(collapsed),
            self.copy_active,
        ];

        for index in (0..CHILD_COUNT).rev() {
            if !active[index] {
                continue;
            }
            let interaction = children[index].as_widget().mouse_interaction(
                &tree.children[index],
                layout.children().nth(index).unwrap_or_else(|| {
                    unreachable!("CodeView always lays out {CHILD_COUNT} children")
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
