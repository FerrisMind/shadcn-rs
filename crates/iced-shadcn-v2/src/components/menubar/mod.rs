//! Menubar component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! Port of the shadcn-svelte menubar (`Menubar.Root` / `Menu` / `Trigger` /
//! `Content` / `Item` / `CheckboxItem` / `RadioItem` / `Label` / `Separator` /
//! `Shortcut` / `Sub` / `SubTrigger` / `SubContent`) as a single iced builder.
//! A persistent horizontal bar holds multiple menu triggers; only one content
//! panel is open at a time. While open, hovering another trigger switches menus
//! (bits-ui roving). Checkbox / radio indicators sit on the leading edge.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{MenubarItem, MenubarMenu, Theme, menubar};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     NewTab,
//! }
//!
//! fn app_menu<'a>(theme: &'a Theme) -> Element<'a, Message> {
//!     menubar(theme)
//!         .menu(
//!             MenubarMenu::new("File")
//!                 .item(MenubarItem::new("New Tab").on_select(Message::NewTab)),
//!         )
//!         .into()
//! }
//! ```

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::{MenubarBarStyle, MenubarContentStyle};
pub use types::{
    MenubarCheckboxItem, MenubarItem, MenubarItemVariant, MenubarLabel, MenubarMenu,
    MenubarRadioItem, MenubarSub,
};

use std::fmt;

use shadcn_common::{MENUBAR_ALIGN_OFFSET_PX, MENUBAR_SIDE_OFFSET_PX};

use crate::iced_compat::Element;
use crate::theme::Theme;

use render::MenubarWidget;
use types::MenubarMenu as MenuBuilder;

/// Builder-first menubar styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Pass `&theme` into
/// every menubar — style packs live on the app's [`Theme`], not on this builder.
/// The application owns checkbox / radio state and feeds it back through the
/// entry builders on every change.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Menubar<'a, Message> {
    theme: &'a Theme,
    menus: Vec<MenuBuilder<Message>>,
    width: Option<f32>,
    side_offset: f32,
    align_offset: f32,
    disabled: bool,
    open_menu: Option<usize>,
    default_open_menu: Option<usize>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(MenubarContentStyle) -> MenubarContentStyle + 'a>>,
}

impl<Message> fmt::Debug for Menubar<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Menubar")
            .field("theme", &self.theme)
            .field("menus", &self.menus.len())
            .field("width", &self.width)
            .field("side_offset", &self.side_offset)
            .field("align_offset", &self.align_offset)
            .field("disabled", &self.disabled)
            .field("open_menu", &self.open_menu)
            .field("default_open_menu", &self.default_open_menu)
            .field("on_open", &self.on_open.is_some())
            .field("on_close", &self.on_close.is_some())
            .field("on_open_change", &self.on_open_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Menubar<'a, Message> {
    /// Creates an empty menubar.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`. Append menus with [`Self::menu`].
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            menus: Vec::new(),
            width: None,
            side_offset: MENUBAR_SIDE_OFFSET_PX,
            align_offset: MENUBAR_ALIGN_OFFSET_PX,
            disabled: false,
            open_menu: None,
            default_open_menu: None,
            on_open: None,
            on_close: None,
            on_open_change: None,
            style_override: None,
        }
    }

    /// Appends a top-level menu (`Menubar.Menu`).
    pub fn menu(mut self, menu: MenubarMenu<Message>) -> Self {
        self.menus.push(menu);
        self
    }

    /// Sets a fixed content width in px for open menus.
    ///
    /// When unset, content uses the pack's `min-w-*` (Vega `min-w-36`).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(1.0));
        self
    }

    /// Sets the gap between the trigger and the content (`sideOffset`).
    pub fn side_offset(mut self, side_offset: f32) -> Self {
        self.side_offset = side_offset.max(0.0);
        self
    }

    /// Sets the horizontal content shift relative to the trigger (`alignOffset`).
    pub fn align_offset(mut self, align_offset: f32) -> Self {
        self.align_offset = align_offset;
        self
    }

    /// Disables the menubar (`disabled` on the root).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the controlled open menu index (`None` = closed).
    pub fn open_menu(mut self, index: Option<usize>) -> Self {
        self.open_menu = index;
        self
    }

    /// Sets the uncontrolled initial open menu index.
    pub fn default_open_menu(mut self, index: Option<usize>) -> Self {
        self.default_open_menu = index;
        self
    }

    /// Sets the message emitted when any menu opens.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Sets the message emitted when the open menu closes.
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
        style_override: impl Fn(MenubarContentStyle) -> MenubarContentStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

/// Convenience helper that creates an empty menubar.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{MenubarItem, MenubarMenu, Theme, menubar};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Undo,
/// }
///
/// fn app_menu<'a>(theme: &'a Theme) -> Element<'a, Message> {
///     menubar(theme)
///         .menu(
///             MenubarMenu::new("Edit")
///                 .item(MenubarItem::new("Undo").on_select(Message::Undo)),
///         )
///         .into()
/// }
/// ```
pub fn menubar<'a, Message: Clone + 'a>(theme: &'a Theme) -> Menubar<'a, Message> {
    Menubar::new(theme)
}

impl<'a, Message> From<Menubar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(bar: Menubar<'a, Message>) -> Self {
        let Menubar {
            theme,
            menus,
            width,
            side_offset,
            align_offset,
            disabled,
            open_menu,
            default_open_menu,
            on_open,
            on_close,
            on_open_change,
            style_override,
        } = bar;

        Self::new(MenubarWidget {
            theme,
            menus,
            width,
            side_offset,
            align_offset,
            disabled,
            open_override: open_menu,
            default_open_menu,
            on_open,
            on_close,
            on_open_change,
            style_override,
        })
    }
}
