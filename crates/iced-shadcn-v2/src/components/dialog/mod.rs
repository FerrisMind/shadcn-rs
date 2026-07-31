//! Builder-first dialog component.
//!
//! Port of the shadcn-svelte dialog (`Dialog.Root` / `Trigger` / `Portal` /
//! `Overlay` / `Content` + `Header` / `Footer` / `Title` / `Description` /
//! `Close`, the bits-ui modal) as a single iced builder: the trigger
//! element is wrapped by a custom widget that opens a modal surface
//! centered over a dimmed backdrop. While open the rest of the window is
//! inert; clicks on the backdrop and <kbd>Esc</kbd> dismiss the dialog,
//! and the built-in close button mirrors the web `ghost` / `icon-sm`
//! X button. The public API lives in this module; widget/overlay
//! internals live in focused private submodules.

mod close;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::DialogStyle;

use std::fmt;

use shadcn_common::{
    DIALOG_ANIMATION_MS, DIALOG_CLOSE_ICON_PX, DIALOG_CLOSE_SIZE_PX, DIALOG_MARGIN_PX, TypeRecipe,
};

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{Space, column, container, row};
use crate::iced_compat::{
    Background, Border, Element, Length, Padding, Pixels, alignment, border, time::Duration,
};

use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Builder-first dialog styled directly with iced types.
///
/// Wraps a trigger element and opens a modal surface on click, matching
/// shadcn-svelte defaults: the surface is centered (`top-1/2 left-1/2`),
/// capped at `sm:max-w-md` / `sm:max-w-sm` with a 16 px window margin
/// (`max-w-[calc(100%-2rem)]`), paints the `bg-popover` /
/// `text-popover-foreground` pair with a `ring-1 ring-foreground/N`
/// hairline over a `bg-black/N` backdrop, shows the ghost `icon-sm` close
/// button in the top-right corner, closes on backdrop clicks and
/// <kbd>Esc</kbd>, and animates with the web `fade-in-0 zoom-in-95`
/// entrance (`duration-100`).
///
/// While the dialog is open the window behind it is inert: pointer,
/// scroll, and keyboard events never reach the underlying widgets,
/// mirroring the web modal focus containment.
///
/// Content stays fully interactive — forms, buttons, and inputs inside
/// the surface receive events like any other widget.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     Button, ButtonVariant, Dialog, DialogDescription, DialogHeader, DialogTitle, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Dialog::new(
///         Button::text("Open Dialog", theme)
///             .variant(ButtonVariant::Outline)
///             .on_press(Message::Pressed),
///         DialogHeader::new(theme)
///             .title(DialogTitle::text("Edit profile", theme))
///             .description(DialogDescription::text(
///                 "Make changes to your profile here.",
///                 theme,
///             )),
///         theme,
///     )
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Dialog<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    footer: Option<DialogFooter<'a, Message>>,
    theme: &'a Theme,
    max_width: Option<f32>,
    /// Overrides recipe content padding (`p-0` for command dialog).
    content_padding: Option<f32>,
    /// When set, places the surface top edge at this fraction of the window
    /// height (`top-1/3` for command dialog) instead of vertical centering.
    vertical_anchor_top: Option<f32>,
    duration: Duration,
    animated: bool,
    disabled: bool,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    show_close_button: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
    modal: bool,
    style_override: Option<Box<dyn Fn(DialogStyle) -> DialogStyle + 'a>>,
}

enum DialogText<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Dialog<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Dialog")
            .field("footer", &self.footer.is_some())
            .field("theme", &self.theme)
            .field("max_width", &self.max_width)
            .field("content_padding", &self.content_padding)
            .field("vertical_anchor_top", &self.vertical_anchor_top)
            .field("duration", &self.duration)
            .field("animated", &self.animated)
            .field("disabled", &self.disabled)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("show_close_button", &self.show_close_button)
            .field("close_on_click_outside", &self.close_on_click_outside)
            .field("close_on_escape", &self.close_on_escape)
            .field("modal", &self.modal)
            .field("style_override", &self.style_override.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> Dialog<'a, Message> {
    /// Creates a dialog opening `content` over `trigger`.
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
            max_width: None,
            content_padding: None,
            vertical_anchor_top: None,
            duration: Duration::from_millis(DIALOG_ANIMATION_MS),
            animated: true,
            disabled: false,
            open: None,
            default_open: false,
            on_open_change: None,
            show_close_button: true,
            close_on_click_outside: true,
            close_on_escape: true,
            modal: true,
            style_override: None,
        }
    }

    /// Appends the footer slot (`Dialog.Footer`), laid out by the active
    /// style pack — a right-aligned action row, or the full-width muted
    /// bar of packs like Nova.
    pub fn footer(mut self, footer: DialogFooter<'a, Message>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Overrides the maximum surface width in px (`sm:max-w-md` — 448 px —
    /// by default; compact packs use `sm:max-w-sm`).
    ///
    /// Mirrors passing a `sm:max-w-[N]` class to `Dialog.Content`.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Overrides the content padding in px (`p-0` for command dialog).
    ///
    /// `None` (default) keeps the style-pack recipe padding.
    pub fn content_padding(mut self, padding: f32) -> Self {
        self.content_padding = Some(padding.max(0.0));
        self
    }

    /// Places the surface top edge at `fraction` of the window height
    /// (`top-1/3` → `1.0 / 3.0`) instead of vertical centering.
    pub fn vertical_anchor_top(mut self, fraction: f32) -> Self {
        self.vertical_anchor_top = Some(fraction.clamp(0.0, 1.0));
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
    /// clicks, the close button, backdrop clicks, and <kbd>Esc</kbd>.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Shows or hides the built-in top-right close button
    /// (`showCloseButton`, `true` by default).
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Keeps the dialog open on backdrop clicks
    /// (`interactOutsideBehavior: "ignore"`).
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

    /// Lets events through to the window behind the dialog while it is
    /// open (`modal: false`); the backdrop is still painted.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Patches the resolved [`DialogStyle`] (backdrop, colors, ring,
    /// radius, shadow) after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(DialogStyle) -> DialogStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<Dialog<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(dialog: Dialog<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(dialog.theme);
        let mut resolved = style::resolve_style(dialog.theme);

        if let Some(style_override) = dialog.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let pad = dialog.content_padding.unwrap_or(recipe.pad_px);
        let footer_row = dialog.footer.map(|footer| {
            let spacing = footer.spacing.unwrap_or(recipe.footer_gap_px);

            container(row(footer.children).spacing(spacing))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Right)
        });

        let surface: Element<'a, Message> = if recipe.footer_bar {
            // Nova-like packs: the footer escapes the content padding into
            // a full-width `bg-muted/50` bar with a hairline on top
            // (`-mx-4 -mb-4 rounded-b-xl border-t p-4`).
            let mut sections = column(None).width(Length::Fill);

            sections = sections.push(
                container(dialog.content)
                    .width(Length::Fill)
                    .padding(Padding::new(pad)),
            );

            if let Some(footer_row) = footer_row {
                let footer_background = resolved.footer_background;
                let footer_border = resolved.footer_border_color;
                let bottom_radius = (resolved.radius - resolved.border_width).max(0.0);

                sections = sections
                    .push(
                        container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
                            .style(move |_| container::Style {
                                background: Some(Background::Color(footer_border)),
                                ..container::Style::default()
                            }),
                    )
                    .push(
                        container(footer_row)
                            .width(Length::Fill)
                            .padding(Padding::new(pad))
                            .style(move |_| container::Style {
                                background: Some(Background::Color(footer_background)),
                                border: Border::default().rounded(border::bottom(bottom_radius)),
                                ..container::Style::default()
                            }),
                    );
            }

            sections.into()
        } else {
            let mut sections = column(None).width(Length::Fill).spacing(recipe.gap_px);
            sections = sections.push(dialog.content);

            if let Some(footer_row) = footer_row {
                sections = sections.push(footer_row);
            }

            container(sections)
                .width(Length::Fill)
                .padding(Padding::new(pad))
                .into()
        };

        let close = dialog
            .show_close_button
            .then(|| close::close_icon(DIALOG_CLOSE_ICON_PX, resolved.close_icon_color));

        Element::new(render::DialogWidget {
            trigger: dialog.trigger,
            surface,
            close,
            max_width: dialog.max_width.unwrap_or(recipe.max_width_px),
            margin: DIALOG_MARGIN_PX,
            close_size: DIALOG_CLOSE_SIZE_PX,
            close_offset: recipe.close_offset_px,
            vertical_anchor_top: dialog.vertical_anchor_top,
            duration: dialog.duration,
            animated: dialog.animated,
            disabled: dialog.disabled,
            open_override: dialog.open,
            default_open: dialog.default_open,
            on_open_change: dialog.on_open_change,
            close_on_click_outside: dialog.close_on_click_outside,
            close_on_escape: dialog.close_on_escape,
            modal: dialog.modal,
            style: resolved,
        })
    }
}

/// Styled dialog header: a tight column for title and description
/// (`.cn-dialog-header`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DialogHeader<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for DialogHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DialogHeader")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> DialogHeader<'a, Message> {
    /// Creates an empty header.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends the title slot.
    pub fn title(self, title: DialogTitle<'a, Message>) -> Self
    where
        Message: 'a,
    {
        self.push(title)
    }

    /// Appends the description slot.
    pub fn description(self, description: DialogDescription<'a, Message>) -> Self
    where
        Message: 'a,
    {
        self.push(description)
    }

    /// Appends arbitrary header content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Overrides the gap between header rows (`gap-2` / `gap-1.5` /
    /// `gap-1` by default).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> From<DialogHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: DialogHeader<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(header.theme);

        column(header.children)
            .spacing(header.spacing.unwrap_or(recipe.header_gap_px))
            .width(Length::Fill)
            .into()
    }
}

/// Styled dialog footer (`.cn-dialog-footer`): a right-aligned action row
/// (`flex sm:flex-row sm:justify-end gap-2`).
///
/// Convert it into an [`Element`] to place it manually, or hand it to
/// [`Dialog::footer`] so packs with a full-width footer bar (Nova) lay it
/// out faithfully.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DialogFooter<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for DialogFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DialogFooter")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> DialogFooter<'a, Message> {
    /// Creates an empty footer.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends an action (typically a button).
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Overrides the gap between actions (`gap-2` by default).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> From<DialogFooter<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(footer: DialogFooter<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(footer.theme);

        container(row(footer.children).spacing(footer.spacing.unwrap_or(recipe.footer_gap_px)))
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .into()
    }
}

/// Styled dialog title (`.cn-dialog-title`): heading font, style-pack
/// weight and size, inheriting the dialog foreground color.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DialogTitle<'a, Message> {
    content: DialogText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for DialogTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DialogTitle")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> DialogTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: DialogText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: DialogText::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<DialogTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: DialogTitle<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(title.theme);

        typeset(title.content, recipe.title, title.theme, true, None)
    }
}

/// Styled dialog description (`.cn-dialog-description`):
/// `text-muted-foreground` body copy.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DialogDescription<'a, Message> {
    content: DialogText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for DialogDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DialogDescription")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> DialogDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: DialogText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: DialogText::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<DialogDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: DialogDescription<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(description.theme);
        let muted = description.theme.palette.muted_foreground;

        typeset(
            description.content,
            recipe.description,
            description.theme,
            false,
            Some(muted),
        )
    }
}

/// Typesets a text slot with a [`TypeRecipe`]; element content passes
/// through untouched.
fn typeset<'a, Message: 'a>(
    content: DialogText<'a, Message>,
    recipe: TypeRecipe,
    theme: &Theme,
    heading: bool,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message> {
    match content {
        DialogText::Label(label) => {
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

            widget.into()
        }
        DialogText::Element(element) => element,
    }
}
