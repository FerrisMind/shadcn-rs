use iced::Element;

use crate::menu_primitives::{
    MenuContentProps, MenuContentSize, MenuContentVariant, MenuEntry, MenuKind, MenuOverlayProps,
    menu,
};
use crate::theme::Theme;
use crate::tokens::AccentColor;

pub use crate::menu_primitives::{
    MenuCheckboxItem as DropdownMenuCheckboxItem, MenuItem as DropdownMenuItem,
    MenuItemProps as DropdownMenuItemProps, MenuRadioItem as DropdownMenuRadioItem,
    MenuSubMenu as DropdownMenuSubMenu,
};

pub type DropdownMenuContentProps = MenuContentProps;
pub type DropdownMenuContentSize = MenuContentSize;
pub type DropdownMenuContentVariant = MenuContentVariant;
pub type DropdownMenuEntry<'a, Message> = MenuEntry<'a, Message>;

#[derive(Clone, Copy, Debug)]
pub struct DropdownMenuProps {
    pub content: DropdownMenuContentProps,
    pub width: Option<u32>,
    pub offset: f32,
    pub disabled: bool,
}

impl Default for DropdownMenuProps {
    fn default() -> Self {
        Self {
            content: DropdownMenuContentProps::new(),
            width: None,
            offset: 4.0,
            disabled: false,
        }
    }
}

impl DropdownMenuProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: DropdownMenuContentSize) -> Self {
        self.content.size = size;
        self
    }

    pub fn variant(mut self, variant: DropdownMenuContentVariant) -> Self {
        self.content.variant = variant;
        self
    }

    pub fn color(mut self, color: AccentColor) -> Self {
        self.content.color = color;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.content.high_contrast = high_contrast;
        self
    }

    pub fn show_shadow(mut self, show_shadow: bool) -> Self {
        self.content.show_shadow = show_shadow;
        self
    }

    /// Custom corner radius for the dropdown surface.
    pub fn radius(mut self, radius: f32) -> Self {
        self.content = self.content.radius(radius);
        self
    }

    /// Custom corner radius for item hover highlights.
    pub fn item_radius(mut self, item_radius: f32) -> Self {
        self.content = self.content.item_radius(item_radius);
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width.max(1));
        self
    }

    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset.max(0.0);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn dropdown_menu<'a, Message: Clone + 'a>(
    trigger: impl Into<Element<'a, Message>>,
    entries: Vec<DropdownMenuEntry<'a, Message>>,
    props: DropdownMenuProps,
    theme: &Theme,
) -> Element<'a, Message> {
    menu(
        trigger,
        entries,
        props.content,
        MenuOverlayProps {
            kind: MenuKind::Dropdown,
            width: props.width,
            offset: props.offset,
            disabled: props.disabled,
            on_close: None,
        },
        theme,
    )
    .into()
}
