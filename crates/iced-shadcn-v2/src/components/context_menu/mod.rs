//! Context menu component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Port of the shadcn-svelte context-menu (`ContextMenu.Root` / `Trigger` /
//! `Content` / `Item` / `CheckboxItem` / `RadioItem` / `Label` / `Separator` /
//! `Shortcut` / `Sub` / `SubTrigger` / `SubContent`) as a single iced builder.
//! The trigger element opens a design-system popover surface on **secondary
//! (right) click**, anchored at the cursor position (bits-ui `position`).
//! Keyboard navigation, nested submenus, checkbox / radio rows, destructive
//! variants, shortcuts, insets, and the optional `side` prop all match the
//! web component. Interactions match bits-ui: right-click to open,
//! click-away / Esc to close, arrow keys + Enter while open; plain items and
//! radios close on pick, checkboxes stay open.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{ContextMenuItem, Theme, context_menu};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Back,
//! }
//!
//! fn browser_area<'a>(theme: &'a Theme) -> Element<'a, Message> {
//!     context_menu("Right click here", theme)
//!         .item(ContextMenuItem::new("Back").on_select(Message::Back))
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::ContextMenuContentStyle;
pub use types::{
    ContextMenuCheckboxItem, ContextMenuItem, ContextMenuItemVariant, ContextMenuLabel,
    ContextMenuRadioItem, ContextMenuSub,
};

use std::fmt;

use shadcn_common::{CONTEXT_MENU_SIDE_OFFSET_PX, ContextMenuRecipe, FloatingSide};

use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Background, Border, Element, Length, Padding};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

use render::ContextMenuWidget;
use types::Entry;

/// Builder-first context menu styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Pass `&theme` into
/// every menu — style packs live on the app's [`Theme`], not on this builder.
/// The application owns checkbox / radio state and feeds it back through the
/// entry builders on every change.
///
/// Unlike [`crate::DropdownMenu`], the menu opens on a **secondary (right)
/// click** over the trigger and is anchored at the cursor position (bits-ui
/// `position`), not below the trigger bounds. Use [`Self::side`] to pin the
/// placement to a specific edge of the cursor; the default is `auto` (flip).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct ContextMenu<'a, Message> {
    theme: &'a Theme,
    trigger: Option<Element<'a, Message>>,
    trigger_label: Option<String>,
    entries: Vec<Entry<Message>>,
    width: Option<f32>,
    side: Option<FloatingSide>,
    side_offset: f32,
    disabled: bool,
    open: Option<bool>,
    default_open: bool,
    on_open: Option<Message>,
    on_close: Option<Message>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(ContextMenuContentStyle) -> ContextMenuContentStyle + 'a>>,
}

impl<Message> fmt::Debug for ContextMenu<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContextMenu")
            .field("theme", &self.theme)
            .field("trigger", &self.trigger.is_some())
            .field("trigger_label", &self.trigger_label)
            .field("entries", &self.entries.len())
            .field("width", &self.width)
            .field("side", &self.side)
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

impl<'a, Message> ContextMenu<'a, Message> {
    /// Creates an empty context menu.
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
            side: None,
            side_offset: CONTEXT_MENU_SIDE_OFFSET_PX,
            disabled: false,
            open: None,
            default_open: false,
            on_open: None,
            on_close: None,
            on_open_change: None,
            style_override: None,
        }
    }

    /// Sets the trigger element (`ContextMenu.Trigger`).
    ///
    /// Secondary (right) clicks on the trigger open the menu; do not attach a
    /// separate `on_press` that fights the open state unless you also drive
    /// [`Self::open`].
    pub fn trigger(mut self, trigger: impl Into<Element<'a, Message>>) -> Self {
        self.trigger = Some(trigger.into());
        self.trigger_label = None;
        self
    }

    /// Builds the default bordered trigger area with the given label.
    ///
    /// Matches the shadcn-svelte docs demo: a full-width bordered box that says
    /// "Right click here".
    pub fn trigger_label(mut self, label: impl Into<String>) -> Self {
        self.trigger_label = Some(label.into());
        self.trigger = None;
        self
    }

    /// Appends a plain item (`ContextMenu.Item`).
    pub fn item(mut self, item: ContextMenuItem<Message>) -> Self {
        self.entries.push(Entry::Item(item));
        self
    }

    /// Appends a checkbox item (`ContextMenu.CheckboxItem`).
    pub fn checkbox_item(mut self, item: ContextMenuCheckboxItem<Message>) -> Self {
        self.entries.push(Entry::Checkbox(item));
        self
    }

    /// Appends a radio item (`ContextMenu.RadioItem`).
    pub fn radio_item(mut self, item: ContextMenuRadioItem<Message>) -> Self {
        self.entries.push(Entry::Radio(item));
        self
    }

    /// Appends a section label (`ContextMenu.Label`).
    pub fn label(mut self, label: impl Into<ContextMenuLabel>) -> Self {
        self.entries.push(Entry::Label(label.into()));
        self
    }

    /// Appends a hairline separator (`ContextMenu.Separator`).
    pub fn separator(mut self) -> Self {
        self.entries.push(Entry::Separator);
        self
    }

    /// Appends a nested submenu (`ContextMenu.Sub`).
    pub fn submenu(mut self, submenu: ContextMenuSub<Message>) -> Self {
        self.entries.push(Entry::Sub(submenu));
        self
    }

    /// Sets a fixed content width in px.
    ///
    /// When unset, the content uses the pack's `min-w-*`.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(1.0));
        self
    }

    /// Pins the placement side relative to the cursor anchor
    /// (`ContextMenu.Content side="…"`).
    ///
    /// `None` (the default) keeps the bits-ui `auto` placement: prefer
    /// below-right of the cursor, flip above when there isn't room.
    pub fn side(mut self, side: FloatingSide) -> Self {
        self.side = Some(side);
        self
    }

    /// Sets the gap between the cursor anchor and the content (`sideOffset`).
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
        style_override: impl Fn(ContextMenuContentStyle) -> ContextMenuContentStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

/// Convenience helper that creates a context menu with the default bordered
/// trigger area used by the shadcn-svelte docs demos.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{ContextMenuItem, Theme, context_menu};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Back,
/// }
///
/// fn browser_area<'a>(theme: &'a Theme) -> Element<'a, Message> {
///     context_menu("Right click here", theme)
///         .item(ContextMenuItem::new("Back").on_select(Message::Back))
///         .into()
/// }
/// ```
pub fn context_menu<'a, Message: Clone + 'a>(
    trigger_label: impl Into<String>,
    theme: &'a Theme,
) -> ContextMenu<'a, Message> {
    ContextMenu::new(theme).trigger_label(trigger_label)
}

impl<'a, Message> From<ContextMenu<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu: ContextMenu<'a, Message>) -> Self {
        let ContextMenu {
            theme,
            trigger,
            trigger_label,
            entries,
            width,
            side,
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
            let label = trigger_label.unwrap_or_else(|| "Right click here".to_owned());
            default_trigger_area(label, theme)
        });

        Self::new(ContextMenuWidget {
            theme,
            trigger,
            entries,
            width,
            side,
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

/// Default trigger matching the shadcn-svelte docs demo: a full-width bordered
/// box (`flex aspect-[2/0.5] w-full items-center justify-center rounded-lg
/// border text-sm`).
fn default_trigger_area<'a, Message: 'a>(label: String, theme: &'a Theme) -> Element<'a, Message> {
    let border = theme.palette.border;
    let foreground = theme.palette.foreground;
    let background = theme.palette.background;
    let radius = component_radius_px(theme, theme.style.button_type().default_radius);

    container(
        text(label)
            .size(14.0)
            .color(foreground)
            .align_x(iced_core::alignment::Horizontal::Center),
    )
    .padding(Padding {
        top: 0.0,
        right: 12.0,
        bottom: 0.0,
        left: 12.0,
    })
    .width(Length::Fill)
    .height(Length::Fixed(80.0))
    .center_x(Length::Fill)
    .center_y(Length::Fixed(80.0))
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

/// Resolves the active context-menu recipe for the given theme.
///
/// Exposed so the example / tests can compute geometry without reaching into
/// the private style module.
#[doc(hidden)]
pub fn recipe_of(theme: &Theme) -> ContextMenuRecipe {
    theme.style.context_menu()
}
