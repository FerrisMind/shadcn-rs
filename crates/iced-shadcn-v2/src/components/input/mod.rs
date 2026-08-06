//! Input component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Wraps `iced::widget::text_input` with `.cn-input` styling from the active
//! style pack: control height, corner radius, fill, border, placeholder and
//! disabled treatments all restyle together with [`crate::Theme`]. The value
//! is controlled — the application owns the [`String`] and receives edits
//! through [`Input::on_input`], mirroring the `bind:value` contract of the web
//! component.
//!
//! Web `type` attributes map as follows: `type="password"` is
//! [`Input::secure`]; `type="file"` has no iced counterpart; the remaining
//! types (`email`, `number`, …) only add browser chrome/validation and stay an
//! application concern. `aria-invalid` is [`Input::invalid`] and `disabled` is
//! [`Input::disabled`].
//!
//! Two web details degrade on iced: the translucent `focus-visible:ring-*`
//! halo is approximated by recoloring the border with `ring`, and Sera's
//! underline-only border becomes a full hairline box.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Input, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     EmailChanged(String),
//! }
//!
//! fn email<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
//!     Input::new(theme)
//!         .value(value)
//!         .placeholder("Email")
//!         .on_input(Message::EmailChanged)
//!         .into()
//! }
//! ```

mod error;
mod geometry;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::InputBuildError;
pub use types::{InputRadius, InputSize};

use std::fmt;

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::text_input as text_input_widget;
use crate::iced_compat::{Element, Font, Length, Pixels, alignment, widget};

use shadcn_common::AccentColor;
use twill_core::prelude::Padding;

use crate::fonts::iced_font;
use crate::theme::Theme;

/// Builder-first input styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]; iced styles are built
/// directly on top of `twill-core` tokens, without an intermediate style
/// layer. Pass `&theme` into every input — style packs (Vega, Nova, …) live on
/// the app's [`Theme`], not on this builder.
///
/// [`Self::style_override`] only patches the resolved iced
/// `text_input::Style` (background, border, text colors). It is not
/// [`shadcn_common::StyleId`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Input, InputSize, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     UsernameChanged(String),
///     Submitted,
/// }
///
/// fn username<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
///     Input::new(theme)
///         .value(value)
///         .placeholder("Username")
///         .size(InputSize::Lg)
///         .on_input(Message::UsernameChanged)
///         .on_submit(Message::Submitted)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Input<'a, Message> {
    theme: &'a Theme,
    value: Fragment<'a>,
    placeholder: Fragment<'a>,
    size: InputSize,
    radius: Option<InputRadius>,
    /// `None` = theme ring/primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    padding: Option<crate::iced_compat::Padding>,
    group_inline_start: bool,
    group_inline_end: bool,
    text_size: Option<f32>,
    align_x: alignment::Horizontal,
    secure: bool,
    disabled: bool,
    invalid: bool,
    id: Option<widget::Id>,
    icon: Option<text_input_widget::Icon<Font>>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_paste: Option<Box<dyn Fn(String) -> Message + 'a>>,
    style_override: Option<
        Box<
            dyn Fn(text_input_widget::Style, text_input_widget::Status) -> text_input_widget::Style
                + 'a,
        >,
    >,
}

impl<Message> fmt::Debug for Input<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Never leak a secure (password) value through Debug output.
        let value: &dyn fmt::Debug = if self.secure {
            &"<secure>"
        } else {
            &self.value
        };

        formatter
            .debug_struct("Input")
            .field("theme", &self.theme)
            .field("value", value)
            .field("placeholder", &self.placeholder)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("group_inline_start", &self.group_inline_start)
            .field("group_inline_end", &self.group_inline_end)
            .field("text_size", &self.text_size)
            .field("align_x", &self.align_x)
            .field("secure", &self.secure)
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("id", &self.id)
            .field("icon", &self.icon.is_some())
            .field("on_input", &self.on_input.is_some())
            .field("on_submit", &self.on_submit.is_some())
            .field("on_paste", &self.on_paste.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Input<'a, Message> {
    /// Creates an empty input.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Input, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let input = Input::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: Fragment::default(),
            placeholder: Fragment::default(),
            size: InputSize::Default,
            radius: None,
            color: None,
            width: Length::Fill,
            padding: None,
            group_inline_start: false,
            group_inline_end: false,
            text_size: None,
            align_x: alignment::Horizontal::Left,
            secure: false,
            disabled: false,
            invalid: false,
            id: None,
            icon: None,
            on_input: None,
            on_submit: None,
            on_paste: None,
            style_override: None,
        }
    }

    pub(crate) const fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub(crate) const fn is_invalid(&self) -> bool {
        self.invalid
    }

    pub(crate) fn focus_id(&self) -> Option<widget::Id> {
        self.id.clone()
    }

    pub(crate) fn group_slot_padding(mut self, inline_start: bool, inline_end: bool) -> Self {
        self.group_inline_start = inline_start;
        self.group_inline_end = inline_end;
        self
    }

    /// Sets the controlled value shown in the input.
    ///
    /// The application owns the text: store the [`String`] in state and feed
    /// it back on every [`Self::on_input`] message.
    pub fn value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.value = value.into_fragment();
        self
    }

    /// Sets the placeholder shown while the value is empty.
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = placeholder.into_fragment();
        self
    }

    /// Sets the preset control size.
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the input corner radius.
    ///
    /// Without an explicit radius the active style pack decides (`rounded-md`
    /// on Vega, pill on Maia/Luma, square on Lyra/Sera, …).
    pub fn radius(mut self, radius: InputRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the focus ring and selection.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`Input::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Sets a custom input width (`Length::Fill` by default, like `w-full`).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets all supported sides of the input padding.
    ///
    /// The default padding recreates `.cn-input` (`px-*` from the pack, `py`
    /// derived from the fixed control height); overriding it also changes the
    /// resulting control height.
    ///
    /// [`twill_core::prelude::PaddingValue::Var`] cannot be resolved by iced
    /// and is rejected with [`InputBuildError::UnsupportedPaddingVariable`].
    /// The same applies to [`twill_core::prelude::Spacing::Auto`], which has
    /// no fixed-size iced representation.
    ///
    /// # Errors
    ///
    /// Returns [`InputBuildError`] when any padding side contains a custom
    /// variable or `auto` value. The builder is consumed either way; rebuild
    /// the input with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, InputBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Sets the value text size. The pack's `.cn-input` size is used by
    /// default (`text-sm` on Vega, `text-xs` on Lyra/Mira).
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into().0);
        self
    }

    /// Sets the horizontal alignment of the value text.
    pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Masks the value like the web `type="password"` input.
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Disables the input (`disabled` attribute: no edits, 50% opacity).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Marks the value as invalid (`aria-invalid`): the border turns
    /// `destructive` and outranks the focus treatment.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets the widget id, enabling focus management via
    /// `iced::widget::text_input::focus`.
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets a font-glyph icon inside the input (iced extension; the web
    /// component composes icons through `input-group` instead).
    pub fn icon(mut self, icon: text_input_widget::Icon<Font>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the callback receiving the edited text.
    ///
    /// Without it (or when [`Self::disabled`] is set) the input rejects edits,
    /// matching the iced `text_input` contract.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets or clears the callback receiving the edited text.
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_input = on_input.map(|on_input| Box::new(on_input) as _);
        self
    }

    /// Sets the message emitted when Enter is pressed while focused.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets or clears the message emitted when Enter is pressed.
    pub fn on_submit_maybe(mut self, message: Option<Message>) -> Self {
        self.on_submit = message;
        self
    }

    /// Sets the callback receiving pasted text.
    pub fn on_paste(mut self, on_paste: impl Fn(String) -> Message + 'a) -> Self {
        self.on_paste = Some(Box::new(on_paste));
        self
    }

    /// Sets or clears the callback receiving pasted text.
    pub fn on_paste_maybe(mut self, on_paste: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_paste = on_paste.map(|on_paste| Box::new(on_paste) as _);
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(
            text_input_widget::Style,
            text_input_widget::Status,
        ) -> text_input_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying `iced` text-input widget.
    pub fn into_text_input(self) -> text_input_widget::TextInput<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Input {
            theme,
            value,
            placeholder,
            size,
            radius,
            color,
            width,
            padding,
            group_inline_start,
            group_inline_end,
            text_size,
            align_x,
            secure,
            disabled,
            invalid,
            id,
            icon,
            on_input,
            on_submit,
            on_paste,
            style_override,
        } = self;

        let text_size = text_size.unwrap_or_else(|| style::pack_text_size(theme));
        let mut resolved_padding =
            padding.unwrap_or_else(|| geometry::default_padding(theme, size, text_size));
        let group_pad_x = style::group_slot_pad_x(theme);
        if group_inline_start {
            resolved_padding.left = group_pad_x;
        }
        if group_inline_end {
            resolved_padding.right = group_pad_x;
        }

        let mut widget = text_input_widget::TextInput::new(placeholder.as_ref(), value.as_ref())
            .size(text_size)
            .line_height(LineHeight::Absolute(
                geometry::line_height_px(text_size).into(),
            ))
            .font(iced_font(theme.font_pack().sans))
            .padding(resolved_padding)
            .width(width)
            .align_x(align_x)
            .secure(secure);

        if let Some(id) = id {
            widget = widget.id(id);
        }

        if let Some(icon) = icon {
            widget = widget.icon(icon);
        }

        if !disabled {
            widget = widget.on_input_maybe(on_input).on_submit_maybe(on_submit);

            if let Some(on_paste) = on_paste {
                widget = widget.on_paste(on_paste);
            }
        }

        widget.style(move |_iced_theme, status| {
            let mut style =
                style::resolve_input_style(theme, radius, color, invalid, disabled, status);

            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style, status);
            }

            style
        })
    }
}

/// Convenience wrapper mirroring [`iced::widget::text_input()`](iced_widget::text_input()).
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, input};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     NameChanged(String),
/// }
///
/// fn name<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
///     input("Name", value, theme)
///         .on_input(Message::NameChanged)
///         .into()
/// }
/// ```
pub fn input<'a, Message>(
    placeholder: impl IntoFragment<'a>,
    value: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> Input<'a, Message> {
    Input::new(theme).placeholder(placeholder).value(value)
}

impl<'a, Message> From<Input<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(input: Input<'a, Message>) -> Self {
        if style::uses_underline_only(input.theme) {
            let theme = input.theme;
            let color = input.color;
            let invalid = input.invalid;
            let disabled = input.disabled;
            let width = input.width;
            let text_input_el: Element<'a, Message> = input.into_text_input().into();

            // The underline is a 1px-tall container colored with the resolved
            // border-b color (resting = input, focus = ring, invalid =
            // destructive). Since iced text_input status is not exposed after
            // build, we resolve only the resting + invalid states statically;
            // the focus treatment degrades to the same resting underline
            // (matching how the web Sera uses a simple color transition that
            // is hard to replicate without a live-status callback).
            let underline_color = style::resolve_underline_color(
                theme,
                color,
                invalid,
                disabled,
                crate::iced_compat::widget::text_input::Status::Active,
            );

            widget::column![
                text_input_el,
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
            input.into_text_input().into()
        }
    }
}
