//! Dropdown menu component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Port of the shadcn-svelte dropdown-menu (`DropdownMenu.Root` / `Trigger` /
//! `Content` / `Item` / `CheckboxItem` / `RadioItem` / `Label` / `Separator` /
//! `Shortcut` / `Sub` / `SubTrigger` / `SubContent`) as a single iced builder.
//! The trigger element toggles a design-system popover surface with keyboard
//! navigation, nested submenus, and checkbox / radio rows. Interactions match
//! bits-ui: click to open, click-away / Esc to close, arrow keys + Enter while
//! open; plain items and radios close on pick, checkboxes stay open.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{DropdownMenuItem, Theme, dropdown_menu};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Profile,
//! }
//!
//! fn account<'a>(theme: &'a Theme) -> Element<'a, Message> {
//!     dropdown_menu("Open", theme)
//!         .item(DropdownMenuItem::new("Profile").on_select(Message::Profile))
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::{DropdownMenuContentStyle, dropdown_menu_content_style};
pub use types::{
    DropdownMenuCheckboxItem, DropdownMenuItem, DropdownMenuItemVariant, DropdownMenuLabel,
    DropdownMenuRadioItem, DropdownMenuSub,
};

use std::fmt;

use shadcn_common::DROPDOWN_MENU_SIDE_OFFSET_PX;

use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Background, Border, Element, Length, Padding};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

use render::DropdownMenuWidget;
use types::Entry;

/// Builder-first dropdown menu styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Pass `&theme` into
/// every menu — style packs live on the app's [`Theme`], not on this builder.
/// The application owns checkbox / radio state and feeds it back through the
/// entry builders on every change.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct DropdownMenu<'a, Message> {
    theme: &'a Theme,
    trigger: Option<Element<'a, Message>>,
    trigger_label: Option<String>,
    entries: Vec<Entry<Message>>,
    width: Option<f32>,
    side_offset: f32,
    disabled: bool,
    open: Option<bool>,
    default_open: bool,
    on_open: Option<Message>,
    on_close: Option<Message>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(DropdownMenuContentStyle) -> DropdownMenuContentStyle + 'a>>,
}

impl<Message> fmt::Debug for DropdownMenu<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DropdownMenu")
            .field("theme", &self.theme)
            .field("trigger", &self.trigger.is_some())
            .field("trigger_label", &self.trigger_label)
            .field("entries", &self.entries.len())
            .field("width", &self.width)
            .field("side_offset", &self.side_offset)
            .field("disabled", &self.disabled)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("on_open", &self.on_open.is_some())
            .field("on_close", &self.on_close.is_some())
            .field("on_open_change", &self.on_open_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> DropdownMenu<'a, Message> {
    /// Creates an empty dropdown menu.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`. Provide a trigger with
    /// [`Self::trigger`] or [`Self::trigger_label`].
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            trigger: None,
            trigger_label: None,
            entries: Vec::new(),
            width: None,
            side_offset: DROPDOWN_MENU_SIDE_OFFSET_PX,
            disabled: false,
            open: None,
            default_open: false,
            on_open: None,
            on_close: None,
            on_open_change: None,
            style_override: None,
        }
    }

    /// Sets the trigger element (`DropdownMenu.Trigger`).
    ///
    /// Clicks on the trigger toggle the menu; do not attach a separate
    /// `on_press` that fights the open state unless you also drive
    /// [`Self::open`].
    pub fn trigger(mut self, trigger: impl Into<Element<'a, Message>>) -> Self {
        self.trigger = Some(trigger.into());
        self.trigger_label = None;
        self
    }

    /// Builds an outline [`Button`] trigger with the given label.
    pub fn trigger_label(mut self, label: impl Into<String>) -> Self {
        self.trigger_label = Some(label.into());
        self.trigger = None;
        self
    }

    /// Appends a plain item (`DropdownMenu.Item`).
    pub fn item(mut self, item: DropdownMenuItem<Message>) -> Self {
        self.entries.push(Entry::Item(item));
        self
    }

    /// Appends a checkbox item (`DropdownMenu.CheckboxItem`).
    pub fn checkbox_item(mut self, item: DropdownMenuCheckboxItem<Message>) -> Self {
        self.entries.push(Entry::Checkbox(item));
        self
    }

    /// Appends a radio item (`DropdownMenu.RadioItem`).
    pub fn radio_item(mut self, item: DropdownMenuRadioItem<Message>) -> Self {
        self.entries.push(Entry::Radio(item));
        self
    }

    /// Appends a section label (`DropdownMenu.Label`).
    pub fn label(mut self, label: impl Into<DropdownMenuLabel>) -> Self {
        self.entries.push(Entry::Label(label.into()));
        self
    }

    /// Appends a hairline separator (`DropdownMenu.Separator`).
    pub fn separator(mut self) -> Self {
        self.entries.push(Entry::Separator);
        self
    }

    /// Appends a nested submenu (`DropdownMenu.Sub`).
    pub fn submenu(mut self, submenu: DropdownMenuSub<Message>) -> Self {
        self.entries.push(Entry::Sub(submenu));
        self
    }

    /// Sets a fixed content width in px.
    ///
    /// When unset, the content is at least as wide as the trigger and the
    /// pack's `min-w-*` (`w-(--bits-dropdown-menu-anchor-width)`).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(1.0));
        self
    }

    /// Sets the gap between the trigger and the content (`sideOffset`).
    pub fn side_offset(mut self, side_offset: f32) -> Self {
        self.side_offset = side_offset.max(0.0);
        self
    }

    /// Disables the menu (`disabled` on the root: no open, 50% opacity trigger).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the controlled open state (`bind:open`).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Sets the uncontrolled initial open state (`defaultOpen`).
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Sets the message emitted when the menu opens.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Sets the message emitted when the menu closes.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Sets the callback receiving every open-state change.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Applies a narrow iced-style escape hatch after content style resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(DropdownMenuContentStyle) -> DropdownMenuContentStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

/// Convenience helper that creates a dropdown with an outline trigger button.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{DropdownMenuItem, Theme, dropdown_menu};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Profile,
/// }
///
/// fn account<'a>(theme: &'a Theme) -> Element<'a, Message> {
///     dropdown_menu("Open", theme)
///         .item(DropdownMenuItem::new("Profile").on_select(Message::Profile))
///         .into()
/// }
/// ```
pub fn dropdown_menu<'a, Message: Clone + 'a>(
    trigger_label: impl Into<String>,
    theme: &'a Theme,
) -> DropdownMenu<'a, Message> {
    DropdownMenu::new(theme).trigger_label(trigger_label)
}

impl<'a, Message> From<DropdownMenu<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu: DropdownMenu<'a, Message>) -> Self {
        let DropdownMenu {
            theme,
            trigger,
            trigger_label,
            entries,
            width,
            side_offset,
            disabled,
            open,
            default_open,
            on_open,
            on_close,
            on_open_change,
            style_override,
        } = menu;

        let trigger = trigger.unwrap_or_else(|| {
            let label = trigger_label.unwrap_or_else(|| "Open".to_owned());
            default_outline_trigger(label, theme)
        });

        Self::new(DropdownMenuWidget {
            theme,
            trigger,
            entries,
            width,
            side_offset,
            disabled,
            open_override: open,
            default_open,
            on_open,
            on_close,
            on_open_change,
            style_override,
        })
    }
}

/// Outline trigger matching `Button variant="outline"` without requiring an
/// `on_press` message (the menu widget owns the click).
fn default_outline_trigger<'a, Message: 'a>(
    label: String,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let border = theme.palette.border;
    let foreground = theme.palette.foreground;
    let background = theme.palette.background;
    let radius = component_radius_px(theme, theme.style.button_type().default_radius);
    let height = theme.style.control_height_md_px;

    container(text(label).size(14.0).color(foreground))
        .padding(Padding {
            top: 0.0,
            right: 16.0,
            bottom: 0.0,
            left: 16.0,
        })
        .height(Length::Fixed(height))
        .center_y(Length::Fixed(height))
        .style(move |_theme: &crate::iced_compat::Theme| container::Style {
            background: Some(Background::Color(background)),
            text_color: Some(foreground),
            border: Border {
                color: border,
                width: 1.0,
                radius: radius.into(),
            },
            ..container::Style::default()
        })
        .into()
}
