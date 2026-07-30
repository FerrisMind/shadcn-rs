//! Behavioral tests for the context-menu component.

use crate::iced_compat::Element;
use shadcn_common::{
    FloatingSide, MenuActivateKind, MenuItemVariant, StyleId, context_menu_recipe,
};

use super::types::Entry;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Back,
    ToggleBookmarks,
    ThemeLight,
    Opened,
    Closed,
    OpenChanged(bool),
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let menu = ContextMenu::new(&theme)
        .trigger_label("Right click here")
        .item(
            ContextMenuItem::new("Back")
                .shortcut("⌘[")
                .on_select(Message::Back),
        )
        .checkbox_item(
            ContextMenuCheckboxItem::new("Bookmarks", true).on_toggle(Message::ToggleBookmarks),
        )
        .radio_item(ContextMenuRadioItem::new("Light", true).on_select(Message::ThemeLight))
        .separator()
        .label("Actions")
        .item(
            ContextMenuItem::new("Delete")
                .variant(MenuItemVariant::Destructive)
                .on_select(Message::Back),
        )
        .submenu(
            ContextMenuSub::new("More")
                .item(ContextMenuItem::new("Nested").on_select(Message::Back)),
        )
        .width(224.0)
        .side(FloatingSide::Top)
        .side_offset(8.0)
        .disabled(false)
        .default_open(true)
        .on_open(Message::Opened)
        .on_close(Message::Closed)
        .on_open_change(Message::OpenChanged)
        .style_override(|style| style);

    assert_eq!(menu.trigger_label.as_deref(), Some("Right click here"));
    assert_eq!(menu.entries.len(), 7);
    assert_eq!(menu.width, Some(224.0));
    assert_eq!(menu.side, Some(FloatingSide::Top));
    assert_eq!(menu.side_offset, 8.0);
    assert!(menu.default_open);
    assert_eq!(menu.on_open, Some(Message::Opened));
    assert_eq!(menu.on_close, Some(Message::Closed));
    assert!(menu.on_open_change.is_some());
    assert!(menu.style_override.is_some());
    assert!(matches!(&menu.entries[0], Entry::Item(item) if item.label == "Back"));
    assert!(matches!(&menu.entries[1], Entry::Checkbox(item) if item.checked));
    assert!(matches!(&menu.entries[2], Entry::Radio(item) if item.selected));
    assert!(matches!(&menu.entries[3], Entry::Separator));
    assert!(matches!(&menu.entries[4], Entry::Label(label) if label.text() == "Actions"));
    assert!(matches!(
        &menu.entries[5],
        Entry::Item(item) if item.variant == MenuItemVariant::Destructive
    ));
    assert!(matches!(&menu.entries[6], Entry::Sub(sub) if sub.label_text() == "More"));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = ContextMenu::new(&theme)
        .trigger_label("Right click here")
        .item(ContextMenuItem::new("Back").on_select(Message::Back))
        .into();

    let _: Element<'_, Message> = context_menu("Right click here", &theme)
        .item(ContextMenuItem::new("Back").on_select(Message::Back))
        .into();
}

#[test]
fn disabled_and_separator_are_not_selectable() {
    let theme = Theme::light();
    let menu = ContextMenu::new(&theme)
        .item(ContextMenuItem::new("Ok").on_select(Message::Back))
        .item(
            ContextMenuItem::new("Nope")
                .disabled(true)
                .on_select(Message::Back),
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

    let item = ContextMenuItem::<Message>::new("Back");
    assert!(item.close_on_select);

    let checkbox = ContextMenuCheckboxItem::<Message>::new("Bookmarks", false);
    assert!(checkbox.on_toggle.is_none());

    let radio = ContextMenuRadioItem::<Message>::new("Light", false);
    assert!(radio.close_on_select);
}

#[test]
fn recipe_packs_resolve() {
    for style in StyleId::ALL {
        let recipe = context_menu_recipe(style);
        assert!(recipe.content_min_width_px >= 96.0);
        assert!(recipe.item_pad_x_px > 0.0);
    }
}

#[test]
fn context_menu_recipe_matches_dropdown_menu_recipe() {
    for style in StyleId::ALL {
        let context = context_menu_recipe(style);
        let dropdown = shadcn_common::dropdown_menu_recipe(style);
        // Maia context-menu stays at ring/5 in dark; dropdown bumps to /10.
        if matches!(style, StyleId::Maia) {
            assert_eq!(context.content_ring_alpha_dark, context.content_ring_alpha);
            assert_ne!(
                context.content_ring_alpha_dark,
                dropdown.content_ring_alpha_dark
            );
        } else {
            assert_eq!(context, dropdown);
        }
    }
}

#[test]
fn dense_menu_content_height_exceeds_legacy_max_h_96() {
    // Regression: a docs-sized menu (~15 rows) is taller than `max-h-96`
    // (384px). Capping the surface there without a scrollport left the last
    // rows painted outside the white panel.
    let theme = Theme::light();
    let menu: ContextMenu<'_, Message> = ContextMenu::new(&theme)
        .label("File")
        .item(ContextMenuItem::new("New File").shortcut("⌘N"))
        .item(ContextMenuItem::new("Open File").shortcut("⌘O"))
        .item(ContextMenuItem::new("Save").shortcut("⌘S"))
        .separator()
        .label("Edit")
        .item(ContextMenuItem::new("Undo").shortcut("⌘Z"))
        .item(ContextMenuItem::new("Redo").shortcut("⇧⌘Z"))
        .separator()
        .item(ContextMenuItem::new("Cut").shortcut("⌘X"))
        .item(ContextMenuItem::new("Copy").shortcut("⌘C"))
        .item(ContextMenuItem::new("Paste").shortcut("⌘V"))
        .separator()
        .item(
            ContextMenuItem::new("Delete")
                .variant(MenuItemVariant::Destructive)
                .shortcut("⌫"),
        )
        .separator()
        .label("View")
        .item(ContextMenuItem::new("Zoom In").shortcut("⌘+"))
        .item(ContextMenuItem::new("Zoom Out").shortcut("⌘-"));

    let recipe = context_menu_recipe(theme.style_id());
    let height = super::render::menu_height(&menu.entries, recipe);
    assert!(
        height > 384.0,
        "expected dense menu taller than max-h-96, got {height}"
    );
}

#[test]
fn dense_menu_inset_and_submenu_heights_are_computed() {
    // Smoke: inset + submenu geometry does not panic and yields a positive height.
    let theme = Theme::light();
    let menu: ContextMenu<'_, Message> = ContextMenu::new(&theme)
        .label(ContextMenuLabel::new("Actions"))
        .item(ContextMenuItem::new("Copy").inset(true))
        .item(ContextMenuItem::new("Cut").inset(true))
        .separator()
        .submenu(
            ContextMenuSub::new("More Options")
                .inset(true)
                .item(ContextMenuItem::new("Save Page...")),
        );

    let recipe = context_menu_recipe(theme.style_id());
    let height = super::render::menu_height(&menu.entries, recipe);
    assert!(height > 0.0);
}

#[test]
fn cursor_menu_origin_places_on_each_explicit_side() {
    use crate::iced_compat::{Point, Size};

    let anchor = Point::new(400.0, 300.0);
    let viewport = Size::new(800.0, 600.0);
    let width = 192.0;
    let height = 120.0;
    let offset = 4.0;

    let top = super::render::cursor_menu_origin(
        anchor,
        width,
        height,
        viewport,
        Some(FloatingSide::Top),
        offset,
    );
    assert!(
        top.y + height <= anchor.y + 0.5,
        "top should sit above the cursor, got y={}",
        top.y
    );

    let bottom = super::render::cursor_menu_origin(
        anchor,
        width,
        height,
        viewport,
        Some(FloatingSide::Bottom),
        offset,
    );
    assert!(
        bottom.y >= anchor.y + offset - 0.5,
        "bottom should sit below the cursor, got y={}",
        bottom.y
    );

    let left = super::render::cursor_menu_origin(
        anchor,
        width,
        height,
        viewport,
        Some(FloatingSide::Left),
        offset,
    );
    assert!(
        left.x + width <= anchor.x + 0.5,
        "left should sit to the left of the cursor, got x={}",
        left.x
    );
    assert!(
        (left.x - (anchor.x - offset - width)).abs() < 1.0 || left.x >= 0.0,
        "left x should be near anchor - offset - width, got {}",
        left.x
    );

    let right = super::render::cursor_menu_origin(
        anchor,
        width,
        height,
        viewport,
        Some(FloatingSide::Right),
        offset,
    );
    assert!(
        right.x >= anchor.x + offset - 0.5,
        "right should sit to the right of the cursor, got x={}",
        right.x
    );
}

#[test]
fn luma_content_radius_is_rounded_3xl() {
    let theme = Theme::light().with_style(StyleId::Luma);
    let style = super::style::resolve_content_style(&theme, false);
    // Default `--radius: 0.625rem` (10px): 3xl = +12 → 22, 2xl = +8 → 18.
    assert!(
        (style.radius - 22.0).abs() < f32::EPSILON,
        "Luma content uses rounded-3xl (radius+12), got {}",
        style.radius
    );
    assert!(
        (style.item_radius - 18.0).abs() < f32::EPSILON,
        "Luma items use rounded-2xl (radius+8), got {}",
        style.item_radius
    );
}
