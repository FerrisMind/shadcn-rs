//! Builder-first snippet component.
//!
//! Port of the shadcn-svelte-extra `Snippet`: a bordered, padded `<pre>`-style
//! frame for short shell commands with an absolutely positioned copy button.
//! The reference markup (`relative w-full max-w-full rounded-md border py-2.5
//! pr-12 pl-3`, mono `text-sm font-light whitespace-nowrap` rows, `size-7`
//! ghost copy button) is expressed through [`Snippet::variant`],
//! [`Snippet::radius`], [`Snippet::width`], [`Snippet::max_width`], and
//! [`Snippet::height`].
//!
//! Clipboard access is application-side in iced, so the copy button is a
//! controlled component, exactly like [`super::CopyButton`]: the application
//! feeds back the outcome through [`Snippet::copy_status`] and receives
//! [`CopyButtonAction`]s from [`Snippet::on_copy`]. Without a handler the
//! button is still rendered (matching the reference) but stays inactive.
//!
//! ```rust,no_run
//! use iced::{Element, Length};
//! use iced_shadcn_v2::{
//!     Snippet, SnippetVariant, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {}
//!
//! fn demo(theme: &Theme) -> Element<'_, Message> {
//!     Snippet::new("npx jsrepo add ui/snippet", theme)
//!         .variant(SnippetVariant::Secondary)
//!         .max_width(300.0)
//!         .into()
//! }
//!
//! fn multiline<'a>(theme: &'a Theme) -> Element<'a, Message> {
//!     Snippet::lines(["npx jsrepo add", "npx jsrepo add ui/snippet"], theme)
//!         .max_width(300.0)
//!         .into()
//! }
//! ```

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{SnippetRadius, SnippetText, SnippetVariant};

use std::fmt;
use std::time::Duration;

use crate::components::copy_button::CopyButtonAction;
use crate::components::scroll_area::ScrollArea;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, Length, Padding};
use crate::theme::Theme;

use super::copy_button::CopyButtonStatus;
use render::SnippetView;

/// How the copy action is wired to the application.
enum SnippetOnCopy<'a, Message> {
    /// A fixed message published on press.
    Message(Message),
    /// A callback producing the message from the [`CopyButtonAction`].
    Callback(Box<dyn Fn(CopyButtonAction) -> Message + 'a>),
}

/// Builder-first code snippet frame with a floating copy button.
///
/// ```rust,no_run
/// use iced::{Element, Length};
/// use iced_shadcn_v2::{Snippet, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {}
///
/// fn usage<'a>(theme: &'a Theme) -> Element<'a, Message> {
///     Snippet::new("npx jsrepo add ui/snippet", theme)
///         .max_width(300.0)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Snippet<'a, Message> {
    text: SnippetText,
    theme: &'a Theme,
    variant: SnippetVariant,
    radius: SnippetRadius,
    width: Length,
    height: Option<Length>,
    max_width: Option<f32>,
    copy_status: CopyButtonStatus,
    copy_animation_duration: Duration,
    on_copy: Option<SnippetOnCopy<'a, Message>>,
}

impl<Message> fmt::Debug for Snippet<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Snippet")
            .field("text", &self.text)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("max_width", &self.max_width)
            .field("copy_status", &self.copy_status)
            .field("copy_animation_duration", &self.copy_animation_duration)
            .field("on_copy", &self.on_copy.as_ref().map(|_| "<closure>"))
            .finish()
    }
}

impl<'a, Message> Snippet<'a, Message> {
    /// Creates a snippet from a single text (`text: string` prop). Embedded
    /// `\n` sequences split into separate rows; the copy action receives the
    /// raw string.
    pub fn new(text: impl Into<SnippetText>, theme: &'a Theme) -> Self {
        Self {
            text: text.into(),
            theme,
            variant: SnippetVariant::Default,
            radius: SnippetRadius::Medium,
            width: Length::Fill,
            height: None,
            max_width: None,
            copy_status: CopyButtonStatus::Idle,
            copy_animation_duration: Duration::from_millis(500),
            on_copy: None,
        }
    }

    /// Creates a snippet from a list of lines (`text: string[]` prop): one
    /// row per entry, copied as `\n`-joined text.
    pub fn lines(lines: impl IntoIterator<Item = impl Into<String>>, theme: &'a Theme) -> Self {
        Self::new(
            SnippetText::Lines(lines.into_iter().map(Into::into).collect()),
            theme,
        )
    }

    /// Sets the visual variant (`bg-card` / `bg-accent` / `bg-destructive` /
    /// `bg-primary` surface).
    pub fn variant(mut self, variant: SnippetVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Overrides the corner radius (`rounded-md` by default).
    pub fn radius(mut self, radius: SnippetRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the frame width (`w-full` by default).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Caps the frame width (`max-w-[...]` from the reference `class` prop).
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Bounds the frame height and enables vertical scrolling
    /// (`overflow-y-auto` on the reference `<pre>`). Without a height the
    /// frame grows with its rows and never scrolls.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the feedback status of the copy button (see
    /// [`CopyButtonStatus`]).
    pub fn copy_status(mut self, status: CopyButtonStatus) -> Self {
        self.copy_status = status;
        self
    }

    /// Sets how long the success/failure icon animation runs (500 ms by
    /// default, matching the reference `animationDuration`).
    pub fn copy_animation_duration(mut self, duration: Duration) -> Self {
        self.copy_animation_duration = duration;
        self
    }

    /// Publishes `message` when the copy button is pressed.
    pub fn on_copy(mut self, message: Message) -> Self {
        self.on_copy = Some(SnippetOnCopy::Message(message));
        self
    }

    /// Publishes `message` when the copy button is pressed, or leaves the
    /// button inactive when `message` is `None`.
    pub fn on_copy_maybe(mut self, message: Option<Message>) -> Self {
        self.on_copy = message.map(SnippetOnCopy::Message);
        self
    }

    /// Maps each copy action to a message, enabling the full
    /// `Pressed → Success/Failure → Reset` cycle.
    pub fn on_copy_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(CopyButtonAction) -> Message + 'a,
    {
        self.on_copy = Some(SnippetOnCopy::Callback(Box::new(callback)));
        self
    }

    /// Builds the underlying iced element.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let theme = self.theme;
        let radius = render::radius_px(theme, self.radius);
        let frame = render::frame_style(theme, self.variant, radius);

        let recipe = theme.style.snippet();
        // iced's text rasterizer centers the line box, but the visible glyph
        // bounds sit a little below that box center. Keep the reference
        // frame height unchanged while compensating with a 2 px upward visual
        // offset for the snippet content.
        let padding = Padding::default()
            .top((recipe.pad_y_px - 2.0).max(0.0))
            .bottom(recipe.pad_y_px + 2.0)
            .left(recipe.pad_left_px)
            .right(recipe.pad_right_px);

        // `<pre>` content: rows, padded by the frame (`py-2.5 pr-12 pl-3`).
        // Built twice — once plain, once inside a ScrollArea — like the code
        // component's `build_pre` closure.
        let build_pre = || -> Element<'a, Message> {
            container(render::build_rows(&self.text, theme, frame.text_color))
                .width(Length::Fill)
                .padding(padding)
                .into()
        };
        let pre = build_pre();

        // Bounded height → the pre scrolls vertically (`overflow-y-auto`).
        let content: Element<'a, Message> = match self.height {
            Some(_) => container(
                ScrollArea::new(build_pre(), theme)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(padding)
            .into(),
            None => pre,
        };

        let on_copy = self.on_copy.map(|source| match source {
            SnippetOnCopy::Message(message) => message,
            SnippetOnCopy::Callback(callback) => callback(CopyButtonAction::Pressed),
        });

        let copy = render::build_copy_button(
            theme,
            self.copy_status,
            self.copy_animation_duration,
            on_copy,
            frame.text_color,
        );

        SnippetView {
            content,
            copy,
            copy_active: true,
            frame,
            width: self.width,
            height: self.height,
            max_width: self.max_width,
            copy_offset: recipe.copy_button_offset_px,
        }
        .into()
    }
}

impl<'a, Message> From<Snippet<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(snippet: Snippet<'a, Message>) -> Self {
        snippet.into_element()
    }
}
