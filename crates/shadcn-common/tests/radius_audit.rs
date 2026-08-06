//! Radius tokens in recipes must match shadcn-svelte `style-*.css`.
//!
//! Source of truth: `nova_refs/svelte/shadcn-svelte/docs/src/lib/registry/styles/`.
//! Token names map to shadcn `--radius-*` (`calc(var(--radius) ± Npx)`), not
//! raw Tailwind 12/16/24/32. Pixel resolution is in `component_radius_px`.
//! Rhea dialog/drawer use `rounded-[min(var(--radius-4xl),24px)]` → 24 (`radius_px`).

use shadcn_common::{
    ComponentRadius, StyleId, alert_dialog_recipe, badge_recipe, button_type, dialog_recipe,
    drawer_recipe, dropdown_menu_recipe, hover_card_recipe, native_select_recipe, popover_recipe,
    progress_recipe, radio_group_recipe, select_recipe, skeleton_default_radius, slider_recipe,
    switch_recipe, toggle_recipe, tooltip_recipe,
};

#[derive(Clone, Copy)]
struct Expected {
    button: ComponentRadius,
    badge: ComponentRadius,
    dd_content: ComponentRadius,
    dd_item: ComponentRadius,
    sel_trig: ComponentRadius,
    sel_content: ComponentRadius,
    sel_item: ComponentRadius,
    native: ComponentRadius,
    popover: ComponentRadius,
    hover: ComponentRadius,
    tooltip: ComponentRadius,
    toggle: ComponentRadius,
    switch: ComponentRadius,
    drawer: ComponentRadius,
    /// `None` means use `dialog.radius`; `Some` means `dialog.radius_px`.
    dialog_px: Option<f32>,
    dialog: ComponentRadius,
    alert_px: Option<f32>,
    alert: ComponentRadius,
    skeleton: ComponentRadius,
    progress: ComponentRadius,
    slider_track: ComponentRadius,
    slider_thumb: ComponentRadius,
    radio: ComponentRadius,
}

const EXPECTED: &[(StyleId, Expected)] = &[
    (
        StyleId::Vega,
        Expected {
            button: ComponentRadius::Md,
            badge: ComponentRadius::S4xl,
            dd_content: ComponentRadius::Md,
            dd_item: ComponentRadius::Sm,
            sel_trig: ComponentRadius::Md,
            sel_content: ComponentRadius::Md,
            sel_item: ComponentRadius::Sm,
            native: ComponentRadius::Md,
            popover: ComponentRadius::Md,
            hover: ComponentRadius::Lg,
            tooltip: ComponentRadius::Md,
            toggle: ComponentRadius::Md,
            switch: ComponentRadius::Full,
            drawer: ComponentRadius::Xl,
            dialog_px: None,
            dialog: ComponentRadius::Xl,
            alert_px: None,
            alert: ComponentRadius::Xl,
            skeleton: ComponentRadius::Md,
            progress: ComponentRadius::Full,
            slider_track: ComponentRadius::Full,
            slider_thumb: ComponentRadius::Full,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Nova,
        Expected {
            button: ComponentRadius::Lg,
            badge: ComponentRadius::S4xl,
            dd_content: ComponentRadius::Lg,
            dd_item: ComponentRadius::Md,
            sel_trig: ComponentRadius::Lg,
            sel_content: ComponentRadius::Lg,
            sel_item: ComponentRadius::Md,
            native: ComponentRadius::Lg,
            popover: ComponentRadius::Lg,
            hover: ComponentRadius::Lg,
            tooltip: ComponentRadius::Md,
            toggle: ComponentRadius::Lg,
            switch: ComponentRadius::Full,
            drawer: ComponentRadius::Xl,
            dialog_px: None,
            dialog: ComponentRadius::Xl,
            alert_px: None,
            alert: ComponentRadius::Xl,
            skeleton: ComponentRadius::Md,
            progress: ComponentRadius::Full,
            slider_track: ComponentRadius::Full,
            slider_thumb: ComponentRadius::Full,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Maia,
        Expected {
            button: ComponentRadius::S4xl,
            badge: ComponentRadius::S4xl,
            dd_content: ComponentRadius::S2xl,
            dd_item: ComponentRadius::Xl,
            sel_trig: ComponentRadius::S4xl,
            sel_content: ComponentRadius::S2xl,
            sel_item: ComponentRadius::Xl,
            native: ComponentRadius::S4xl,
            popover: ComponentRadius::S2xl,
            hover: ComponentRadius::S2xl,
            tooltip: ComponentRadius::S2xl,
            toggle: ComponentRadius::S4xl,
            switch: ComponentRadius::Full,
            drawer: ComponentRadius::S4xl,
            dialog_px: None,
            dialog: ComponentRadius::S4xl,
            alert_px: None,
            alert: ComponentRadius::S4xl,
            skeleton: ComponentRadius::Xl,
            progress: ComponentRadius::S4xl,
            slider_track: ComponentRadius::S4xl,
            slider_thumb: ComponentRadius::S4xl,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Lyra,
        Expected {
            button: ComponentRadius::None,
            badge: ComponentRadius::None,
            dd_content: ComponentRadius::None,
            dd_item: ComponentRadius::None,
            sel_trig: ComponentRadius::None,
            sel_content: ComponentRadius::None,
            sel_item: ComponentRadius::None,
            native: ComponentRadius::None,
            popover: ComponentRadius::None,
            hover: ComponentRadius::None,
            tooltip: ComponentRadius::None,
            toggle: ComponentRadius::None,
            switch: ComponentRadius::Full,
            drawer: ComponentRadius::None,
            dialog_px: None,
            dialog: ComponentRadius::None,
            alert_px: None,
            alert: ComponentRadius::None,
            skeleton: ComponentRadius::None,
            progress: ComponentRadius::None,
            slider_track: ComponentRadius::None,
            slider_thumb: ComponentRadius::None,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Mira,
        Expected {
            button: ComponentRadius::Md,
            badge: ComponentRadius::Full,
            dd_content: ComponentRadius::Lg,
            dd_item: ComponentRadius::Md,
            sel_trig: ComponentRadius::Md,
            sel_content: ComponentRadius::Lg,
            sel_item: ComponentRadius::Md,
            native: ComponentRadius::Md,
            popover: ComponentRadius::Lg,
            hover: ComponentRadius::Lg,
            tooltip: ComponentRadius::Md,
            toggle: ComponentRadius::Md,
            switch: ComponentRadius::Full,
            drawer: ComponentRadius::Xl,
            dialog_px: None,
            dialog: ComponentRadius::Xl,
            alert_px: None,
            alert: ComponentRadius::Xl,
            skeleton: ComponentRadius::Md,
            progress: ComponentRadius::Md,
            slider_track: ComponentRadius::Md,
            slider_thumb: ComponentRadius::Md,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Luma,
        Expected {
            button: ComponentRadius::S4xl,
            badge: ComponentRadius::S3xl,
            dd_content: ComponentRadius::S3xl,
            dd_item: ComponentRadius::S2xl,
            sel_trig: ComponentRadius::S3xl,
            sel_content: ComponentRadius::S3xl,
            sel_item: ComponentRadius::S2xl,
            native: ComponentRadius::S3xl,
            popover: ComponentRadius::S3xl,
            hover: ComponentRadius::S3xl,
            tooltip: ComponentRadius::Xl,
            toggle: ComponentRadius::S3xl,
            switch: ComponentRadius::Full,
            drawer: ComponentRadius::S4xl,
            dialog_px: None,
            dialog: ComponentRadius::S4xl,
            alert_px: None,
            alert: ComponentRadius::S4xl,
            skeleton: ComponentRadius::S2xl,
            progress: ComponentRadius::Full,
            slider_track: ComponentRadius::Full,
            slider_thumb: ComponentRadius::Full,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Sera,
        Expected {
            button: ComponentRadius::None,
            badge: ComponentRadius::None,
            dd_content: ComponentRadius::None,
            dd_item: ComponentRadius::None,
            sel_trig: ComponentRadius::None,
            sel_content: ComponentRadius::None,
            sel_item: ComponentRadius::None,
            native: ComponentRadius::None,
            popover: ComponentRadius::None,
            hover: ComponentRadius::None,
            tooltip: ComponentRadius::None,
            toggle: ComponentRadius::None,
            switch: ComponentRadius::None,
            drawer: ComponentRadius::None,
            dialog_px: None,
            dialog: ComponentRadius::None,
            alert_px: None,
            alert: ComponentRadius::None,
            skeleton: ComponentRadius::None,
            progress: ComponentRadius::None,
            slider_track: ComponentRadius::None,
            slider_thumb: ComponentRadius::None,
            radio: ComponentRadius::Full,
        },
    ),
    (
        StyleId::Rhea,
        Expected {
            button: ComponentRadius::S2xl,
            badge: ComponentRadius::S2xl,
            dd_content: ComponentRadius::S2xl,
            dd_item: ComponentRadius::Xl,
            sel_trig: ComponentRadius::S2xl,
            sel_content: ComponentRadius::S2xl,
            sel_item: ComponentRadius::Xl,
            native: ComponentRadius::S2xl,
            popover: ComponentRadius::S3xl,
            hover: ComponentRadius::S3xl,
            tooltip: ComponentRadius::Xl,
            toggle: ComponentRadius::S2xl,
            switch: ComponentRadius::S2xl,
            drawer: ComponentRadius::S3xl,
            dialog_px: Some(24.0),
            dialog: ComponentRadius::Xl,
            alert_px: Some(24.0),
            alert: ComponentRadius::Xl,
            skeleton: ComponentRadius::S2xl,
            progress: ComponentRadius::S2xl,
            slider_track: ComponentRadius::S2xl,
            slider_thumb: ComponentRadius::S2xl,
            radio: ComponentRadius::S2xl,
        },
    ),
];

#[test]
fn recipe_radii_match_shadcn_svelte_css() {
    for &(style, exp) in EXPECTED {
        let button = button_type(style);
        let badge = badge_recipe(style);
        let dd = dropdown_menu_recipe(style);
        let sel = select_recipe(style);
        let native = native_select_recipe(style);
        let pop = popover_recipe(style);
        let hover = hover_card_recipe(style);
        let tip = tooltip_recipe(style);
        let toggle = toggle_recipe(style);
        let switch = switch_recipe(style);
        let drawer = drawer_recipe(style);
        let dialog = dialog_recipe(style);
        let alert = alert_dialog_recipe(style);
        let progress = progress_recipe(style);
        let slider = slider_recipe(style);
        let radio = radio_group_recipe(style);

        assert_eq!(button.default_radius, exp.button, "{style:?} button");
        assert_eq!(badge.default_radius, exp.badge, "{style:?} badge");
        assert_eq!(
            dd.content_radius, exp.dd_content,
            "{style:?} dropdown content"
        );
        assert_eq!(dd.item_radius, exp.dd_item, "{style:?} dropdown item");
        assert_eq!(sel.trigger_radius, exp.sel_trig, "{style:?} select trigger");
        assert_eq!(
            sel.content_radius, exp.sel_content,
            "{style:?} select content"
        );
        assert_eq!(sel.item_radius, exp.sel_item, "{style:?} select item");
        assert_eq!(native.radius, exp.native, "{style:?} native select");
        assert_eq!(pop.radius, exp.popover, "{style:?} popover");
        assert_eq!(hover.radius, exp.hover, "{style:?} hover-card");
        assert_eq!(tip.radius, exp.tooltip, "{style:?} tooltip");
        assert_eq!(toggle.default_radius, exp.toggle, "{style:?} toggle");
        assert_eq!(switch.default_radius, exp.switch, "{style:?} switch");
        assert_eq!(drawer.radius, exp.drawer, "{style:?} drawer");
        assert_eq!(
            dialog.radius_px, exp.dialog_px,
            "{style:?} dialog radius_px"
        );
        if exp.dialog_px.is_none() {
            assert_eq!(dialog.radius, exp.dialog, "{style:?} dialog");
        }
        assert_eq!(alert.radius_px, exp.alert_px, "{style:?} alert radius_px");
        if exp.alert_px.is_none() {
            assert_eq!(alert.radius, exp.alert, "{style:?} alert");
        }
        assert_eq!(
            skeleton_default_radius(style),
            exp.skeleton,
            "{style:?} skeleton"
        );
        assert_eq!(progress.default_radius, exp.progress, "{style:?} progress");
        assert_eq!(
            slider.track_radius, exp.slider_track,
            "{style:?} slider track"
        );
        assert_eq!(
            slider.thumb_radius, exp.slider_thumb,
            "{style:?} slider thumb"
        );
        assert_eq!(radio.radius, exp.radio, "{style:?} radio");
    }
}
