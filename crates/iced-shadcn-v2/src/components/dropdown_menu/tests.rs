//! Behavioral tests for the dropdown-menu component.

use crate::iced_compat::Element;
use shadcn_common::{MenuActivateKind, MenuItemVariant, StyleId, dropdown_menu_recipe};

use super::types::Entry;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Profile,
    ToggleEmail,
    ThemeLight,
    Opened,
    Closed,
    OpenChanged(bool),
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let menu = DropdownMenu::new(&theme)
        .trigger_label("Open")
        .item(
            DropdownMenuItem::new("Profile")
                .shortcut("⇧⌘P")
                .on_select(Message::Profile),
        )
        .checkbox_item(DropdownMenuCheckboxItem::new("Email", true).on_toggle(Message::ToggleEmail))
        .radio_item(DropdownMenuRadioItem::new("Light", true).on_select(Message::ThemeLight))
        .separator()
        .label("Account")
        .item(
            DropdownMenuItem::new("Sign out")
                .variant(MenuItemVariant::Destructive)
                .on_select(Message::Profile),
        )
        .submenu(
            DropdownMenuSub::new("More")
                .item(DropdownMenuItem::new("Nested").on_select(Message::Profile)),
        )
        .width(224.0)
        .side_offset(8.0)
        .disabled(false)
        .default_open(true)
        .on_open(Message::Opened)
        .on_close(Message::Closed)
        .on_open_change(Message::OpenChanged)
        .style_override(|style| style);

    assert_eq!(menu.trigger_label.as_deref(), Some("Open"));
    assert_eq!(menu.entries.len(), 7);
    assert_eq!(menu.width, Some(224.0));
    assert_eq!(menu.side_offset, 8.0);
    assert!(menu.default_open);
    assert_eq!(menu.on_open, Some(Message::Opened));
    assert_eq!(menu.on_close, Some(Message::Closed));
    assert!(menu.on_open_change.is_some());
    assert!(menu.style_override.is_some());
    assert!(matches!(&menu.entries[0], Entry::Item(item) if item.label == "Profile"));
    assert!(matches!(&menu.entries[1], Entry::Checkbox(item) if item.checked));
    assert!(matches!(&menu.entries[2], Entry::Radio(item) if item.selected));
    assert!(matches!(&menu.entries[3], Entry::Separator));
    assert!(matches!(&menu.entries[4], Entry::Label(label) if label.text() == "Account"));
    assert!(matches!(
        &menu.entries[5],
        Entry::Item(item) if item.variant == MenuItemVariant::Destructive
    ));
    assert!(matches!(&menu.entries[6], Entry::Sub(sub) if sub.label_text() == "More"));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = DropdownMenu::new(&theme)
        .trigger_label("Open")
        .item(DropdownMenuItem::new("Profile").on_select(Message::Profile))
        .into();

    let _: Element<'_, Message> = dropdown_menu("Open", &theme)
        .item(DropdownMenuItem::new("Profile").on_select(Message::Profile))
        .into();
}

#[test]
fn disabled_and_separator_are_not_selectable() {
    let theme = Theme::light();
    let menu = DropdownMenu::new(&theme)
        .item(DropdownMenuItem::new("Ok").on_select(Message::Profile))
        .item(
            DropdownMenuItem::new("Nope")
                .disabled(true)
                .on_select(Message::Profile),
        )
        .separator()
        .label("Heading");

    assert!(menu.entries[0].is_selectable());
    assert!(!menu.entries[1].is_selectable());
    assert!(!menu.entries[2].is_selectable());
    assert!(!menu.entries[3].is_selectable());
}

#[test]
fn close_on_select_defaults_match_bits_ui() {
    assert!(MenuActivateKind::Item.closes_menu_by_default());
    assert!(!MenuActivateKind::Checkbox.closes_menu_by_default());
    assert!(MenuActivateKind::Radio.closes_menu_by_default());

    let item = DropdownMenuItem::<Message>::new("Profile");
    assert!(item.close_on_select);

    let checkbox = DropdownMenuCheckboxItem::<Message>::new("Email", false);
    assert!(checkbox.on_toggle.is_none());

    let radio = DropdownMenuRadioItem::<Message>::new("Light", false);
    assert!(radio.close_on_select);
}

#[test]
fn recipe_packs_resolve() {
    for style in StyleId::ALL {
        let recipe = dropdown_menu_recipe(style);
        assert!(recipe.content_min_width_px >= 96.0);
        assert!(recipe.item_pad_x_px > 0.0);
    }
}

#[test]
fn dense_menu_content_height_exceeds_legacy_max_h_96() {
    // Regression: a docs-sized menu (~15 rows) is taller than `max-h-96`
    // (384px). Capping the surface there without a scrollport left the last
    // rows painted outside the white panel.
    let theme = Theme::light();
    let menu: DropdownMenu<'_, Message> = DropdownMenu::new(&theme)
        .label("My Account")
        .item(DropdownMenuItem::new("Profile").shortcut("⇧⌘P"))
        .item(DropdownMenuItem::new("Billing").shortcut("⌘B"))
        .item(DropdownMenuItem::new("Settings").shortcut("⌘S"))
        .item(DropdownMenuItem::new("Keyboard shortcuts").shortcut("⌘K"))
        .separator()
        .item(DropdownMenuItem::new("Team"))
        .submenu(DropdownMenuSub::new("Invite users").item(DropdownMenuItem::new("Email")))
        .item(DropdownMenuItem::new("New Team").shortcut("⌘+T"))
        .separator()
        .item(DropdownMenuItem::new("GitHub"))
        .item(DropdownMenuItem::new("Support"))
        .item(DropdownMenuItem::new("API").disabled(true))
        .separator()
        .item(DropdownMenuItem::new("Log out").shortcut("⇧⌘Q"));

    let recipe = dropdown_menu_recipe(theme.style_id());
    let height = super::render::menu_height(&menu.entries, recipe);
    assert!(
        height > 384.0,
        "expected dense menu taller than max-h-96, got {height}"
    );
}

#[test]
fn rhea_theme_resolves_pack_specific_dropdown_chrome() {
    // DropdownMenu (unlike Form) has pack-specific CSS in shadcn-svelte.
    // Selecting Rhea on Theme must change content/item radii vs Vega — apps
    // must not hard-code those numbers; they come from the recipe.
    let vega = Theme::dark().with_style(StyleId::Vega);
    let rhea = Theme::dark().with_style(StyleId::Rhea);

    let vega_style = dropdown_menu_content_style(&vega, false);
    let rhea_style = dropdown_menu_content_style(&rhea, false);
    let vega_recipe = dropdown_menu_recipe(StyleId::Vega);
    let rhea_recipe = dropdown_menu_recipe(StyleId::Rhea);

    assert_ne!(vega_recipe.item_radius, rhea_recipe.item_radius);
    assert_ne!(vega_recipe.content_radius, rhea_recipe.content_radius);
    assert_ne!(vega_style.item_radius, rhea_style.item_radius);
    assert_ne!(vega_style.radius, rhea_style.radius);
    assert!(rhea_style.item_radius > vega_style.item_radius);
    assert!(rhea_style.radius > vega_style.radius);
    assert_eq!(rhea_recipe.item_min_height_px, 28.0);
    assert!(rhea_recipe.separator_muted);
    assert!(!vega_recipe.separator_muted);
}

#[test]
fn form_like_composites_without_pack_deltas_still_follow_theme_parts() {
    // Sanity mirror of Form's compose rule: pack-invariant host recipe must
    // not block StyleId from reaching composed control recipes.
    let vega = Theme::light().with_style(StyleId::Vega);
    let rhea = Theme::light().with_style(StyleId::Rhea);
    assert_eq!(vega.style.form(), rhea.style.form());
    assert_ne!(vega.style.button_type(), rhea.style.button_type());
    assert_eq!(rhea.style_id(), StyleId::Rhea);
}
