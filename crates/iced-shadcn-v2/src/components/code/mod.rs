//! Builder-first syntax-highlighted code block component for `iced-shadcn-v2`.
//!
//! The component mirrors the `Code` block from `shadcn-svelte-extra`: a
//! scrollable `<pre>` with syntax-highlighted lines, optional line numbers,
//! line highlighting, a copy-to-clipboard button, and an overflow wrapper
//! with a gradient fade + expand button.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Code, CodeCopyButton, CodeOverflow, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Copy,
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     Code::new("fn main() {\n    println!(\"Hello!\");\n}", "rust", theme)
//!         .copy_button(CodeCopyButton::new(Message::Copy))
//!         .overflow(CodeOverflow::new(true))
//!         .into()
//! }
//! ```

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{CodeCopyButton, CodeOverflow, CodeVariant};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::copy_button::CopyButton;
use crate::components::scroll_area::ScrollArea;
use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::{Space, column, container};
use crate::iced_compat::{Element, Length, Padding};
use crate::theme::Theme;
use std::fmt;

use render::{
    CodeView, OverflowView, PRE_VERTICAL_PAD, build_line, code_container_style,
    fade_gradient_style, mono_font, split_lines,
};

use shadcn_common::{
    CodeLineHighlight, LanguageId, code_palette, highlight_code, line_is_highlighted,
};

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for a syntax-highlighted code block.
///
/// Create one with [`Code::new`], configure it with the builder methods, then
/// call [`.into()`](Element::from) or [`.into_element()`](Self::into_element)
/// to produce an iced `Element`.
///
/// The builder borrows the theme; build it inside your `view` function.
///
/// # Examples
///
/// ```rust,no_run
/// use iced_shadcn_v2::{Code, CodeCopyButton, CodeOverflow, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message { Copy, ToggleOverflow }
/// # let theme = Theme::light();
/// let code_block = Code::new("let x = 42;", "rust", &theme)
///     .highlight(vec![1u32.into()])
///     .copy_button(CodeCopyButton::new(Message::Copy))
///     .overflow(CodeOverflow::new(true))
///     .into_element();
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Code<'a, Message> {
    source: &'a str,
    lang: LanguageId,
    theme: &'a Theme,
    variant: CodeVariant,
    hide_lines: bool,
    highlight: Vec<CodeLineHighlight>,
    copy_button: Option<CodeCopyButton<'a, Message>>,
    overflow: Option<CodeOverflow<'a, Message>>,
    width: Length,
    height: Option<Length>,
}

impl<Message> fmt::Debug for Code<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Code")
            .field("source_length", &self.source.len())
            .field("lang", &self.lang)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("hide_lines", &self.hide_lines)
            .field("highlight", &self.highlight)
            .field("copy_button", &self.copy_button.is_some())
            .field("overflow", &self.overflow.is_some())
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl<'a, Message> Code<'a, Message> {
    /// Creates a new code block builder.
    ///
    /// `source` is the raw source code to highlight. `lang` selects the
    /// syntax grammar (any name accepted by
    /// [`LanguageId::parse_name`](LanguageId::parse_name); unknown names fall
    /// back to plain text). `theme` provides colors, fonts, and radius tokens.
    pub fn new(source: &'a str, lang: impl Into<LanguageId>, theme: &'a Theme) -> Self {
        Self {
            source,
            lang: lang.into(),
            theme,
            variant: CodeVariant::default(),
            hide_lines: false,
            highlight: Vec::new(),
            copy_button: None,
            overflow: None,
            width: Length::Fill,
            height: None,
        }
    }

    /// Sets the visual variant (`default` or `secondary`).
    pub fn variant(mut self, variant: CodeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// When `true`, line numbers are hidden (the `hideLines` prop of the
    /// reference Svelte component).
    pub fn hide_lines(mut self, hide: bool) -> Self {
        self.hide_lines = hide;
        self
    }

    /// Sets the lines to highlight. Accepts anything that converts into
    /// [`CodeLineHighlight`]: `u32`, `(u32, u32)`, or `RangeInclusive<u32>`.
    pub fn highlight(mut self, highlights: Vec<CodeLineHighlight>) -> Self {
        self.highlight = highlights;
        self
    }

    /// Attaches a copy-to-clipboard button positioned at the top-right corner
    /// of the code block.
    pub fn copy_button(mut self, button: CodeCopyButton<'a, Message>) -> Self {
        self.copy_button = Some(button);
        self
    }

    /// Enables the overflow wrapper with collapse/expand behavior.
    pub fn overflow(mut self, overflow: CodeOverflow<'a, Message>) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Sets the width of the code block.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a fixed viewport height; content that overflows scrolls via
    /// [`ScrollArea`](crate::ScrollArea).
    ///
    /// Without an explicit height the block grows with its content and does
    /// not show a scrollbar. Collapsed [`CodeOverflow`] also never scrolls —
    /// it clips to `max_height` like the reference `overflow-y-hidden`.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Builds the code block as an iced `Element`.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let theme = self.theme;
        let font = mono_font(theme);
        let palette = code_palette(theme.mode());

        // 1. Highlight the source (trimmed like the Svelte `code.trimEnd()`).
        let trimmed = self.source.trim_end();
        let tokens = highlight_code(trimmed, self.lang);

        // 2. Split into lines.
        let lines = split_lines(trimmed);

        // 3. Build the `<pre>` content (line grid + `py-4` padding).
        let build_pre = || -> Element<'a, Message> {
            let line_elements: Vec<Element<'a, Message>> = lines
                .iter()
                .map(|line| {
                    let highlighted = line_is_highlighted(&self.highlight, line.number);
                    build_line(
                        line,
                        &tokens,
                        trimmed,
                        &palette,
                        font,
                        self.hide_lines,
                        highlighted,
                        theme,
                    )
                })
                .collect();

            container(column(line_elements).width(Length::Fill))
                .width(Length::Fill)
                .padding(
                    Padding::default()
                        .top(PRE_VERTICAL_PAD)
                        .bottom(PRE_VERTICAL_PAD),
                )
                .into()
        };

        // Collapsed overflow: plain pre, clipped by CodeView (no scroll —
        // Svelte `overflow-y-hidden` + `max-h-[300px]`).
        let content_collapsed: Element<'a, Message> = build_pre();

        // Expanded / default: scrolling is opt-in via `.height(...)`. Without
        // an explicit height the block grows with its content (no scrollbar).
        let content_expanded: Element<'a, Message> = if self.height.is_some() {
            ScrollArea::new(build_pre(), theme)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            build_pre()
        };

        // 4. Overflow decorations (fade + expand button), active only while
        //    the block is collapsed.
        let fade: Element<'a, Message> = container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(fade_gradient_style(theme))
            .into();

        let (overflow_view, expand_button): (
            Option<OverflowView<'a, Message>>,
            Element<'a, Message>,
        ) = match self.overflow {
            Some(CodeOverflow {
                default_collapsed,
                collapsed_override,
                max_height,
                expand_button,
                on_collapse_change,
            }) => (
                Some(OverflowView {
                    collapsed_override,
                    default_collapsed,
                    max_height,
                    on_collapse_change,
                }),
                expand_button.unwrap_or_else(|| {
                    // Default "Expand" button of the reference component;
                    // the widget toggles the collapsed state itself, so
                    // no `on_press` is required.
                    Button::text("Expand", theme)
                        .variant(ButtonVariant::Secondary)
                        .size(ButtonSize::Sm)
                        .into()
                }),
            ),
            None => (None, Space::new().into()),
        };

        let expand: Element<'a, Message> = container(expand_button)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Bottom)
            .padding(Padding::default().bottom(render::EXPAND_BOTTOM_OFFSET))
            .into();

        // 5. Copy button overlay, active only when a copy button is given.
        let copy_active = self.copy_button.is_some();
        let copy: Element<'a, Message> = match self.copy_button {
            Some(button) => {
                let mut copy_button = CopyButton::new(trimmed, theme)
                    .variant(button.variant)
                    .size(button.size)
                    .status(button.status)
                    .on_copy(button.on_copy);

                if let Some(radius) = button.radius {
                    copy_button = copy_button.radius(radius);
                }
                if let Some(icon) = button.icon {
                    copy_button = copy_button.icon(icon);
                }

                container(copy_button.into_element())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Top)
                    .padding(
                        Padding::default()
                            .top(render::COPY_OFFSET)
                            .right(render::COPY_OFFSET),
                    )
                    .into()
            }
            None => Space::new().into(),
        };

        // 6. Assemble the custom widget and wrap it in the outer container
        //    (`border-border bg-card rounded-lg`, per `codeVariants`).
        let content_height = 2.0 * PRE_VERTICAL_PAD + lines.len() as f32 * render::LINE_HEIGHT;

        let view = CodeView {
            content_collapsed,
            content_expanded,
            fade,
            expand,
            copy,
            copy_active,
            overflow: overflow_view,
            content_height,
            width: self.width,
            height: self.height,
        };

        container(Element::new(view))
            .width(self.width)
            .style(code_container_style(self.variant, theme))
            .into()
    }
}

impl<'a, Message: Clone + 'a> From<Code<'a, Message>> for Element<'a, Message> {
    fn from(code: Code<'a, Message>) -> Self {
        code.into_element()
    }
}
