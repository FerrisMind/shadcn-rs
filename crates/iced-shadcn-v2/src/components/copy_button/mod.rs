//! Copy-to-clipboard button for `iced-shadcn-v2`.
//!
//! The component follows the shadcn-svelte extra closely: it defaults to a
//! ghost icon button, swaps Copy/Check/X icons for idle/success/failure, lets
//! callers provide an idle icon and trailing content, and animates each
//! built-in icon from `0.85` to `1.0` over [`CopyButton::animation_duration`].
//!
//! Clipboard ownership is intentionally application-side. The iced clipboard
//! API is a command without a success result, so the button emits the action
//! configured with [`CopyButton::on_copy`], while the application feeds the
//! resulting [`CopyButtonStatus`] back through [`CopyButton::status`].
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{CopyButton, CopyButtonAction, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Copy(CopyButtonAction),
//! }
//!
//! fn view(theme: &Theme) -> Element<'_, Message> {
//!     CopyButton::new("Hello, World!", theme)
//!         .content(iced::widget::text("Copy text"))
//!         .on_copy(Message::Copy(CopyButtonAction::Pressed))
//!         .into()
//! }
//! ```

mod icon;
mod render;
#[cfg(test)]
mod tests;
mod types;

pub use types::{
    CopyButtonAction, CopyButtonState, CopyButtonStatus, CopyButtonUpdate, copy_button_reduce,
};

/// The built-in Copy/Check/X icon program, shared with the snippet overlay.
pub(crate) use self::icon::CopyButtonIcon;

use std::fmt;
use std::time::Duration;

use crate::components::button::{Button, ButtonRadius, ButtonSize, ButtonVariant};
use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::widget::{container, row};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;
use shadcn_common::AccentColor;

/// Content appended after the status icon.
enum CopyButtonContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

/// Message source invoked when the user presses a [`CopyButton`].
enum CopyButtonOnCopy<'a, Message> {
    Message(Message),
    Callback(Box<dyn Fn(CopyButtonAction) -> Message + 'a>),
}

/// Builder-first copy button with controlled feedback state.
///
/// `text` is retained by the builder so an application can use
/// [`Self::text`] when handling the press message. The component does not
/// write to the clipboard itself because `iced_core::Clipboard::write` cannot
/// report whether the platform accepted the write.
///
/// The default configuration is `Ghost` + `Icon`, matching the Svelte
/// component. Adding [`Self::label`] or [`Self::content`] automatically
/// promotes an icon size to `ButtonSize::Default`, matching its children
/// behavior on the web.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{CopyButton, CopyButtonAction, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Copy(CopyButtonAction),
/// }
///
/// fn copy_button(theme: &Theme) -> Element<'_, Message> {
///     CopyButton::new("text to copy", theme)
///         .label("Copy")
///         .on_copy(Message::Copy(CopyButtonAction::Pressed))
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct CopyButton<'a, Message> {
    text: String,
    theme: &'a Theme,
    idle_icon: Option<Element<'a, Message>>,
    content: Option<CopyButtonContent<'a, Message>>,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    color: Option<shadcn_common::AccentColor>,
    width: Length,
    height: Option<Length>,
    full_width: bool,
    status: CopyButtonStatus,
    animation_duration: Duration,
    disabled: bool,
    on_copy: Option<CopyButtonOnCopy<'a, Message>>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

impl<Message> fmt::Debug for CopyButton<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            None => "none",
            Some(CopyButtonContent::Label(_)) => "label",
            Some(CopyButtonContent::Element(_)) => "element",
        };

        formatter
            .debug_struct("CopyButton")
            .field("text_length", &self.text.len())
            .field("theme", &self.theme)
            .field("idle_icon", &self.idle_icon.is_some())
            .field("content", &content)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("full_width", &self.full_width)
            .field("status", &self.status)
            .field("animation_duration", &self.animation_duration)
            .field("disabled", &self.disabled)
            .field("on_copy", &self.on_copy.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CopyButton<'a, Message> {
    /// Creates a copy button for `text` using the supplied shadcn theme.
    #[must_use = "builder methods return the modified copy button"]
    pub fn new(text: impl Into<String>, theme: &'a Theme) -> Self {
        Self {
            text: text.into(),
            theme,
            idle_icon: None,
            content: None,
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            radius: None,
            color: None,
            width: Length::Shrink,
            height: None,
            full_width: false,
            status: CopyButtonStatus::Idle,
            animation_duration: Duration::from_millis(500),
            disabled: false,
            on_copy: None,
            style_override: None,
        }
    }

    /// Returns the text that the application should write to its clipboard.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets the visual button variant.
    #[must_use = "builder methods return the modified copy button"]
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the preset button size.
    #[must_use = "builder methods return the modified copy button"]
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button corner radius.
    #[must_use = "builder methods return the modified copy button"]
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color to the button's theme tokens.
    #[must_use = "builder methods return the modified copy button"]
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Clears a previously selected accent and uses the theme primary.
    #[must_use = "builder methods return the modified copy button"]
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self
    }

    /// Sets a custom button width.
    #[must_use = "builder methods return the modified copy button"]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom button height.
    #[must_use = "builder methods return the modified copy button"]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Makes the button fill the available width.
    #[must_use = "builder methods return the modified copy button"]
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Replaces the built-in idle Copy icon with arbitrary iced content.
    ///
    /// The built-in Check and X icons are still used for success and failure.
    #[must_use = "builder methods return the modified copy button"]
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.idle_icon = Some(icon.into());
        self
    }

    /// Appends a text label after the status icon.
    ///
    /// As in the Svelte component, any trailing content turns an icon size
    /// into the matching text-button size.
    #[must_use = "builder methods return the modified copy button"]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.content = Some(CopyButtonContent::Label(label.into_fragment()));
        self
    }

    /// Appends arbitrary iced content after the status icon.
    ///
    /// This is the Rust equivalent of the Svelte `children` snippet.
    #[must_use = "builder methods return the modified copy button"]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(CopyButtonContent::Element(content.into()));
        self
    }

    /// Alias for [`Self::content`] using the source component's terminology.
    #[must_use = "builder methods return the modified copy button"]
    pub fn children(self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content(content)
    }

    /// Sets the controlled feedback status.
    #[must_use = "builder methods return the modified copy button"]
    pub fn status(mut self, status: CopyButtonStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the complete controlled state.
    #[must_use = "builder methods return the modified copy button"]
    pub fn state(mut self, state: CopyButtonState) -> Self {
        self.status = state.status();
        self
    }

    /// Sets the icon entrance duration and the recommended feedback reset
    /// delay. `Duration::ZERO` disables the entrance animation.
    #[must_use = "builder methods return the modified copy button"]
    pub fn animation_duration(mut self, duration: Duration) -> Self {
        self.animation_duration = duration;
        self
    }

    /// Disables interaction while retaining the current visual content.
    #[must_use = "builder methods return the modified copy button"]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the button is pressed.
    ///
    /// The message represents [`CopyButtonAction::Pressed`]. The application
    /// performs the clipboard write and feeds the result back through
    /// [`Self::status`].
    #[must_use = "builder methods return the modified copy button"]
    pub fn on_copy(mut self, message: Message) -> Self {
        self.on_copy = Some(CopyButtonOnCopy::Message(message));
        self
    }

    /// Sets or clears the message emitted when the button is pressed.
    #[must_use = "builder methods return the modified copy button"]
    pub fn on_copy_maybe(mut self, message: Option<Message>) -> Self {
        self.on_copy = message.map(CopyButtonOnCopy::Message);
        self
    }

    /// Sets a callback that maps the press action to an application message.
    ///
    /// This is useful when the application uses one message enum for several
    /// copy buttons and wants to include the action in that message.
    #[must_use = "builder methods return the modified copy button"]
    pub fn on_copy_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(CopyButtonAction) -> Message + 'a,
    {
        self.on_copy = Some(CopyButtonOnCopy::Callback(Box::new(callback)));
        self
    }

    /// Alias for [`Self::on_copy`].
    #[must_use = "builder methods return the modified copy button"]
    pub fn on_press(self, message: Message) -> Self {
        self.on_copy(message)
    }

    /// Applies an iced-style escape hatch after button style resolution.
    #[must_use = "builder methods return the modified copy button"]
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying styled iced button.
    pub fn into_button(self) -> button_widget::Button<'a, Message>
    where
        Message: Clone + 'a,
    {
        let CopyButton {
            text: _,
            theme,
            idle_icon,
            content,
            variant,
            size,
            radius,
            color,
            width,
            height,
            full_width,
            status,
            animation_duration,
            disabled,
            on_copy,
            style_override,
        } = self;

        let size = if content.is_some() && size.is_icon() {
            ButtonSize::Default
        } else {
            size
        };
        let icon_size = render::icon_size(size, theme);
        let icon = match (status, idle_icon) {
            (CopyButtonStatus::Idle, Some(icon)) => icon,
            (status, _) => CopyButtonIcon {
                status,
                color: render::icon_color(theme, variant, color),
                hover_color: render::icon_hover_color(theme, variant, color),
                size: icon_size,
                animation_duration,
            }
            .element(),
        };
        let icon: Element<'a, Message> = container(icon)
            .width(Length::Fixed(icon_size))
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();

        let on_press = on_copy.map(|source| match source {
            CopyButtonOnCopy::Message(message) => message,
            CopyButtonOnCopy::Callback(callback) => callback(CopyButtonAction::Pressed),
        });

        let mut button = match content {
            None if size.is_icon() => Button::icon(icon, theme),
            None => Button::new(icon, theme),
            Some(CopyButtonContent::Label(label)) => Button::new(
                row![icon, render::label_element(label, size, theme)]
                    .spacing(render::content_gap())
                    .align_y(Vertical::Center),
                theme,
            ),
            Some(CopyButtonContent::Element(content)) => Button::new(
                row![icon, content]
                    .spacing(render::content_gap())
                    .align_y(Vertical::Center),
                theme,
            ),
        }
        .variant(variant)
        .size(size)
        .disabled(disabled)
        .on_press_maybe(on_press)
        .width(width);

        if full_width {
            button = button.full_width();
        }

        if let Some(radius) = radius {
            button = button.radius(radius);
        }
        if let Some(color) = color {
            button = button.color(color);
        }
        if let Some(height) = height {
            button = button.height(height);
        }
        if let Some(style_override) = style_override {
            button = button.style_override(style_override);
        }

        button.into_button()
    }

    /// Builds the button as an iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.into_button().into()
    }
}

impl<'a, Message: Clone + 'a> From<CopyButton<'a, Message>> for Element<'a, Message> {
    fn from(button: CopyButton<'a, Message>) -> Self {
        button.into_element()
    }
}
