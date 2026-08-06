use super::types::matches_query;
use super::{EmojiPicker, EmojiPickerCategory, EmojiPickerRecents, EmojiPickerSkin, SelectedEmoji};
use crate::theme::Theme;
use shadcn_common::StyleId;
use std::cell::Cell;

#[derive(Debug, Clone)]
enum Message {
    Query,
    Selected,
    Skin,
}

#[test]
fn skin_cycles_like_reference() {
    assert_eq!(EmojiPickerSkin::Default.next(), EmojiPickerSkin::Light);
    assert_eq!(EmojiPickerSkin::Dark.next(), EmojiPickerSkin::Default);
    assert_eq!(
        EmojiPickerSkin::from_index(3),
        Some(EmojiPickerSkin::Medium)
    );
    assert_eq!(EmojiPickerSkin::from_index(6), None);
}

#[test]
fn categories_match_reference_count_and_titles() {
    assert_eq!(EmojiPickerCategory::ALL.len(), 8);
    assert_eq!(EmojiPickerCategory::ALL[0].title(), "People");
    assert_eq!(EmojiPickerCategory::ALL[7].title(), "Flags");
}

#[test]
fn recents_are_validated_and_sorted_by_frecency() {
    let mut recents = EmojiPickerRecents::new();
    assert!(recents.record_emoji("😀"));
    assert!(recents.record_emoji("🚀"));
    assert!(recents.record_emoji("😀"));
    assert!(!recents.record_emoji("not an emoji"));
    assert_eq!(recents.entries()[0].emoji(), "😀");
    assert_eq!(recents.entries()[0].uses(), 2);
    assert_eq!(recents.len(), 2);
}

#[test]
fn selected_value_contains_catalog_metadata_and_skin() {
    let selected = SelectedEmoji::from_native("👍🏽", EmojiPickerSkin::Default).unwrap();
    assert_eq!(selected.emoji(), "👍🏽");
    assert_eq!(selected.skin(), EmojiPickerSkin::Medium);
    assert_eq!(selected.data().id(), "+1");
    assert_eq!(selected.data().name(), "thumbs up");
    assert_eq!(selected.data().skin_count(), 6);
    assert!(selected.data().has_skin_tones());
}

#[test]
fn search_matches_keyword_tokens_inside_catalog_names() {
    let waving_hand = emojis::get("👋").unwrap();
    assert!(matches_query(waving_hand, "hand"));
    assert!(matches_query(waving_hand, "wave"));
    assert!(!matches_query(waving_hand, "rocket"));
}

#[test]
fn picker_is_controlled_and_builds_with_all_callbacks() {
    let theme = Theme::light();
    let picker = EmojiPicker::<Message>::new(&theme)
        .value("😀")
        .query("hand")
        .skin(EmojiPickerSkin::Medium)
        .show_recents(true)
        .max_recents(8)
        .on_query_change(|_| Message::Query)
        .on_select(|_| Message::Selected)
        .on_skin_change(|_| Message::Skin);

    let _element: crate::iced_compat::Element<'_, Message> = picker.into();
}

#[test]
fn viewport_radius_follows_the_active_style_pack() {
    let cases = [
        (StyleId::Vega, 8.0),
        (StyleId::Nova, 10.0),
        (StyleId::Maia, 18.0),
        (StyleId::Lyra, 0.0),
        (StyleId::Mira, 10.0),
        (StyleId::Luma, 22.0),
        (StyleId::Sera, 0.0),
        (StyleId::Rhea, 22.0),
    ];

    for (style_id, expected) in cases {
        let theme = Theme::light().with_style(style_id);
        assert_eq!(super::super::popover::surface_radius(&theme), expected);
    }
}

#[test]
fn selection_callback_is_lazy_until_a_cell_is_pressed() {
    let theme = Theme::light();
    let calls = Cell::new(0);
    let picker = EmojiPicker::<Message>::new(&theme).on_select(|_| {
        calls.set(calls.get() + 1);
        Message::Selected
    });

    let _element: crate::iced_compat::Element<'_, Message> = picker.into();
    assert_eq!(calls.get(), 0);
}
