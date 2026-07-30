//! Behavioral tests for the menubar component.

use crate::iced_compat::Element;
use shadcn_common::{MenuActivateKind, MenuItemVariant, StyleId, menubar_recipe};

use super::types::Entry;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    NewTab,
    ToggleBookmarks,
    ThemeLight,
    Opened,
    Closed,
    OpenChanged(bool),
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let bar = Menubar::new(&theme)
        .menu(
            MenubarMenu::new("File")
                .item(
                    MenubarItem::new("New Tab")
                        .shortcut("⌘T")
                        .on_select(Message::NewTab),
                )
                .checkbox_item(
                    MenubarCheckboxItem::new("Bookmarks", true).on_toggle(Message::ToggleBookmarks),
                )
                .radio_item(MenubarRadioItem::new("Light", true).on_select(Message::ThemeLight))
                .separator()
                .label("Account")
                .item(
                    MenubarItem::new("Sign out")
                        .variant(MenuItemVariant::Destructive)
                        .on_select(Message::NewTab),
                )
                .submenu(
                    MenubarSub::new("More")
                        .item(MenubarItem::new("Nested").on_select(Message::NewTab)),
                ),
        )
        .menu(MenubarMenu::new("Edit").item(MenubarItem::new("Undo").on_select(Message::NewTab)))
        .width(224.0)
        .side_offset(8.0)
        .align_offset(-4.0)
        .disabled(false)
        .default_open_menu(Some(0))
        .on_open(Message::Opened)
        .on_close(Message::Closed)
        .on_open_change(Message::OpenChanged)
        .style_override(|style| style);

    assert_eq!(bar.menus.len(), 2);
    assert_eq!(bar.menus[0].trigger(), "File");
    assert_eq!(bar.menus[0].len(), 7);
    assert_eq!(bar.menus[1].trigger(), "Edit");
    assert_eq!(bar.width, Some(224.0));
    assert_eq!(bar.side_offset, 8.0);
    assert_eq!(bar.align_offset, -4.0);
    assert_eq!(bar.default_open_menu, Some(0));
    assert_eq!(bar.on_open, Some(Message::Opened));
    assert_eq!(bar.on_close, Some(Message::Closed));
    assert!(bar.on_open_change.is_some());
    assert!(bar.style_override.is_some());
    assert!(matches!(&bar.menus[0].entries[0], Entry::Item(item) if item.label == "New Tab"));
    assert!(matches!(&bar.menus[0].entries[1], Entry::Checkbox(item) if item.checked));
    assert!(matches!(&bar.menus[0].entries[2], Entry::Radio(item) if item.selected));
    assert!(matches!(&bar.menus[0].entries[3], Entry::Separator));
    assert!(matches!(&bar.menus[0].entries[4], Entry::Label(label) if label.text() == "Account"));
    assert!(matches!(
        &bar.menus[0].entries[5],
        Entry::Item(item) if item.variant == MenuItemVariant::Destructive
    ));
    assert!(matches!(&bar.menus[0].entries[6], Entry::Sub(sub) if sub.label_text() == "More"));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Menubar::new(&theme)
        .menu(MenubarMenu::new("File").item(MenubarItem::new("New Tab").on_select(Message::NewTab)))
        .into();

    let _: Element<'_, Message> = menubar(&theme)
        .menu(MenubarMenu::new("Edit").item(MenubarItem::new("Undo").on_select(Message::NewTab)))
        .into();
}

#[test]
fn disabled_and_separator_are_not_selectable() {
    let menu = MenubarMenu::new("File")
        .item(MenubarItem::new("Ok").on_select(Message::NewTab))
        .item(
            MenubarItem::new("Nope")
                .disabled(true)
                .on_select(Message::NewTab),
        )
        .separator()
        .label("Heading");

    assert!(menu.entries[0].is_selectable());
    assert!(!menu.entries[1].is_selectable());
    assert!(!menu.entries[2].is_selectable());
    assert!(!menu.entries[3].is_selectable());
}

#[test]
fn activate_kind_close_policy_matches_bits_ui() {
    assert!(MenuActivateKind::Item.closes_menu_by_default());
    assert!(MenuActivateKind::Radio.closes_menu_by_default());
    assert!(!MenuActivateKind::Checkbox.closes_menu_by_default());
    assert!(!MenuActivateKind::SubTrigger.closes_menu_by_default());
}

#[test]
fn menubar_recipe_vega_matches_web() {
    let recipe = menubar_recipe(StyleId::Vega);
    assert_eq!(recipe.bar_height_px, 36.0);
    assert_eq!(recipe.menu.content_min_width_px, 144.0);
    assert!(recipe.indicator_leading);
    assert_eq!(recipe.item_indicator_left_px, 8.0);
}
