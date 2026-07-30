//! Builder-first sheet component.
//!
//! Port of the shadcn-svelte sheet (`Sheet.Root` / `Trigger` / `Portal` /
//! `Overlay` / `Content` + `Header` / `Footer` / `Title` / `Description` /
//! `Close`, a bits-ui dialog docked to an edge) as a single iced builder:
//! the trigger element is wrapped by a custom widget that opens a modal
//! surface along `side` over a dimmed backdrop. While open the rest of the
//! window is inert; clicks on the backdrop and <kbd>Esc</kbd> dismiss the
//! sheet, and the built-in close button mirrors the web `ghost` / `icon-sm`
//! X button. The public API lives in this module; widget/overlay internals
//! live in focused private submodules.

mod close;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::SheetStyle;

use std::fmt;

use shadcn_common::{SHEET_ANIMATION_MS, SHEET_CLOSE_ICON_PX, SHEET_CLOSE_SIZE_PX, TypeRecipe};

pub use shadcn_common::SheetSide;

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{Space, column, container};
use crate::iced_compat::{Element, Length, Padding, Pixels, time::Duration};

use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Builder-first sheet styled directly with iced types.
///
/// Wraps a trigger element and opens a modal surface on click, matching
/// shadcn-svelte defaults: the surface docks to [`SheetSide::Right`]
/// (`inset-y-0 right-0 w-3/4 sm:max-w-sm`), paints the `bg-popover` /
/// `text-popover-foreground` pair with an inner-edge `border-*` hairline
/// over a `bg-black/N` backdrop, shows the ghost `icon-sm` close button in
/// the top-right corner, closes on backdrop clicks and <kbd>Esc</kbd>, and
/// animates with the web `fade-in-0 slide-in-from-*-10` entrance
/// (`duration-200`).
///
/// While the sheet is open the window behind it is inert: pointer, scroll,
/// and keyboard events never reach the underlying widgets, mirroring the
/// web modal focus containment.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     Button, ButtonVariant, Sheet, SheetDescription, SheetHeader, SheetSide, SheetTitle,
///     Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Sheet::new(
///         Button::text("Open", theme)
///             .variant(ButtonVariant::Outline)
///             .on_press(Message::Pressed),
///         SheetHeader::new(theme)
///             .title(SheetTitle::text("Edit profile", theme))
///             .description(SheetDescription::text(
///                 "Make changes to your profile here.",
///                 theme,
///             )),
///         theme,
///     )
///     .side(SheetSide::Right)
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Sheet<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    footer: Option<SheetFooter<'a, Message>>,
    theme: &'a Theme,
    side: SheetSide,
    max_width: Option<f32>,
    max_height: Option<f32>,
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
    style_override: Option<Box<dyn Fn(SheetStyle) -> SheetStyle + 'a>>,
}

enum SheetText<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Sheet<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sheet")
            .field("footer", &self.footer.is_some())
            .field("theme", &self.theme)
            .field("side", &self.side)
            .field("max_width", &self.max_width)
            .field("max_height", &self.max_height)
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

impl<'a, Message> Sheet<'a, Message> {
    /// Creates a sheet opening `content` over `trigger`.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
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
            side: SheetSide::Right,
            max_width: None,
            max_height: None,
            duration: Duration::from_millis(SHEET_ANIMATION_MS),
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

    /// Docks the sheet to `side` (`data-side`, defaults to
    /// [`SheetSide::Right`]).
    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    /// Appends the footer slot (`Sheet.Footer`), laid out with `mt-auto`
    /// against the bottom of left/right sheets.
    pub fn footer(mut self, footer: SheetFooter<'a, Message>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Overrides the left/right maximum width in px (`sm:max-w-sm` — 384 px
    /// by default).
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Caps top/bottom sheet height in px (mirrors `max-h-[50vh]` classes).
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height.max(0.0));
        self
    }

    /// Sets the duration of the open/close animation (`duration-200`).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Enables or disables the open/close animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Prevents the sheet from opening while keeping the trigger active.
    ///
    /// An already open sheet closes when it becomes disabled.
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

    /// Opens the sheet on first mount when uncontrolled (`defaultOpen`).
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

    /// Keeps the sheet open on backdrop clicks
    /// (`interactOutsideBehavior: "ignore"`).
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Keeps the sheet open on <kbd>Esc</kbd>
    /// (`escapeKeydownBehavior: "ignore"`).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Lets events through to the window behind the sheet while it is open
    /// (`modal: false`); the backdrop is still painted.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Patches the resolved [`SheetStyle`] after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(SheetStyle) -> SheetStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<Sheet<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(sheet: Sheet<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(sheet.theme);
        let mut resolved = style::resolve_style(sheet.theme);

        if let Some(style_override) = sheet.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let footer_row = sheet.footer.map(|footer| {
            let spacing = footer.spacing.unwrap_or(recipe.footer_gap_px);
            let pad = recipe.footer_pad_px;
            // `flex flex-col` stretches children (`w-full` buttons in the web demos).
            let children = footer
                .children
                .into_iter()
                .map(|child| container(child).width(Length::Fill).into())
                .collect::<Vec<_>>();

            container(column(children).spacing(spacing).width(Length::Fill))
                .width(Length::Fill)
                .padding(Padding::new(pad))
        });

        let fill_height = sheet.side.is_vertical_edge();

        let mut sections = column(None).width(Length::Fill);
        if fill_height {
            sections = sections.height(Length::Fill);
        }
        if recipe.gap_px > 0.0 {
            sections = sections.spacing(recipe.gap_px);
        }

        sections = sections.push(sheet.content);

        if let Some(footer_row) = footer_row {
            if fill_height {
                sections = sections.push(Space::new().width(Length::Fill).height(Length::Fill));
            }
            sections = sections.push(footer_row);
        }

        let surface: Element<'a, Message> = sections.into();

        let close = sheet
            .show_close_button
            .then(|| close::close_icon(SHEET_CLOSE_ICON_PX, resolved.close_icon_color));

        Element::new(render::SheetWidget {
            trigger: sheet.trigger,
            surface,
            close,
            side: sheet.side,
            max_width: sheet.max_width.unwrap_or(recipe.max_width_px),
            max_height: sheet.max_height,
            close_size: SHEET_CLOSE_SIZE_PX,
            close_offset: recipe.close_offset_px,
            duration: sheet.duration,
            animated: sheet.animated,
            disabled: sheet.disabled,
            open_override: sheet.open,
            default_open: sheet.default_open,
            on_open_change: sheet.on_open_change,
            close_on_click_outside: sheet.close_on_click_outside,
            close_on_escape: sheet.close_on_escape,
            modal: sheet.modal,
            style: resolved,
        })
    }
}

/// Styled sheet header: a padded column for title and description
/// (`.cn-sheet-header`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SheetHeader<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for SheetHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SheetHeader")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> SheetHeader<'a, Message> {
    /// Creates an empty header.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends the title slot.
    pub fn title(self, title: SheetTitle<'a, Message>) -> Self
    where
        Message: 'a,
    {
        self.push(title)
    }

    /// Appends the description slot.
    pub fn description(self, description: SheetDescription<'a, Message>) -> Self
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

    /// Overrides the gap between header rows.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> From<SheetHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: SheetHeader<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(header.theme);

        container(
            column(header.children)
                .spacing(header.spacing.unwrap_or(recipe.header_gap_px))
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding(Padding::new(recipe.header_pad_px))
        .into()
    }
}

/// Styled sheet footer (`.cn-sheet-footer`): a padded column of actions
/// (`mt-auto flex flex-col gap-2`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SheetFooter<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for SheetFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SheetFooter")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> SheetFooter<'a, Message> {
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

impl<'a, Message> From<SheetFooter<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(footer: SheetFooter<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(footer.theme);
        let children = footer
            .children
            .into_iter()
            .map(|child| container(child).width(Length::Fill).into())
            .collect::<Vec<_>>();

        container(
            column(children)
                .spacing(footer.spacing.unwrap_or(recipe.footer_gap_px))
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding(Padding::new(recipe.footer_pad_px))
        .into()
    }
}

/// Padded body region between header and footer (mirrors the `px-*` wrapper
/// around form fields in the shadcn-svelte sheet demos).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SheetBody<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for SheetBody<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SheetBody")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> SheetBody<'a, Message> {
    /// Creates an empty body.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends body content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Overrides the gap between body rows.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> From<SheetBody<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(body: SheetBody<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(body.theme);
        // Horizontal padding matches the header/footer pack padding
        // (`style-*:px-4` / `px-6`).
        let pad_x = recipe.header_pad_px;

        container(
            column(body.children)
                .spacing(body.spacing.unwrap_or(16.0))
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding(Padding {
            top: 0.0,
            right: pad_x,
            bottom: 0.0,
            left: pad_x,
        })
        .into()
    }
}

/// Styled sheet title (`.cn-sheet-title`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SheetTitle<'a, Message> {
    content: SheetText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for SheetTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SheetTitle")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> SheetTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: SheetText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: SheetText::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<SheetTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: SheetTitle<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(title.theme);
        let foreground = title.theme.palette.foreground;

        typeset(
            title.content,
            recipe.title,
            title.theme,
            true,
            Some(foreground),
        )
    }
}

/// Styled sheet description (`.cn-sheet-description`):
/// `text-muted-foreground` body copy.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct SheetDescription<'a, Message> {
    content: SheetText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for SheetDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SheetDescription")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> SheetDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: SheetText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: SheetText::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<SheetDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: SheetDescription<'a, Message>) -> Element<'a, Message> {
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
    content: SheetText<'a, Message>,
    recipe: TypeRecipe,
    theme: &Theme,
    heading: bool,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message> {
    match content {
        SheetText::Label(label) => {
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
        SheetText::Element(element) => element,
    }
}
