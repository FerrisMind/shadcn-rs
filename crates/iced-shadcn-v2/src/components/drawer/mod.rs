//! Builder-first drawer component.
//!
//! Port of the shadcn-svelte drawer (`Drawer.Root` / `Trigger` / `Portal` /
//! `Overlay` / `Content` + `Header` / `Footer` / `Title` / `Description` /
//! `Close` / `NestedRoot`, a vaul sheet) as a single iced builder: the trigger
//! element is wrapped by a custom widget that opens a modal surface along
//! `direction` over a dimmed backdrop. While open the rest of the window is
//! inert; clicks on the backdrop and <kbd>Esc</kbd> dismiss the drawer, and
//! bottom drawers expose the web drag handle. The public API lives in this
//! module; widget/overlay internals live in focused private submodules.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::DrawerStyle;

use std::fmt;

use shadcn_common::{
    DRAWER_ANIMATION_MS, DRAWER_HANDLE_MARGIN_TOP_PX, DRAWER_HANDLE_WIDTH_PX, TypeRecipe,
};

pub use shadcn_common::DrawerDirection;

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{column, container};
use crate::iced_compat::{Element, Length, Padding, Pixels, alignment, time::Duration};

use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Builder-first drawer styled directly with iced types.
///
/// Wraps a trigger element and opens a modal surface on click, matching
/// shadcn-svelte / vaul defaults: the surface docks to
/// [`DrawerDirection::Bottom`] (`inset-x-0 bottom-0 mt-24 max-h-[80vh]
/// rounded-t-xl`), paints the `bg-popover` / `text-popover-foreground` pair
/// with an inner-edge `border-*` hairline over a `bg-black/N` backdrop, shows
/// the muted drag handle on bottom drawers, closes on backdrop clicks and
/// <kbd>Esc</kbd>, and animates with a full-panel slide (vaul).
///
/// While the drawer is open the window behind it is inert: pointer, scroll,
/// and keyboard events never reach the underlying widgets, mirroring the
/// web modal focus containment.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     Button, ButtonVariant, Drawer, DrawerDescription, DrawerDirection, DrawerHeader,
///     DrawerTitle, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Drawer::new(
///         Button::text("Open", theme)
///             .variant(ButtonVariant::Outline)
///             .on_press(Message::Pressed),
///         DrawerHeader::new(theme)
///             .title(DrawerTitle::text("Move Goal", theme))
///             .description(DrawerDescription::text(
///                 "Set your daily activity goal.",
///                 theme,
///             )),
///         theme,
///     )
///     .direction(DrawerDirection::Bottom)
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Drawer<'a, Message> {
    trigger: Element<'a, Message>,
    content: Element<'a, Message>,
    footer: Option<DrawerFooter<'a, Message>>,
    theme: &'a Theme,
    direction: DrawerDirection,
    max_width: Option<f32>,
    max_height: Option<f32>,
    duration: Duration,
    animated: bool,
    disabled: bool,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    modal: bool,
    should_scale_background: bool,
    show_handle: bool,
    snap_points: Vec<f32>,
    active_snap_point: Option<f32>,
    on_snap_point_change: Option<Box<dyn Fn(Option<f32>) -> Message + 'a>>,
    nested: bool,
    style_override: Option<Box<dyn Fn(DrawerStyle) -> DrawerStyle + 'a>>,
}

enum DrawerText<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Drawer<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Drawer")
            .field("footer", &self.footer.is_some())
            .field("theme", &self.theme)
            .field("direction", &self.direction)
            .field("max_width", &self.max_width)
            .field("max_height", &self.max_height)
            .field("duration", &self.duration)
            .field("animated", &self.animated)
            .field("disabled", &self.disabled)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("close_on_click_outside", &self.close_on_click_outside)
            .field("close_on_escape", &self.close_on_escape)
            .field("modal", &self.modal)
            .field("should_scale_background", &self.should_scale_background)
            .field("show_handle", &self.show_handle)
            .field("snap_points", &self.snap_points)
            .field("active_snap_point", &self.active_snap_point)
            .field("on_snap_point_change", &self.on_snap_point_change.is_some())
            .field("nested", &self.nested)
            .field("style_override", &self.style_override.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, Message> Drawer<'a, Message> {
    /// Creates a drawer opening `content` over `trigger`.
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
            direction: DrawerDirection::Bottom,
            max_width: None,
            max_height: None,
            duration: Duration::from_millis(DRAWER_ANIMATION_MS),
            animated: true,
            disabled: false,
            open: None,
            default_open: false,
            on_open_change: None,
            close_on_click_outside: true,
            close_on_escape: true,
            modal: true,
            should_scale_background: true,
            show_handle: true,
            snap_points: Vec::new(),
            active_snap_point: None,
            on_snap_point_change: None,
            nested: false,
            style_override: None,
        }
    }

    /// Docks the drawer to `direction` (`data-vaul-drawer-direction`, defaults
    /// to [`DrawerDirection::Bottom`]).
    pub fn direction(mut self, direction: DrawerDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Appends the footer slot (`Drawer.Footer`), laid out with `mt-auto`
    /// against the bottom of left/right drawers.
    pub fn footer(mut self, footer: DrawerFooter<'a, Message>) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Overrides the left/right maximum width in px (`sm:max-w-sm` — 384 px
    /// by default).
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Caps top/bottom drawer height in px (mirrors demo `max-h-[50vh]`).
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height.max(0.0));
        self
    }

    /// Sets the duration of the open/close animation (vaul default 500 ms).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Enables or disables the open/close animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Prevents the drawer from opening while keeping the trigger active.
    ///
    /// An already open drawer closes when it becomes disabled.
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

    /// Opens the drawer on first mount when uncontrolled (`defaultOpen`).
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Notifies about open-state change requests (`onOpenChange`): trigger
    /// clicks, backdrop clicks, drag-dismiss, and <kbd>Esc</kbd>.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Keeps the drawer open on backdrop clicks
    /// (`interactOutsideBehavior: "ignore"`).
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Keeps the drawer open on <kbd>Esc</kbd>
    /// (`escapeKeydownBehavior: "ignore"`).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Lets events through to the window behind the drawer while it is open
    /// (`modal: false`); the backdrop is still painted.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Mirrors vaul `shouldScaleBackground` (default `true`): when enabled the
    /// backdrop paints slightly stronger to suggest the scaled page behind.
    pub fn should_scale_background(mut self, scale: bool) -> Self {
        self.should_scale_background = scale;
        self
    }

    /// Shows or hides the bottom drag handle (`.cn-drawer-handle`). Defaults
    /// to `true`; the handle only paints for [`DrawerDirection::Bottom`].
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Sets vaul-style snap points as viewport-height fractions
    /// (`snapPoints={[0.4, 0.8]}`).
    pub fn snap_points(mut self, snap_points: impl IntoIterator<Item = f32>) -> Self {
        self.snap_points = snap_points
            .into_iter()
            .map(|point| point.clamp(0.0, 1.0))
            .collect();
        self
    }

    /// Controls the active snap point (`bind:activeSnapPoint`).
    pub fn active_snap_point(mut self, point: Option<f32>) -> Self {
        self.active_snap_point = point.map(|point| point.clamp(0.0, 1.0));
        self
    }

    /// Notifies when a drag settles onto a new snap point.
    pub fn on_snap_point_change(
        mut self,
        on_snap_point_change: impl Fn(Option<f32>) -> Message + 'a,
    ) -> Self {
        self.on_snap_point_change = Some(Box::new(on_snap_point_change));
        self
    }

    /// Marks this drawer as nested (`Drawer.NestedRoot`): the surface is
    /// inset slightly so it reads as stacked above another drawer.
    pub fn nested(mut self, nested: bool) -> Self {
        self.nested = nested;
        self
    }

    /// Patches the resolved [`DrawerStyle`] after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(DrawerStyle) -> DrawerStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<Drawer<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(drawer: Drawer<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(drawer.theme);
        let mut resolved = style::resolve_style(drawer.theme, drawer.direction);

        if let Some(style_override) = drawer.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let footer_row = drawer.footer.map(|footer| {
            let spacing = footer.spacing.unwrap_or(recipe.footer_gap_px);
            let pad = recipe.footer_pad_px;
            let children = footer
                .children
                .into_iter()
                .map(|child| container(child).width(Length::Fill).into())
                .collect::<Vec<_>>();

            container(column(children).spacing(spacing).width(Length::Fill))
                .width(Length::Fill)
                .padding(Padding::new(pad))
        });

        let fill_height = drawer.direction.is_vertical_edge();
        let centers = drawer.direction.centers_header();

        let mut sections = column(None).width(Length::Fill);
        if fill_height {
            sections = sections.height(Length::Fill);
        }
        if centers {
            sections = sections.align_x(alignment::Horizontal::Center);
        }

        // Vertical drawers: content fills remaining height so nested
        // `ScrollArea` / `scrollable` get a bounded viewport (web
        // `overflow-y-auto`). Footer stays at the bottom without a spacer.
        if fill_height {
            sections = sections.push(
                container(drawer.content)
                    .width(Length::Fill)
                    .height(Length::Fill),
            );
        } else {
            sections = sections.push(drawer.content);
        }

        if let Some(footer_row) = footer_row {
            sections = sections.push(footer_row);
        }

        let surface: Element<'a, Message> = sections.into();

        Element::new(render::DrawerWidget {
            trigger: drawer.trigger,
            surface,
            direction: drawer.direction,
            max_width: drawer.max_width.unwrap_or(recipe.max_width_px),
            max_height: drawer.max_height,
            handle_width: DRAWER_HANDLE_WIDTH_PX,
            handle_margin_top: DRAWER_HANDLE_MARGIN_TOP_PX,
            duration: drawer.duration,
            animated: drawer.animated,
            disabled: drawer.disabled,
            open_override: drawer.open,
            default_open: drawer.default_open,
            on_open_change: drawer.on_open_change,
            close_on_click_outside: drawer.close_on_click_outside,
            close_on_escape: drawer.close_on_escape,
            modal: drawer.modal,
            should_scale_background: drawer.should_scale_background,
            show_handle: drawer.show_handle && drawer.direction.shows_handle(),
            snap_points: drawer.snap_points,
            active_snap_point: drawer.active_snap_point,
            on_snap_point_change: drawer.on_snap_point_change,
            nested: drawer.nested,
            style: resolved,
        })
    }
}

/// Styled drawer header: a padded column for title and description
/// (`.cn-drawer-header`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DrawerHeader<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
    center: bool,
}

impl<Message> fmt::Debug for DrawerHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DrawerHeader")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("center", &self.center)
            .finish()
    }
}

impl<'a, Message> DrawerHeader<'a, Message> {
    /// Creates an empty header.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
            center: false,
        }
    }

    /// Centers the header text (web default for top/bottom drawers).
    pub fn center(mut self, center: bool) -> Self {
        self.center = center;
        self
    }

    /// Appends the title slot.
    pub fn title(self, title: DrawerTitle<'a, Message>) -> Self
    where
        Message: 'a,
    {
        self.push(title)
    }

    /// Appends the description slot.
    pub fn description(self, description: DrawerDescription<'a, Message>) -> Self
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

impl<'a, Message> From<DrawerHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: DrawerHeader<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(header.theme);

        let mut col = column(header.children)
            .spacing(header.spacing.unwrap_or(recipe.header_gap_px))
            .width(Length::Fill);

        if header.center {
            col = col.align_x(alignment::Horizontal::Center);
        }

        container(col)
            .width(Length::Fill)
            .padding(Padding::new(recipe.header_pad_px))
            .into()
    }
}

/// Styled drawer footer (`.cn-drawer-footer`): a padded column of actions
/// (`mt-auto flex flex-col gap-2`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DrawerFooter<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for DrawerFooter<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DrawerFooter")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> DrawerFooter<'a, Message> {
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

impl<'a, Message> From<DrawerFooter<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(footer: DrawerFooter<'a, Message>) -> Element<'a, Message> {
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

/// Padded body region between header and footer (mirrors the `px-4` scroll
/// wrapper in the shadcn-svelte drawer demos).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DrawerBody<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for DrawerBody<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DrawerBody")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> DrawerBody<'a, Message> {
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

impl<'a, Message> From<DrawerBody<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(body: DrawerBody<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(body.theme);
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

/// Styled drawer title (`.cn-drawer-title`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DrawerTitle<'a, Message> {
    content: DrawerText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for DrawerTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DrawerTitle")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> DrawerTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: DrawerText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: DrawerText::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<DrawerTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: DrawerTitle<'a, Message>) -> Element<'a, Message> {
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

/// Styled drawer description (`.cn-drawer-description`):
/// `text-muted-foreground` body copy.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DrawerDescription<'a, Message> {
    content: DrawerText<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for DrawerDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DrawerDescription")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> DrawerDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: DrawerText::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: DrawerText::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<DrawerDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: DrawerDescription<'a, Message>) -> Element<'a, Message> {
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
    content: DrawerText<'a, Message>,
    recipe: TypeRecipe,
    theme: &Theme,
    heading: bool,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message> {
    match content {
        DrawerText::Label(label) => {
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
        DrawerText::Element(element) => element,
    }
}
