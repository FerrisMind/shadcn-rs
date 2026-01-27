use iced::{Element};

use crate::menu_primitives::{
    MenuContentProps, MenuContentSize, MenuContentVariant, MenuEntry, MenuOverlayProps, MenuKind,
    menu,
};
use crate::theme::Theme;
use crate::tokens::AccentColor;

pub use crate::menu_primitives::{
    MenuCheckboxItem as ContextMenuCheckboxItem, MenuItem as ContextMenuItem,
    MenuItemProps as ContextMenuItemProps, MenuRadioItem as ContextMenuRadioItem,
    MenuSubMenu as ContextMenuSubMenu,
};

pub type ContextMenuContentProps = MenuContentProps;
pub type ContextMenuContentSize = MenuContentSize;
pub type ContextMenuContentVariant = MenuContentVariant;
pub type ContextMenuEntry<'a, Message> = MenuEntry<'a, Message>;

#[derive(Clone, Copy, Debug)]
pub struct ContextMenuProps {
    pub content: ContextMenuContentProps,
    pub width: Option<u32>,
    pub disabled: bool,
}

impl Default for ContextMenuProps {
    fn default() -> Self {
        Self {
            content: ContextMenuContentProps::new(),
            width: None,
            disabled: false,
        }
    }
}

impl ContextMenuProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: ContextMenuContentSize) -> Self {
        self.content.size = size;
        self
    }

    pub fn variant(mut self, variant: ContextMenuContentVariant) -> Self {
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

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width.max(1));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn context_menu<'a, Message: Clone + 'a>(
    trigger: impl Into<Element<'a, Message>>,
    entries: Vec<ContextMenuEntry<'a, Message>>,
    props: ContextMenuProps,
    theme: &Theme,
) -> Element<'a, Message> {
    menu(
        trigger,
        entries,
        props.content,
        MenuOverlayProps {
            kind: MenuKind::Context,
            width: props.width,
            offset: 0.0,
            disabled: props.disabled,
        },
        theme,
    )
    .into()
}

