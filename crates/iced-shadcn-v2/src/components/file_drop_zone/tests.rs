//! Unit tests for the file-drop-zone builder and helpers.

use super::*;
use crate::recipes::component_radius_px;
use shadcn_common::{ACCEPT_IMAGE, MEGABYTE, StyleId};

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Message {
    Zone(FileDropZoneAction),
}

#[test]
fn builder_defaults_match_extras() {
    let theme = Theme::light();
    let state = FileDropZoneState::new();
    let zone = FileDropZone::<Message>::new(&theme, &state)
        .max_files(4)
        .file_count(0)
        .max_file_size(3 * MEGABYTE)
        .accept(ACCEPT_IMAGE)
        .on_action(Message::Zone);

    assert_eq!(zone.config.max_files, Some(4));
    assert_eq!(zone.config.file_count, Some(0));
    assert_eq!(zone.config.max_file_size, Some(3 * MEGABYTE));
    assert_eq!(zone.config.accept.as_deref(), Some(ACCEPT_IMAGE));
    assert_eq!(zone.variant, FileDropZoneVariant::Default);
}

#[test]
fn state_apply_tracks_hover() {
    let mut state = FileDropZoneState::new();
    state.apply(&FileDropZoneAction::Hovered(true));
    assert!(state.hovered);
    state.apply(&FileDropZoneAction::Hovered(false));
    assert!(!state.hovered);
}

#[test]
fn partition_paths_rejects_wrong_type() {
    let config = FileDropZoneConfig::new().with_accept(ACCEPT_IMAGE);
    let (accepted, rejected) = partition_paths(vec![std::path::PathBuf::from("clip.mp4")], &config);
    assert!(accepted.is_empty());
    assert_eq!(rejected.len(), 1);
}

#[test]
fn composed_look_follows_theme_style_pack() {
    // Extras FileDropZone has no pack-specific tables, but selecting Rhea on
    // the shared Theme must still change resolved radius and composed Button.
    let vega = Theme::light().with_style(StyleId::Vega);
    let rhea = Theme::light().with_style(StyleId::Rhea);
    let sera = Theme::light().with_style(StyleId::Sera);

    assert_eq!(vega.style.file_drop_zone(), rhea.style.file_drop_zone());
    assert_eq!(vega.style.file_drop_zone(), sera.style.file_drop_zone());

    let recipe = vega.style.file_drop_zone();
    let vega_r = component_radius_px(&vega, recipe.radius);
    let _rhea_r = component_radius_px(&rhea, recipe.radius);
    let sera_r = component_radius_px(&sera, recipe.radius);
    // Sera locks radii to 0; Vega (and Rhea) resolve a non-zero lg slot.
    assert_eq!(sera_r, 0.0);
    assert_ne!(vega_r, sera_r);
    assert_ne!(rhea.style.button_type(), sera.style.button_type());

    let state = FileDropZoneState::new();
    let _zone = FileDropZone::<Message>::new(&rhea, &state).on_action(Message::Zone);
    assert_eq!(rhea.style_id(), StyleId::Rhea);
}
