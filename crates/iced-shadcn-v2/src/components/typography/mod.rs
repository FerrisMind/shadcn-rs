//! Builder-first typography component.
//!
//! Port of the shadcn-svelte “Typography” page. The web version ships no
//! component — only documented utility-class recipes (`h1` … `muted`). Here
//! those recipes become a [`Typography`] builder plus [`TypographyList`] and
//! [`TypographyTable`] for the two non-text examples. The public API lives in
//! this module; recipes, rendering, and style constants are kept in focused
//! private submodules.

mod list;
mod render;
mod style;
mod table;
mod types;

#[cfg(test)]
mod tests;

pub use list::TypographyList;
pub use table::TypographyTable;
pub use types::TypographyVariant;

use std::fmt;

use crate::iced_compat::alignment::Horizontal;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Color, Element, Length};

use crate::theme::Theme;

/// Builder-first text block styled with the shadcn typography recipes.
///
/// Mirrors the shadcn-svelte typography examples: theme-aware fonts (heading
/// face for `h1`–`h4`, mono for inline code), muted foreground for `lead` /
/// `muted`, the `h2` bottom border, the italic blockquote bar, and the
/// inline-code chip. Theme tokens come from `shadcn-common` via [`Theme`].
///
/// Block variants default to full width (like block-level HTML elements) so
/// text wraps; [`TypographyVariant::InlineCode`] hugs its content. Spacing
/// between blocks is opt-in via [`Self::default_margin`] / [`Self::margin_top`]
/// because iced layouts usually control spacing at the `column` level.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, Typography};
///
/// fn title(theme: &Theme) -> Element<'_, ()> {
///     Typography::h1("Taxing Laughter: The Joke Tax Chronicles", theme).into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Typography<'a, Message> {
    content: TypographyContent<'a, Message>,
    theme: &'a Theme,
    variant: TypographyVariant,
    color: Option<Color>,
    size: Option<f32>,
    line_height: Option<f32>,
    width: Option<Length>,
    align_x: Horizontal,
    margin_top: Option<f32>,
    use_default_margin: bool,
}

enum TypographyContent<'a, Message> {
    Text(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Typography<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            TypographyContent::Text(_) => "text",
            TypographyContent::Element(_) => "element",
        };

        formatter
            .debug_struct("Typography")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("color", &self.color)
            .field("size", &self.size)
            .field("line_height", &self.line_height)
            .field("width", &self.width)
            .field("align_x", &self.align_x)
            .field("margin_top", &self.margin_top)
            .field("use_default_margin", &self.use_default_margin)
            .finish()
    }
}

impl<'a, Message> Typography<'a, Message> {
    /// Creates a text block with the default paragraph variant.
    ///
    /// `theme` is required because typography and color resolve from
    /// `shadcn-common` theme tokens instead of `iced::Theme`.
    pub fn text(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(TypographyContent::Text(content.into_fragment()), theme)
    }

    /// Wraps arbitrary content with the variant chrome (underline, quote bar,
    /// code chip).
    ///
    /// Font, size, and color settings only apply to text content — custom
    /// elements keep their own styling (the iced stand-in for nesting markup
    /// inside `<h2>` / `<blockquote>` on the web).
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(TypographyContent::Element(content.into()), theme)
    }

    /// `h1` text block.
    pub fn h1(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::H1)
    }

    /// `h2` text block (with bottom border).
    pub fn h2(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::H2)
    }

    /// `h3` text block.
    pub fn h3(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::H3)
    }

    /// `h4` text block.
    pub fn h4(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::H4)
    }

    /// Paragraph text block.
    pub fn p(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::P)
    }

    /// Italic quote with a leading bar.
    pub fn blockquote(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::Blockquote)
    }

    /// Mono code chip on a muted background.
    pub fn inline_code(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::InlineCode)
    }

    /// Muted intro paragraph (`lead`).
    pub fn lead(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::Lead)
    }

    /// Emphasized inline block (`large`).
    pub fn large(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::Large)
    }

    /// Compact medium-weight text (`small`).
    pub fn small(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::Small)
    }

    /// Muted helper text (`muted`).
    pub fn muted(content: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::text(content, theme).variant(TypographyVariant::Muted)
    }

    fn from_content(content: TypographyContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            variant: TypographyVariant::default(),
            color: None,
            size: None,
            line_height: None,
            width: None,
            align_x: Horizontal::Left,
            margin_top: None,
            use_default_margin: false,
        }
    }

    /// Sets the text treatment.
    pub fn variant(mut self, variant: TypographyVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Overrides the foreground color (defaults to theme `foreground`, or
    /// `muted_foreground` for `lead` / `muted`).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Overrides the font size in px (e.g. `48.0` for the web `lg:text-5xl`
    /// step of `h1`). The line height scales proportionally unless
    /// [`Self::line_height`] is also set.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size.max(1.0));
        self
    }

    /// Overrides the absolute line height in px.
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(line_height.max(1.0));
        self
    }

    /// Sets a custom block width.
    ///
    /// Defaults to [`Length::Fill`] (block-level flow) for every variant
    /// except [`TypographyVariant::InlineCode`], which shrinks to its content.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the horizontal text alignment inside the block.
    pub fn align_x(mut self, align_x: impl Into<Horizontal>) -> Self {
        self.align_x = align_x.into();
        self
    }

    /// Sets an explicit top margin in px (beats [`Self::default_margin`]).
    pub fn margin_top(mut self, margin_top: f32) -> Self {
        self.margin_top = Some(margin_top.max(0.0));
        self
    }

    /// Applies the article-flow top margin of the current variant
    /// ([`TypographyVariant::default_margin_top_px`]) — `mt-10` before `h2`,
    /// `mt-6` before paragraphs, and so on.
    pub fn default_margin(mut self, default_margin: bool) -> Self {
        self.use_default_margin = default_margin;
        self
    }

    /// Builds the typography block as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let Self {
            content,
            theme,
            variant,
            color,
            size,
            line_height,
            width,
            align_x,
            margin_top,
            use_default_margin,
        } = self;

        let recipe = variant.type_recipe();
        let size_px = size.unwrap_or(recipe.size_px);
        let line_height_px =
            line_height.unwrap_or(recipe.line_height_px * size_px / recipe.size_px);

        let resolved = render::ResolvedText {
            size_px,
            line_height_px,
            color: style::resolve_color(theme, variant, color),
            font: style::resolve_font(theme, variant),
            align_x,
        };

        let default_width = if variant == TypographyVariant::InlineCode {
            Length::Shrink
        } else {
            Length::Fill
        };
        let width = width.unwrap_or(default_width);
        // Chromed variants size their wrapper; the inner text fills it.
        let text_width = match variant {
            TypographyVariant::H2 | TypographyVariant::Blockquote => Length::Fill,
            _ => width,
        };

        let body = match content {
            TypographyContent::Text(fragment) => render::text_block(fragment, resolved, text_width),
            TypographyContent::Element(element) => element,
        };

        let body = render::apply_chrome(variant, body, theme, width);

        let margin = margin_top.unwrap_or(if use_default_margin {
            variant.default_margin_top_px()
        } else {
            0.0
        });

        render::apply_margin_top(body, margin)
    }
}

impl<'a, Message> From<Typography<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(typography: Typography<'a, Message>) -> Self {
        typography.into_element()
    }
}
