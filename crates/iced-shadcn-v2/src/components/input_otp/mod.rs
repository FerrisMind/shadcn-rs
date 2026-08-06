//! Input-OTP component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! One-time-password field with per-character slots, mirroring the web
//! `InputOTP.Root` / `Group` / `Slot` / `Separator` composition built on
//! `bits-ui`'s `PinInput`. The web component hides a real `<input>` under
//! visual cells; here a single custom widget owns focus, keyboard editing,
//! and clipboard handling, and paints every slot, divider, active ring,
//! fake caret, and group separator from the active style pack's
//! `.cn-input-otp*` recipe.
//!
//! The value is controlled — the application owns the [`String`] and
//! receives edits through [`InputOtp::on_input`], mirroring the
//! `bind:value` contract. [`InputOtp::on_complete`] fires when every slot
//! is filled (web `onComplete`), [`InputOtp::pattern`] maps the
//! `REGEXP_ONLY_*` patterns, [`InputOtp::groups`] replaces the manual
//! `Group`/`Separator` markup, and `disabled` / `aria-invalid` map to
//! [`InputOtp::disabled`] / [`InputOtp::invalid`].
//!
//! Two web details degrade on iced: the translucent `ring-*` halo is
//! painted as a border-only quad instead of a box shadow, and the
//! `textalign` prop has no counterpart because the caret always sits after
//! the last entered character.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{InputOtp, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     CodeChanged(String),
//!     CodeComplete(String),
//! }
//!
//! fn code<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
//!     InputOtp::new(theme)
//!         .value(value)
//!         .groups([3, 3])
//!         .on_input(Message::CodeChanged)
//!         .on_complete(Message::CodeComplete)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{InputOtpPattern, InputOtpRadius, InputOtpStatus, InputOtpStyle};

use std::fmt;

use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Pixels, widget};

use shadcn_common::AccentColor;

use crate::theme::Theme;

/// `maxlength={6}` used by every shadcn-svelte example.
const DEFAULT_MAX_LENGTH: usize = 6;

type OnInput<'a, Message> = Box<dyn Fn(String) -> Message + 'a>;
type OnComplete<'a, Message> = Box<dyn Fn(String) -> Message + 'a>;
type PasteTransformer<'a> = Box<dyn Fn(String) -> String + 'a>;
type StyleOverride<'a> = Box<dyn Fn(InputOtpStyle, InputOtpStatus) -> InputOtpStyle + 'a>;

/// Builder-first one-time-password input styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]; pass `&theme` into
/// every control — style packs (Vega, Nova, …) live on the app's [`Theme`],
/// not on this builder. Click the control (or focus it through
/// [`Self::id`]) and type: accepted characters fill the slots left to
/// right, Backspace clears the last one (Ctrl clears everything), and
/// Ctrl+V distributes the clipboard across the remaining slots.
///
/// [`Self::style_override`] only patches the resolved [`InputOtpStyle`]
/// (colors, ring, radius). It is not [`shadcn_common::StyleId`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{InputOtp, InputOtpPattern, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     PinChanged(String),
/// }
///
/// fn pin<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
///     InputOtp::new(theme)
///         .value(value)
///         .max_length(4)
///         .pattern(InputOtpPattern::Digits)
///         .on_input(Message::PinChanged)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct InputOtp<'a, Message> {
    theme: &'a Theme,
    value: Fragment<'a>,
    max_length: usize,
    groups: Vec<usize>,
    pattern: InputOtpPattern,
    radius: Option<InputOtpRadius>,
    /// `None` = theme ring; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    slot_size: Option<f32>,
    text_size: Option<f32>,
    disabled: bool,
    invalid: bool,
    id: Option<widget::Id>,
    on_input: Option<OnInput<'a, Message>>,
    on_complete: Option<OnComplete<'a, Message>>,
    on_submit: Option<Message>,
    paste_transformer: Option<PasteTransformer<'a>>,
    style_override: Option<StyleOverride<'a>>,
}

impl<Message> fmt::Debug for InputOtp<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InputOtp")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("max_length", &self.max_length)
            .field("groups", &self.groups)
            .field("pattern", &self.pattern)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("slot_size", &self.slot_size)
            .field("text_size", &self.text_size)
            .field("disabled", &self.disabled)
            .field("invalid", &self.invalid)
            .field("id", &self.id)
            .field("on_input", &self.on_input.is_some())
            .field("on_complete", &self.on_complete.is_some())
            .field("on_submit", &self.on_submit.is_some())
            .field("paste_transformer", &self.paste_transformer.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> InputOtp<'a, Message> {
    /// Creates an empty six-slot OTP input.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{InputOtp, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let otp = InputOtp::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: Fragment::default(),
            max_length: DEFAULT_MAX_LENGTH,
            groups: Vec::new(),
            pattern: InputOtpPattern::default(),
            radius: None,
            color: None,
            slot_size: None,
            text_size: None,
            disabled: false,
            invalid: false,
            id: None,
            on_input: None,
            on_complete: None,
            on_submit: None,
            paste_transformer: None,
            style_override: None,
        }
    }

    /// Sets the controlled value shown in the slots.
    ///
    /// The application owns the text: store the [`String`] in state and
    /// feed it back on every [`Self::on_input`] message. Characters beyond
    /// [`Self::max_length`] are not displayed.
    pub fn value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.value = value.into_fragment();
        self
    }

    /// Sets the number of slots (web `maxlength`, clamped to at least 1).
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length.max(1);
        self
    }

    /// Splits the slots into visual groups with a minus separator between
    /// them, replacing the web `InputOTP.Group` / `InputOTP.Separator`
    /// markup (`groups([3, 3])` is the shadcn demo layout).
    ///
    /// Group sizes are normalized against [`Self::max_length`]: zero-sized
    /// groups are dropped, oversized layouts are truncated, and leftover
    /// slots become a trailing group. Without groups every slot shares one
    /// group, like the web `pattern` example.
    pub fn groups(mut self, groups: impl IntoIterator<Item = usize>) -> Self {
        self.groups = groups.into_iter().collect();
        self
    }

    /// Restricts accepted characters (web `pattern={REGEXP_ONLY_*}`).
    pub fn pattern(mut self, pattern: InputOtpPattern) -> Self {
        self.pattern = pattern;
        self
    }

    /// Sets the outer corner radius of each group.
    ///
    /// Without an explicit radius the active style pack decides
    /// (`rounded-md` on Vega, pill on Maia/Luma, square on Lyra/Sera, …).
    pub fn radius(mut self, radius: InputOtpRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the active border and ring.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`InputOtp::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Overrides the square slot side in pixels (iced extension; the web
    /// slot size is fixed per pack: `size-9` on Vega, `size-8` on Nova, …).
    pub fn slot_size(mut self, slot_size: impl Into<Pixels>) -> Self {
        self.slot_size = Some(slot_size.into().0.max(1.0));
        self
    }

    /// Sets the character text size. The pack's `.cn-input-otp-slot` size
    /// is used by default (`text-sm` on Vega, `text-xs` on Lyra/Mira).
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into().0);
        self
    }

    /// Disables the input (`disabled` attribute: no edits, 50% opacity,
    /// not-allowed cursor).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Marks the value as invalid (`aria-invalid`): borders turn
    /// `destructive` and the active ring uses the destructive halo.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Sets the widget id, enabling focus management via
    /// [`iced_core::widget::operation::focusable`].
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the callback receiving the edited value.
    ///
    /// Without it (or when [`Self::disabled`] is set) the control rejects
    /// focus and edits, matching the iced `text_input` contract.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets or clears the callback receiving the edited value.
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_input = on_input.map(|on_input| Box::new(on_input) as _);
        self
    }

    /// Sets the callback fired when every slot is filled (web
    /// `onComplete`), after the matching [`Self::on_input`] message.
    pub fn on_complete(mut self, on_complete: impl Fn(String) -> Message + 'a) -> Self {
        self.on_complete = Some(Box::new(on_complete));
        self
    }

    /// Sets or clears the callback fired when every slot is filled.
    pub fn on_complete_maybe(
        mut self,
        on_complete: Option<impl Fn(String) -> Message + 'a>,
    ) -> Self {
        self.on_complete = on_complete.map(|on_complete| Box::new(on_complete) as _);
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

    /// Rewrites clipboard text before it is matched against the pattern
    /// (web `pasteTransformer`, e.g. stripping dashes from `123-456`).
    pub fn paste_transformer(mut self, paste_transformer: impl Fn(String) -> String + 'a) -> Self {
        self.paste_transformer = Some(Box::new(paste_transformer));
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(InputOtpStyle, InputOtpStatus) -> InputOtpStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build(self)
    }
}

/// Convenience wrapper mirroring the shape of [`crate::input()`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, input_otp};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     CodeChanged(String),
/// }
///
/// fn code<'a>(theme: &'a Theme, value: &'a str) -> Element<'a, Message> {
///     input_otp(value, theme).on_input(Message::CodeChanged).into()
/// }
/// ```
pub fn input_otp<'a, Message>(
    value: impl IntoFragment<'a>,
    theme: &'a Theme,
) -> InputOtp<'a, Message> {
    InputOtp::new(theme).value(value)
}

impl<'a, Message> From<InputOtp<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(otp: InputOtp<'a, Message>) -> Self {
        otp.into_element()
    }
}
