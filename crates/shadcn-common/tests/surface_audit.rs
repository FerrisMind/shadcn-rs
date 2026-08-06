//! Surface color tokens (ring / shadow / fill / border) vs shadcn-svelte CSS.
//!
//! Source: `nova_refs/svelte/shadcn-svelte/docs/src/lib/registry/styles/style-*.css`.
//! Ring alphas are `ring-foreground/N`. Shadows map to [`PopoverShadow`].

use shadcn_common::{
    PopoverShadow, StyleId, alert_dialog_recipe, dialog_recipe, dropdown_menu_recipe,
    hover_card_recipe, popover_recipe, select_recipe,
};

#[derive(Clone, Copy)]
struct SurfaceExpected {
    select_ring: f32,
    select_ring_dark: f32,
    select_shadow: PopoverShadow,
    select_bordered: bool,
    select_fill_light: f32,
    dd_ring: f32,
    dd_ring_dark: f32,
    dd_shadow: PopoverShadow,
    pop_ring: f32,
    pop_ring_dark: f32,
    pop_shadow: PopoverShadow,
    hover_ring: f32,
    hover_ring_dark: f32,
    hover_shadow: PopoverShadow,
    dialog_ring: f32,
    dialog_ring_dark: f32,
    dialog_shadow: Option<PopoverShadow>,
}

const EXPECTED: &[(StyleId, SurfaceExpected)] = &[
    (
        StyleId::Vega,
        SurfaceExpected {
            select_ring: 0.10,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::MD,
            select_bordered: true,
            select_fill_light: 0.0,
            dd_ring: 0.10,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::MD,
            pop_ring: 0.10,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::MD,
            hover_ring: 0.10,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::MD,
            dialog_ring: 0.10,
            dialog_ring_dark: 0.10,
            dialog_shadow: None,
        },
    ),
    (
        StyleId::Nova,
        SurfaceExpected {
            select_ring: 0.10,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::MD,
            select_bordered: true,
            select_fill_light: 0.0,
            dd_ring: 0.10,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::MD,
            pop_ring: 0.10,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::MD,
            hover_ring: 0.10,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::MD,
            dialog_ring: 0.10,
            dialog_ring_dark: 0.10,
            dialog_shadow: None,
        },
    ),
    (
        StyleId::Maia,
        SurfaceExpected {
            // select/popover/hover: ring-foreground/5 only (no dark: variant)
            select_ring: 0.05,
            select_ring_dark: 0.05,
            select_shadow: PopoverShadow::XXL,
            select_bordered: true,
            select_fill_light: 0.3,
            // dropdown: ring-foreground/5 dark:ring-foreground/10
            dd_ring: 0.05,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::XXL,
            pop_ring: 0.05,
            pop_ring_dark: 0.05,
            pop_shadow: PopoverShadow::XXL,
            hover_ring: 0.05,
            hover_ring_dark: 0.05,
            hover_shadow: PopoverShadow::XXL,
            dialog_ring: 0.05,
            dialog_ring_dark: 0.05,
            dialog_shadow: None,
        },
    ),
    (
        StyleId::Lyra,
        SurfaceExpected {
            select_ring: 0.10,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::MD,
            select_bordered: true,
            select_fill_light: 0.0,
            dd_ring: 0.10,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::MD,
            pop_ring: 0.10,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::MD,
            hover_ring: 0.10,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::MD,
            dialog_ring: 0.10,
            dialog_ring_dark: 0.10,
            dialog_shadow: None,
        },
    ),
    (
        StyleId::Mira,
        SurfaceExpected {
            select_ring: 0.10,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::MD,
            select_bordered: true,
            select_fill_light: 0.2,
            dd_ring: 0.10,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::MD,
            pop_ring: 0.10,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::MD,
            hover_ring: 0.10,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::MD,
            dialog_ring: 0.10,
            dialog_ring_dark: 0.10,
            dialog_shadow: None,
        },
    ),
    (
        StyleId::Luma,
        SurfaceExpected {
            select_ring: 0.05,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::LG,
            select_bordered: false,
            select_fill_light: 0.5,
            dd_ring: 0.05,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::LG,
            pop_ring: 0.05,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::LG,
            hover_ring: 0.05,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::LG,
            dialog_ring: 0.05,
            dialog_ring_dark: 0.10,
            dialog_shadow: Some(PopoverShadow::XL),
        },
    ),
    (
        StyleId::Sera,
        SurfaceExpected {
            select_ring: 0.10,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::MD,
            select_bordered: true,
            select_fill_light: 0.0,
            dd_ring: 0.10,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::MD,
            pop_ring: 0.10,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::MD,
            hover_ring: 0.10,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::MD,
            dialog_ring: 0.10,
            dialog_ring_dark: 0.10,
            dialog_shadow: Some(PopoverShadow::MD),
        },
    ),
    (
        StyleId::Rhea,
        SurfaceExpected {
            select_ring: 0.05,
            select_ring_dark: 0.10,
            select_shadow: PopoverShadow::LG,
            select_bordered: false,
            select_fill_light: 0.5,
            dd_ring: 0.05,
            dd_ring_dark: 0.10,
            dd_shadow: PopoverShadow::LG,
            pop_ring: 0.05,
            pop_ring_dark: 0.10,
            pop_shadow: PopoverShadow::LG,
            hover_ring: 0.05,
            hover_ring_dark: 0.10,
            hover_shadow: PopoverShadow::LG,
            dialog_ring: 0.05,
            dialog_ring_dark: 0.10,
            dialog_shadow: Some(PopoverShadow::XL),
        },
    ),
];

#[test]
fn surface_color_tokens_match_shadcn_svelte_css() {
    for &(style, exp) in EXPECTED {
        let sel = select_recipe(style);
        let dd = dropdown_menu_recipe(style);
        let pop = popover_recipe(style);
        let hover = hover_card_recipe(style);
        let dialog = dialog_recipe(style);
        let alert = alert_dialog_recipe(style);

        assert_eq!(
            sel.content_ring_alpha, exp.select_ring,
            "{style:?} select ring"
        );
        assert_eq!(
            sel.content_ring_alpha_dark, exp.select_ring_dark,
            "{style:?} select ring dark"
        );
        assert_eq!(
            sel.content_shadow, exp.select_shadow,
            "{style:?} select shadow"
        );
        assert_eq!(
            sel.bordered, exp.select_bordered,
            "{style:?} select bordered"
        );
        assert_eq!(
            sel.fill_alpha_light, exp.select_fill_light,
            "{style:?} select fill light"
        );

        assert_eq!(
            dd.content_ring_alpha, exp.dd_ring,
            "{style:?} dropdown ring"
        );
        assert_eq!(
            dd.content_ring_alpha_dark, exp.dd_ring_dark,
            "{style:?} dropdown ring dark"
        );
        assert_eq!(
            dd.content_shadow, exp.dd_shadow,
            "{style:?} dropdown shadow"
        );

        assert_eq!(pop.ring_alpha, exp.pop_ring, "{style:?} popover ring");
        assert_eq!(
            pop.ring_alpha_dark, exp.pop_ring_dark,
            "{style:?} popover ring dark"
        );
        assert_eq!(pop.shadow, exp.pop_shadow, "{style:?} popover shadow");

        assert_eq!(hover.ring_alpha, exp.hover_ring, "{style:?} hover ring");
        assert_eq!(
            hover.ring_alpha_dark, exp.hover_ring_dark,
            "{style:?} hover ring dark"
        );
        assert_eq!(hover.shadow, exp.hover_shadow, "{style:?} hover shadow");

        assert_eq!(dialog.ring_alpha, exp.dialog_ring, "{style:?} dialog ring");
        assert_eq!(
            dialog.ring_alpha_dark, exp.dialog_ring_dark,
            "{style:?} dialog ring dark"
        );
        assert_eq!(dialog.shadow, exp.dialog_shadow, "{style:?} dialog shadow");
        assert_eq!(alert.ring_alpha, exp.dialog_ring, "{style:?} alert ring");
        assert_eq!(
            alert.ring_alpha_dark, exp.dialog_ring_dark,
            "{style:?} alert ring dark"
        );
        assert_eq!(alert.shadow, exp.dialog_shadow, "{style:?} alert shadow");
    }
}
