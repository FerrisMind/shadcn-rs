//! Progress recipes mirror the style CSS shipped by shadcn-svelte.

use shadcn_common::{ComponentRadius, StyleId, progress_recipe};

#[test]
fn progress_geometry_matches_style_css() {
    let expected = [
        (StyleId::Vega, 6.0, ComponentRadius::Full),
        (StyleId::Nova, 4.0, ComponentRadius::Full),
        (StyleId::Maia, 12.0, ComponentRadius::S4xl),
        (StyleId::Lyra, 4.0, ComponentRadius::None),
        (StyleId::Mira, 4.0, ComponentRadius::Md),
        (StyleId::Luma, 12.0, ComponentRadius::Full),
        (StyleId::Sera, 2.0, ComponentRadius::None),
        (StyleId::Rhea, 8.0, ComponentRadius::S2xl),
    ];

    for (style, height, radius) in expected {
        let recipe = progress_recipe(style);
        assert_eq!(recipe.height_px, height);
        assert_eq!(recipe.default_radius, radius);
    }
}
