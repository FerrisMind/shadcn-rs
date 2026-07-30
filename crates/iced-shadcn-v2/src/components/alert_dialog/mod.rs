//! Builder-first alert-dialog component.
//!
//! Port of the shadcn-svelte alert dialog (`AlertDialog.Root` / `Trigger` /
//! `Portal` / `Overlay` / `Content` + `Header` / `Footer` / `Title` /
//! `Description` / `Media` / `Action` / `Cancel`, the bits-ui alert dialog)
//! as a single iced builder: the trigger element is wrapped by a custom
//! widget that opens a modal surface centered over a dimmed backdrop.
//! Unlike the regular dialog, the alert dialog interrupts the user with an
//! expected response: it has no close button, ignores backdrop clicks by
//! default (`interactOutsideBehavior: "ignore"`), and closes through its
//! footer [`AlertDialogAction`] / [`AlertDialogCancel`] buttons or
//! <kbd>Esc</kbd>. The public API lives in this module; widget/overlay
//! internals live in focused private submodules.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::AlertDialogStyle;
pub use types::AlertDialogSize;

use std::fmt;

use shadcn_common::{DIALOG_ANIMATION_MS, DIALOG_MARGIN_PX, TypeRecipe};

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{column, container, row};
use crate::iced_compat::{
    Background, Border, Color, Element, Length, Padding, Pixels, alignment, time::Duration,
};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Builder-first alert dialog styled directly with iced types.
///
/// Wraps a trigger element and opens a modal surface on click, matching
/// shadcn-svelte defaults: the surface is centered (`top-1/2 left-1/2`),
/// capped at `data-[size=default]:sm:max-w-lg` / `data-[size=sm]:max-w-xs`
/// with a 16 px window margin, paints the `bg-popover` /
/// `text-popover-foreground` pair with a `ring-1 ring-foreground/N`
/// hairline over a `bg-black/N` backdrop, and animates with the web
/// `fade-in-0 zoom-in-95` entrance (`duration-100`).
///
/// Alert-dialog specifics versus [`crate::Dialog`]: there is no top-right
/// close button, backdrop clicks are ignored by default
/// (`interactOutsideBehavior: "ignore"`), and the footer
/// [`AlertDialogAction`] / [`AlertDialogCancel`] buttons dismiss the
/// dialog on click, publishing both the button's own message and
/// `onOpenChange(false)`.
///
/// While the dialog is open the window behind it is inert: pointer,
/// scroll, and keyboard events never reach the underlying widgets,
/// mirroring the web modal focus containment. Content stays fully
/// interactive.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogDescription,
///     AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, Button, ButtonVariant, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
///     Confirmed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     AlertDialog::new(
///         Button::text("Show Dialog", theme)
///             .variant(ButtonVariant::Outline)
///             .on_press(Message::Pressed),
///         AlertDialogHeader::new(theme)
///             .title(AlertDialogTitle::text("Are you absolutely sure?", theme))
///             .description(AlertDialogDescription::text(
///                 "This action cannot be undone.",
///                 theme,
///             )),
///         theme,
///     )
///     .footer(
///         AlertDialogFooter::new(theme)
///             .cancel(AlertDialogCancel::text("Cancel", theme))
///             .action(AlertDialogAction::text("Continue", theme).on_press(Message::Confirmed)),
///     )
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertDialog<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    footer: Option<AlertDialogFooter<'a, Message>>,
    theme: &'a Theme,
    size: AlertDialogSize,
    max_width: Option<f32>,
    duration: Duration,
    animated: bool,
    disabled: bool,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_open_change_complete: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    style_override: Option<Box<dyn Fn(AlertDialogStyle) -> AlertDialogStyle + 'a>>,
}

enum AlertDialogText<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for AlertDialog<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDialog")
            .field("footer", &self.footer.is_some())
            .field("theme", &self.theme)
            .field("size", &self.size)
            .field("max_width", &self.max_width)
            .field("duration", &self.duration)
            .field("animated", &self.animated)
            .field("disabled", &self.disabled)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field(
                "on_open_change_complete",
                &self.on_open_change_complete.is_some(),
            )
            .field("close_on_click_outside", &self.close_on_click_outside)
            .field("close_on_escape", &self.close_on_escape)
            .field("style_override", &self.style_override.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> AlertDialog<'a, Message> {
    /// Creates an alert dialog opening `content` over `trigger`.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`. Custom content that sets its
    /// own text colors opts out of the fade-in of the default text color.
    pub fn new(
        trigger: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self {
            trigger: trigger.into(),
            content: content.into(),
            footer: None,
            theme,
            size: AlertDialogSize::Default,
            max_width: None,
            duration: Duration::from_millis(DIALOG_ANIMATION_MS),
            animated: true,
            disabled: false,
            open: None,
            default_open: false,
            on_open_change: None,
            on_open_change_complete: None,
            close_on_click_outside: false,
            close_on_escape: true,
            style_override: None,
        }
    }

    /// Appends the footer slot (`AlertDialog.Footer`): a right-aligned
    /// action row at [`AlertDialogSize::Default`], or the two-column grid
    /// (`grid-cols-2`) at [`AlertDialogSize::Sm`].
    pub fn footer(mut self, footer: AlertDialogFooter<'a, Message>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Sets the surface size (`size` on `AlertDialog.Content`): the width
    /// cap, the footer layout, and — when the same size is passed to
    /// [`AlertDialogHeader::size`] — the header alignment.
    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }

    /// Overrides the maximum surface width in px
    /// (`sm:max-w-lg` — 512 px — by default; compact packs use
    /// `sm:max-w-sm`, and [`AlertDialogSize::Sm`] caps at `max-w-xs`).
    ///
    /// Mirrors passing a `sm:max-w-[N]` class to `AlertDialog.Content`.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Sets the duration of the open/close animation (`duration-100`).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Enables or disables the open/close animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Prevents the dialog from opening while keeping the trigger active.
    ///
    /// An already open dialog closes when it becomes disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Controls the open state explicitly instead of following trigger
    /// clicks (`bind:open`). Combine with [`Self::on_open_change`] to
    /// observe open and dismiss requests.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`, follows trigger clicks when
    /// `None`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Opens the dialog on first mount when uncontrolled (`defaultOpen`).
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Notifies about open-state change requests (`onOpenChange`): trigger
    /// clicks, action/cancel presses, and <kbd>Esc</kbd>.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Notifies once the open/close animation settles
    /// (`onOpenChangeComplete`), with the state that was reached.
    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool) -> Message + 'a,
    ) -> Self {
        self.on_open_change_complete = Some(Box::new(on_open_change_complete));
        self
    }

    /// Dismisses the dialog on backdrop clicks
    /// (`interactOutsideBehavior: "close"`). The alert dialog ignores
    /// outside interactions by default, unlike the regular dialog.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Keeps the dialog open on <kbd>Esc</kbd>
    /// (`escapeKeydownBehavior: "ignore"`).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Patches the resolved [`AlertDialogStyle`] (backdrop, colors, ring,
    /// radius, shadow) after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(AlertDialogStyle) -> AlertDialogStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<AlertDialog<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(dialog: AlertDialog<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(dialog.theme);
        let mut resolved = style::resolve_style(dialog.theme);

        if let Some(style_override) = dialog.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let max_width = dialog.max_width.unwrap_or(match dialog.size {
            AlertDialogSize::Default => recipe.max_width_px,
            AlertDialogSize::Sm => recipe.max_width_sm_px,
        });

        let footer = dialog
            .footer
            .map(|footer| {
                let full_width = dialog.size == AlertDialogSize::Sm;
                let spacing = footer.spacing.unwrap_or(recipe.footer_gap_px);
                let children = footer
                    .items
                    .into_iter()
                    .map(|item| item.into_child(dialog.on_open_change.as_deref(), full_width))
                    .collect::<Vec<_>>();

                (children, spacing)
            })
            .unwrap_or_default();

        Element::new(render::AlertDialogWidget {
            trigger: dialog.trigger,
            surface: container(dialog.content)
                .width(Length::Fill)
                .padding(if footer.0.is_empty() {
                    Padding::new(recipe.pad_px)
                } else {
                    Padding::new(recipe.pad_px).bottom(0)
                })
                .into(),
            footer: footer.0,
            footer_gap: footer.1,
            size: dialog.size,
            max_width,
            margin: DIALOG_MARGIN_PX,
            pad: recipe.pad_px,
            gap: recipe.gap_px,
            duration: dialog.duration,
            animated: dialog.animated,
            disabled: dialog.disabled,
            open_override: dialog.open,
            default_open: dialog.default_open,
            on_open_change: dialog.on_open_change,
            on_open_change_complete: dialog.on_open_change_complete,
            close_on_click_outside: dialog.close_on_click_outside,
            close_on_escape: dialog.close_on_escape,
            style: resolved,
        })
    }
}

/// A converted footer button plus its dismissal wiring.
pub(super) struct FooterChild<'a, Message> {
    pub(super) element: Element<'a, Message>,
    /// Whether a click on this child dismisses the dialog
    /// (`AlertDialog.Action` / `AlertDialog.Cancel`).
    pub(super) dismisses: bool,
    /// Whether the child's own press message already carries
    /// `onOpenChange(false)`, so the overlay must not publish it twice.
    pub(super) publishes_open_change: bool,
}

/// Styled alert-dialog header (`.cn-alert-dialog-header`): the media /
/// title / description grid.
///
/// At [`AlertDialogSize::Default`] the media box sits to the left of a
/// left-aligned text column (`sm:place-items-start sm:text-left`, media
/// `row-span-2`); at [`AlertDialogSize::Sm`] everything stacks centered
/// (`place-items-center text-center`). Pass the same size given to
/// [`AlertDialog::size`].
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertDialogHeader<'a, Message> {
    theme: &'a Theme,
    media: Option<AlertDialogMedia<'a, Message>>,
    title: Option<AlertDialogTitle<'a, Message>>,
    description: Option<AlertDialogDescription<'a, Message>>,
    children: Vec<Element<'a, Message>>,
    size: AlertDialogSize,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for AlertDialogHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDialogHeader")
            .field("theme", &self.theme)
            .field("media", &self.media.is_some())
            .field("title", &self.title.is_some())
            .field("description", &self.description.is_some())
            .field("children", &self.children.len())
            .field("size", &self.size)
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> AlertDialogHeader<'a, Message> {
    /// Creates an empty header.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            media: None,
            title: None,
            description: None,
            children: Vec::new(),
            size: AlertDialogSize::Default,
            spacing: None,
        }
    }

    /// Sets the media slot (`AlertDialog.Media`).
    pub fn media(mut self, media: AlertDialogMedia<'a, Message>) -> Self {
        self.media = Some(media);
        self
    }

    /// Sets the title slot.
    pub fn title(mut self, title: AlertDialogTitle<'a, Message>) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the description slot.
    pub fn description(mut self, description: AlertDialogDescription<'a, Message>) -> Self {
        self.description = Some(description);
        self
    }

    /// Appends arbitrary header content below the description.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Matches the header layout to the surface size passed to
    /// [`AlertDialog::size`].
    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }

    /// Overrides the gap between header rows (`gap-1.5` / `gap-2` /
    /// `gap-1` by default).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> From<AlertDialogHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: AlertDialogHeader<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(header.theme);
        let spacing = header.spacing.unwrap_or(recipe.header_gap_px);
        let centered = header.size == AlertDialogSize::Sm;

        let mut text_column = column(None).width(Length::Fill).spacing(spacing);

        if let Some(title) = header.title {
            text_column = text_column.push(title.into_element(centered));
        }

        if let Some(description) = header.description {
            text_column = text_column.push(description.into_element(centered));
        }

        for child in header.children {
            text_column = text_column.push(child);
        }

        match (header.media, header.size) {
            // `sm:grid-rows-[auto_1fr]` with the media spanning both rows:
            // the media box left, the text column right, top-aligned.
            (Some(media), AlertDialogSize::Default) => row([media.into(), text_column.into()])
                .spacing(recipe.media_gap_x_px)
                .align_y(alignment::Vertical::Top)
                .width(Length::Fill)
                .into(),
            // Stacked centered layout: media (`mb-2`), title, description.
            (Some(media), AlertDialogSize::Sm) => {
                let media = container(Element::from(media))
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center)
                    .padding(Padding::ZERO.bottom(recipe.media_margin_bottom_px));

                column([media.into(), text_column.into()])
                    .spacing(spacing)
                    .width(Length::Fill)
                    .into()
            }
            (None, _) => text_column.into(),
        }
    }
}

/// Styled alert-dialog footer (`.cn-alert-dialog-footer`): holds the
/// [`AlertDialogCancel`] / [`AlertDialogAction`] buttons and arbitrary
/// extra elements, laid out by [`AlertDialog`] as a right-aligned row
/// (`sm:flex-row sm:justify-end gap-2`) or, at [`AlertDialogSize::Sm`],
/// as a two-column grid (`grid grid-cols-2`).
#[must_use = "builders do nothing unless handed to AlertDialog::footer"]
pub struct AlertDialogFooter<'a, Message> {
    theme: &'a Theme,
    items: Vec<AlertDialogFooterItem<'a, Message>>,
    spacing: Option<f32>,
}

enum AlertDialogFooterItem<'a, Message> {
    Action(AlertDialogAction<'a, Message>),
    Cancel(AlertDialogCancel<'a, Message>),
    Custom(Element<'a, Message>),
}

impl<Message> fmt::Debug for AlertDialogFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDialogFooter")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> AlertDialogFooter<'a, Message> {
    /// Creates an empty footer.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            spacing: None,
        }
    }

    /// Appends a cancel button; a click dismisses the dialog.
    pub fn cancel(mut self, cancel: AlertDialogCancel<'a, Message>) -> Self {
        self.items.push(AlertDialogFooterItem::Cancel(cancel));
        self
    }

    /// Appends an action button; a click dismisses the dialog.
    pub fn action(mut self, action: AlertDialogAction<'a, Message>) -> Self {
        self.items.push(AlertDialogFooterItem::Action(action));
        self
    }

    /// Appends arbitrary footer content that does not dismiss the dialog.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(AlertDialogFooterItem::Custom(child.into()));
        self
    }

    /// Overrides the gap between footer items (`gap-2` by default).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> AlertDialogFooterItem<'a, Message>
where
    Message: Clone + 'a,
{
    /// Converts the footer item into an element plus dismissal wiring.
    ///
    /// Action/cancel buttons without their own `on_press` fall back to
    /// publishing `on_open_change(false)` so they render enabled and the
    /// app still observes the dismissal exactly once.
    fn into_child(
        self,
        on_open_change: Option<&(dyn Fn(bool) -> Message + 'a)>,
        full_width: bool,
    ) -> FooterChild<'a, Message> {
        match self {
            Self::Action(action) => action.0.into_child(on_open_change, full_width),
            Self::Cancel(cancel) => cancel.0.into_child(on_open_change, full_width),
            Self::Custom(element) => FooterChild {
                element,
                dismisses: false,
                publishes_open_change: false,
            },
        }
    }
}

/// Shared internals of [`AlertDialogAction`] and [`AlertDialogCancel`].
struct FooterButton<'a, Message> {
    content: AlertDialogText<'a, Message>,
    theme: &'a Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    on_press: Option<Message>,
}

impl<'a, Message> FooterButton<'a, Message> {
    fn text(label: impl IntoFragment<'a>, theme: &'a Theme, variant: ButtonVariant) -> Self {
        Self {
            content: AlertDialogText::Label(label.into_fragment()),
            theme,
            variant,
            size: ButtonSize::Default,
            disabled: false,
            on_press: None,
        }
    }

    fn from_element(
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
        variant: ButtonVariant,
    ) -> Self {
        Self {
            content: AlertDialogText::Element(content.into()),
            theme,
            variant,
            size: ButtonSize::Default,
            disabled: false,
            on_press: None,
        }
    }

    fn into_child(
        self,
        on_open_change: Option<&(dyn Fn(bool) -> Message + 'a)>,
        full_width: bool,
    ) -> FooterChild<'a, Message>
    where
        Message: Clone + 'a,
    {
        let fallback = if self.on_press.is_none() {
            on_open_change.map(|on_open_change| on_open_change(false))
        } else {
            None
        };
        let publishes_open_change = fallback.is_some();

        let mut button = match self.content {
            AlertDialogText::Label(label) => Button::text(label, self.theme),
            AlertDialogText::Element(element) => Button::new(element, self.theme),
        }
        .variant(self.variant)
        .size(self.size)
        .disabled(self.disabled)
        .on_press_maybe(self.on_press.or(fallback));

        if full_width {
            button = button.full_width();
        }

        FooterChild {
            element: button.into(),
            dismisses: true,
            publishes_open_change,
        }
    }

    fn debug(&self, name: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(name)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .finish_non_exhaustive()
    }
}

/// The confirming footer button (`AlertDialog.Action`): a
/// [`ButtonVariant::Default`] button whose click dismisses the dialog.
///
/// The dismissal publishes `onOpenChange(false)` alongside the button's
/// own [`Self::on_press`] message. Without an `on_press`, the button
/// publishes `onOpenChange(false)` itself (attach
/// [`AlertDialog::on_open_change`], otherwise the button renders
/// disabled, matching a message-less iced button).
#[must_use = "builders do nothing unless handed to AlertDialogFooter"]
pub struct AlertDialogAction<'a, Message>(FooterButton<'a, Message>);

impl<Message> fmt::Debug for AlertDialogAction<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.debug("AlertDialogAction", formatter)
    }
}

impl<'a, Message> AlertDialogAction<'a, Message> {
    /// Creates a text action button.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self(FooterButton::text(label, theme, ButtonVariant::Default))
    }

    /// Creates an action button from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self(FooterButton::from_element(
            content,
            theme,
            ButtonVariant::Default,
        ))
    }

    /// Overrides the button variant (`buttonVariants({ variant })`,
    /// `default` for actions).
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.0.variant = variant;
        self
    }

    /// Overrides the button size (`buttonVariants({ size })`).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.0.size = size;
        self
    }

    /// Disables the button; it neither fires nor dismisses.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;
        self
    }

    /// Sets the message published when the action is confirmed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.0.on_press = Some(message);
        self
    }
}

/// The dismissing footer button (`AlertDialog.Cancel`): a
/// [`ButtonVariant::Outline`] button whose click dismisses the dialog.
///
/// The dismissal publishes `onOpenChange(false)` alongside the button's
/// own [`Self::on_press`] message. Without an `on_press`, the button
/// publishes `onOpenChange(false)` itself (attach
/// [`AlertDialog::on_open_change`], otherwise the button renders
/// disabled, matching a message-less iced button).
#[must_use = "builders do nothing unless handed to AlertDialogFooter"]
pub struct AlertDialogCancel<'a, Message>(FooterButton<'a, Message>);

impl<Message> fmt::Debug for AlertDialogCancel<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.debug("AlertDialogCancel", formatter)
    }
}

impl<'a, Message> AlertDialogCancel<'a, Message> {
    /// Creates a text cancel button.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self(FooterButton::text(label, theme, ButtonVariant::Outline))
    }

    /// Creates a cancel button from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self(FooterButton::from_element(
            content,
            theme,
            ButtonVariant::Outline,
        ))
    }

    /// Overrides the button variant (`buttonVariants({ variant })`,
    /// `outline` for cancel).
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.0.variant = variant;
        self
    }

    /// Overrides the button size (`buttonVariants({ size })`).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.0.size = size;
        self
    }

    /// Disables the button (`disabled`); it neither fires nor dismisses.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;
        self
    }

    /// Sets the message published when the dialog is cancelled.
    pub fn on_press(mut self, message: Message) -> Self {
        self.0.on_press = Some(message);
        self
    }
}

/// Styled alert-dialog media box (`.cn-alert-dialog-media`): a square
/// `bg-muted` well for an icon or illustration, `size-16 rounded-md` on
/// Vega and pack-specific sizes/radii elsewhere.
///
/// Size the glyph inside with [`Self::icon_px`]
/// (`*:[svg:not([class*='size-'])]:size-8`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertDialogMedia<'a, Message> {
    content: Element<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for AlertDialogMedia<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDialogMedia")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> AlertDialogMedia<'a, Message> {
    /// Creates a media box around `content`, typically an icon.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: content.into(),
            theme,
        }
    }

    /// The glyph size the active style pack expects inside the media box.
    #[must_use]
    pub fn icon_px(theme: &Theme) -> f32 {
        style::recipe(theme).media_icon_px
    }
}

impl<'a, Message> From<AlertDialogMedia<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(media: AlertDialogMedia<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(media.theme);
        let resolved = style::resolve_style(media.theme);
        let background = resolved.media_background;
        let radius = resolved.media_radius;

        container(media.content)
            .width(Length::Fixed(recipe.media_size_px))
            .height(Length::Fixed(recipe.media_size_px))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .style(move |_| container::Style {
                background: Some(Background::Color(background)),
                border: Border {
                    radius: radius.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            })
            .into()
    }
}

/// Styled alert-dialog title (`.cn-alert-dialog-title`): heading font,
/// style-pack weight and size, inheriting the dialog foreground color.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertDialogTitle<'a, Message> {
    content: AlertDialogText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for AlertDialogTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDialogTitle")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> AlertDialogTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: AlertDialogText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: AlertDialogText::Label(label.into_fragment()),
            theme,
        }
    }

    fn into_element(self, centered: bool) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let recipe = style::recipe(self.theme);

        typeset(self.content, recipe.title, self.theme, true, None, centered)
    }
}

impl<'a, Message> From<AlertDialogTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: AlertDialogTitle<'a, Message>) -> Element<'a, Message> {
        title.into_element(false)
    }
}

/// Styled alert-dialog description (`.cn-alert-dialog-description`):
/// `text-muted-foreground` body copy.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct AlertDialogDescription<'a, Message> {
    content: AlertDialogText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for AlertDialogDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AlertDialogDescription")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> AlertDialogDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: AlertDialogText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: AlertDialogText::Label(label.into_fragment()),
            theme,
        }
    }

    fn into_element(self, centered: bool) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let recipe = style::recipe(self.theme);
        let muted = self.theme.palette.muted_foreground;

        let element = typeset(
            self.content,
            recipe.description,
            self.theme,
            false,
            Some(muted),
            centered,
        );

        if recipe.description_margin_top_px > 0.0 {
            container(element)
                .width(Length::Fill)
                .padding(Padding::ZERO.top(recipe.description_margin_top_px))
                .into()
        } else {
            element
        }
    }
}

impl<'a, Message> From<AlertDialogDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: AlertDialogDescription<'a, Message>) -> Element<'a, Message> {
        description.into_element(false)
    }
}

/// Typesets a text slot with a [`TypeRecipe`]; element content is only
/// re-aligned, never re-styled.
fn typeset<'a, Message: 'a>(
    content: AlertDialogText<'a, Message>,
    recipe: TypeRecipe,
    theme: &Theme,
    heading: bool,
    color: Option<Color>,
    centered: bool,
) -> Element<'a, Message> {
    match content {
        AlertDialogText::Label(label) => {
            let pack = theme.font_pack();
            let mut font = iced_font(if heading { pack.heading } else { pack.sans });
            font.weight = iced_font_weight(recipe.weight);

            let label = if recipe.uppercase {
                label.into_owned().to_uppercase().into()
            } else {
                label
            };

            let mut widget = crate::iced_compat::widget::text(label)
                .size(recipe.size_px)
                .line_height(LineHeight::Absolute(Pixels(recipe.line_height_px)))
                .font(font);

            if let Some(color) = color {
                widget = widget.color(color);
            }

            if centered {
                widget = widget
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center);
            }

            widget.into()
        }
        AlertDialogText::Element(element) if centered => container(element)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .into(),
        AlertDialogText::Element(element) => element,
    }
}
