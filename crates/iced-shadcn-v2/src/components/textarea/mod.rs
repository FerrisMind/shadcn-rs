//! Textarea component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Wraps `iced::widget::text_editor` with `.cn-textarea` styling from the
//! active style pack: corner radius, fill, border, placeholder, disabled and
//! invalid treatments all restyle together with [`crate::Theme`]. The value is
//! controlled — the application owns the [`text_editor::Content`] and receives
//! edits through [`Textarea::on_action`], mirroring the `bind:value` contract
//! of the web component.
//!
//! Geometry and surface tokens come from the shared [`TextareaRecipe`] in
//! `shadcn-common`, so iced and egui share one source of truth. `aria-invalid`
//! is [`Textarea::invalid`] and `disabled` is [`Textarea::disabled`].
//!
//! Two web details degrade on iced: the translucent `focus-visible:ring-*`
//! halo is approximated by recoloring the border with `ring`, and Sera's
//! underline-only border becomes a transparent editor box plus a bottom
//! hairline drawn by the `From<Textarea> for Element` wrapper.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Textarea, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     BioEdited(iced::widget::text_editor::Action),
//! }
//!
//! fn bio<'a>(
//!     theme: &'a Theme,
//!     content: &'a iced::widget::text_editor::Content,
//! ) -> Element<'a, Message> {
//!     Textarea::new(content, theme)
//!         .placeholder("Tell us a little bit about yourself")
//!         .on_action(Message::BioEdited)
//!         .into()
//! }
//! ```

mod geometry;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{TextareaRadius, TextareaResize, TextareaSize};

use std::fmt;

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::text_editor;
use crate::iced_compat::{Element, Length, widget};

use shadcn_common::AccentColor;

use crate::fonts::iced_font;
use crate::theme::Theme;

/// Builder-first textarea styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`] (resolved through the
/// shared [`TextareaRecipe`]); iced styles are built directly on top of
/// `twill-core` tokens, without an intermediate style layer. Pass `&theme`
/// into every textarea — style packs (Vega, Nova, …) live on the app's
/// [`Theme`], not on this builder.
///
/// [`Self::style_override`] only patches the resolved iced
/// `text_editor::Style` (background, border, text colors). It is not
/// [`shadcn_common::StyleId`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Textarea<'a, Message> {
    theme: &'a Theme,
    content: &'a text_editor::Content,
    placeholder: Fragment<'a>,
    size: TextareaSize,
    radius: Option<TextareaRadius>,
    /// `None` = theme ring/primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    padding: Option<[f32; 2]>,
    text_size: Option<f32>,
    rows: Option<usize>,
    max_rows: Option<usize>,
    resize: TextareaResize,
    wrapping: iced_core::text::Wrapping,
    max_len: Option<usize>,
    disabled: bool,
    read_only: bool,
    invalid: bool,
    id: Option<widget::Id>,
    on_action: Option<Box<dyn Fn(text_editor::Action) -> Message + 'a>>,
    style_override:
        Option<Box<dyn Fn(text_editor::Style, text_editor::Status) -> text_editor::Style + 'a>>,
}

impl<Message> fmt::Debug for Textarea<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Textarea")
            .field("theme", &self.theme)
            .field("content", &self.content.text())
            .field("placeholder", &self.placeholder)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("text_size", &self.text_size)
            .field("rows", &self.rows)
            .field("max_rows", &self.max_rows)
            .field("resize", &self.resize)
            .field("wrapping", &self.wrapping)
            .field("max_len", &self.max_len)
            .field("disabled", &self.disabled)
            .field("read_only", &self.read_only)
            .field("invalid", &self.invalid)
            .field("id", &self.id)
            .field("on_action", &self.on_action.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Textarea<'a, Message> {
    /// Creates a textarea backed by caller-owned iced editor content.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Textarea, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let content = iced::widget::text_editor::Content::new();
    /// let textarea = Textarea::<Message>::new(&content, &theme);
    /// ```
    pub fn new(content: &'a text_editor::Content, theme: &'a Theme) -> Self {
        Self {
            theme,
            content,
            placeholder: Fragment::default(),
            size: TextareaSize::Default,
            radius: None,
            color: None,
            width: Length::Fill,
            padding: None,
            text_size: None,
            rows: None,
            max_rows: None,
            resize: TextareaResize::None,
            wrapping: geometry::default_wrapping(),
            max_len: None,
            disabled: false,
            read_only: false,
            invalid: false,
            id: None,
            on_action: None,
            style_override: None,
        }
    }

    /// Whether the textarea is disabled. Exposed for future `InputGroup`
    /// integration; standalone use goes through [`Self::disabled`].
    #[allow(dead_code)]
    pub(crate) const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Whether the textarea is marked invalid. Exposed for future `InputGroup`
    /// integration; standalone use goes through [`Self::invalid`].
    #[allow(dead_code)]
    pub(crate) const fn is_invalid(&self) -> bool {
        self.invalid
    }

    /// The editor id, if set. Exposed for future `InputGroup` integration so a
    /// group addon can focus the embedded textarea on click.
    #[allow(dead_code)]
    pub(crate) fn focus_id(&self) -> Option<widget::Id> {
        self.id.clone()
    }

    /// Reserves inline-start / inline-end padding for a future
    /// [`crate::InputGroup`] embedding. Standalone textareas keep zero slot
    /// padding; this is only consumed when the textarea is later wrapped by a
    /// group that owns the shared border.
    #[doc(hidden)]
    pub fn group_slot_padding(mut self, inline_start: bool, inline_end: bool) -> Self {
        if inline_start || inline_end {
            let pad_x = style::group_slot_pad_x(self.theme);
            let current = self.padding.unwrap_or([0.0, pad_x]);
            self.padding = Some([current[0], pad_x]);
        }
        self
    }

    /// Sets the placeholder shown while the value is empty.
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = placeholder.into_fragment();
        self
    }

    /// Sets the preset control size.
    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the textarea corner radius.
    ///
    /// Without an explicit radius the active style pack decides (`rounded-md`
    /// on Vega, `rounded-lg` on Nova, square on Lyra/Sera, …).
    pub fn radius(mut self, radius: TextareaRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the focus ring and selection.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`Textarea::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Sets a custom textarea width (`Length::Fill` by default, like `w-full`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets vertical and horizontal padding in pixels (`[vertical, horizontal]`).
    ///
    /// The default padding recreates `.cn-textarea` (`px-*` / `py-*` from the
    /// pack recipe); overriding it also changes the resulting minimum height
    /// when `rows` is not set.
    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.padding = Some([padding[0].max(0.0), padding[1].max(0.0)]);
        self
    }

    /// Sets the value text size. The pack's `.cn-textarea` size is used by
    /// default (`text-sm` on Vega, `text-xs` on Lyra/Mira).
    pub fn text_size(mut self, text_size: impl Into<f32>) -> Self {
        self.text_size = Some(text_size.into().max(1.0));
        self
    }

    /// Sets the minimum number of rows, clamped to one.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows.max(1));
        self
    }

    /// Sets the maximum number of rows, clamped to one.
    pub fn max_rows(mut self, rows: usize) -> Self {
        self.max_rows = Some(rows.max(1));
        self
    }

    /// Sets the resize policy.
    ///
    /// Iced does not expose a browser-style resize handle. [`TextareaResize::None`]
    /// fixes the minimum height; the other modes leave the editor height
    /// unconstrained.
    pub fn resize(mut self, resize: TextareaResize) -> Self {
        self.resize = resize;
        self
    }

    /// Sets the iced text wrapping strategy.
    pub fn wrapping(mut self, wrapping: iced_core::text::Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets a maximum character count enforced by [`textarea_apply_action`].
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    /// Disables the textarea (`disabled` attribute: no edits, 50% opacity).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the textarea read-only (value visible, edits blocked, muted text).
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Marks the value as invalid (`aria-invalid`): the border turns
    /// `destructive` and outranks the focus treatment.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets the editor id, enabling focus management via
    /// `iced::widget::text_editor::focus`.
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the callback receiving editor actions.
    ///
    /// Without it (or when [`Self::disabled`] / [`Self::read_only`] is set) the
    /// textarea rejects edits, matching the iced `text_editor` contract.
    pub fn on_action(mut self, on_action: impl Fn(text_editor::Action) -> Message + 'a) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }

    /// Sets or clears the callback receiving editor actions.
    pub fn on_action_maybe(
        mut self,
        on_action: Option<impl Fn(text_editor::Action) -> Message + 'a>,
    ) -> Self {
        self.on_action = on_action.map(|callback| Box::new(callback) as _);
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(text_editor::Style, text_editor::Status) -> text_editor::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying `iced` text-editor widget.
    ///
    /// The editor defaults to `Length::Fill` width (matching the web `w-full`
    /// class). A [`Length::Fixed`] width is forwarded to the editor; `Fill`
    /// and `Shrink` keep the editor's native fill behavior, so the parent
    /// layout controls the final width — the same approach the input-group
    /// wrapper uses.
    pub fn into_text_editor(
        self,
    ) -> text_editor::TextEditor<'a, iced_core::text::highlighter::PlainText, Message>
    where
        Message: Clone + 'a,
    {
        let recipe = style::pack_recipe(self.theme);
        let Textarea {
            theme,
            content,
            placeholder,
            size,
            radius,
            color,
            width,
            padding,
            text_size,
            rows,
            max_rows,
            resize,
            wrapping,
            max_len: _,
            disabled,
            read_only,
            invalid,
            id,
            on_action,
            style_override,
        } = self;

        let resolved_text_size =
            text_size.unwrap_or_else(|| geometry::pack_text_size(recipe, size));
        let resolved_padding = padding.unwrap_or_else(|| geometry::pack_padding(recipe, size));
        let min_height = geometry::min_height(size, resolved_text_size, resolved_padding, rows);

        let mut widget = text_editor::TextEditor::new(content)
            .placeholder(placeholder)
            .padding(resolved_padding)
            .size(resolved_text_size)
            .line_height(LineHeight::Absolute(
                geometry::line_height_px(resolved_text_size).into(),
            ))
            .font(iced_font(theme.font_pack().sans))
            .min_height(min_height)
            .wrapping(wrapping)
            .style(move |_iced_theme, status| {
                let mut style = style::resolve_textarea_style(
                    theme, recipe, radius, color, invalid, disabled, read_only, status,
                );

                if let Some(override_fn) = style_override.as_ref() {
                    style = override_fn(style, status);
                }

                style
            });

        // `text_editor::width` takes `Pixels` (a fixed width); `Fill` is the
        // widget default, so only forward explicit fixed widths and let the
        // parent layout handle `Fill` / `Shrink`.
        if let Length::Fixed(px) = width {
            widget = widget.width(px);
        }

        if let Some(max_height) =
            geometry::max_height(resolved_text_size, resolved_padding, max_rows)
        {
            widget = widget.max_height(max_height);
        }

        if let Some(id) = id {
            widget = widget.id(id);
        }

        if style::fixes_height(resize) {
            widget = widget.height(Length::Fixed(min_height));
        }

        if !disabled
            && !read_only
            && let Some(on_action) = on_action
        {
            widget = widget.on_action(on_action);
        }

        widget
    }
}

/// Convenience wrapper mirroring [`iced::widget::text_editor()`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, textarea};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     BioEdited(iced::widget::text_editor::Action),
/// }
///
/// fn bio<'a>(
///     theme: &'a Theme,
///     content: &'a iced::widget::text_editor::Content,
/// ) -> Element<'a, Message> {
///     textarea(content, "Tell us a little bit about yourself", theme)
///         .on_action(Message::BioEdited)
///         .into()
/// }
/// ```
pub fn textarea<'a, Message>(
    content: &'a text_editor::Content,
    placeholder: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> Textarea<'a, Message> {
    Textarea::new(content, theme).placeholder(placeholder)
}

impl<'a, Message> From<Textarea<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(textarea: Textarea<'a, Message>) -> Self {
        if style::uses_underline_only(textarea.theme) {
            let theme = textarea.theme;
            let color = textarea.color;
            let invalid = textarea.invalid;
            let disabled = textarea.disabled;
            let width = textarea.width;
            let editor_el: Element<'a, Message> = textarea.into_text_editor().into();

            // The underline is a 1px-tall container colored with the resolved
            // border-b color (resting = input, focus = ring, invalid =
            // destructive). Since iced text editor status is not exposed after
            // build, we resolve only the resting + invalid states statically;
            // the focus treatment degrades to the same resting underline
            // (matching how the web Sera uses a simple color transition that is
            // hard to replicate without a live-status callback).
            let underline_color = style::resolve_underline_color(
                theme,
                color,
                invalid,
                disabled,
                crate::iced_compat::widget::text_editor::Status::Active,
            );

            widget::column![
                editor_el,
                widget::container(widget::Space::new())
                    .width(crate::iced_compat::Length::Fill)
                    .height(1.0)
                    .style(move |_| {
                        use crate::iced_compat::widget::container;
                        container::Style {
                            background: Some(crate::iced_compat::Background::Color(
                                underline_color,
                            )),
                            ..container::Style::default()
                        }
                    }),
            ]
            .width(width)
            .into()
        } else {
            textarea.into_text_editor().into()
        }
    }
}

/// Applies a textarea action while honoring read-only, disabled and
/// maximum-length options. The helper returns whether the content changed.
///
/// This is the standalone counterpart of
/// [`crate::components::input_group::input_group_textarea_apply_action`]; it
/// mirrors the same `max_len` enforcement so a standalone [`Textarea`] can
/// cap input without joining an input group.
pub fn textarea_apply_action(
    content: &mut text_editor::Content,
    action: text_editor::Action,
    disabled: bool,
    read_only: bool,
    max_len: Option<usize>,
) -> bool {
    if disabled || (read_only && action.is_edit()) {
        return false;
    }

    if let Some(max_len) = max_len
        && !can_apply_edit(content, &action, max_len)
    {
        return false;
    }

    content.perform(action);
    true
}

fn can_apply_edit(
    content: &text_editor::Content,
    action: &text_editor::Action,
    max_len: usize,
) -> bool {
    let text_editor::Action::Edit(edit) = action else {
        return true;
    };

    let current_len = content.text().chars().count();
    let selection_len = selection_len(content);
    let insert_len = match edit {
        text_editor::Edit::Insert(_) => 1,
        text_editor::Edit::Paste(text) => text.chars().count(),
        text_editor::Edit::Enter => content
            .line_ending()
            .unwrap_or_default()
            .as_str()
            .chars()
            .count(),
        text_editor::Edit::Indent
        | text_editor::Edit::Unindent
        | text_editor::Edit::Backspace
        | text_editor::Edit::Delete => 0,
    };

    insert_len == 0 || current_len.saturating_sub(selection_len) + insert_len <= max_len
}

fn selection_len(content: &text_editor::Content) -> usize {
    let cursor = content.cursor();
    let Some(selection) = cursor.selection else {
        return 0;
    };

    position_to_index(content, cursor.position).abs_diff(position_to_index(content, selection))
}

fn position_to_index(content: &text_editor::Content, position: text_editor::Position) -> usize {
    let mut index = 0;

    for (line_index, line) in content.lines().enumerate() {
        if line_index == position.line {
            return index + position.column.min(line.text.chars().count());
        }

        index += line.text.chars().count();
        index += line.ending.as_str().chars().count();
    }

    index
}
