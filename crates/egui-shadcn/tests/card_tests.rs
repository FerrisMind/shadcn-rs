use egui::Sense;
use egui_shadcn::{CardProps, CardSize, CardVariant};

#[test]
fn card_default_matches_radix_api() {
    let props = CardProps::default();

    assert_eq!(props.size, CardSize::Size3, "size should default to 3");
    assert_eq!(
        props.variant,
        CardVariant::Surface,
        "variant should default to surface"
    );
    assert!(
        !props.as_child,
        "as_child must be opt-in to match Radix `asChild` semantics"
    );
}

#[test]
fn card_as_child_enables_interaction() {
    let props = CardProps::default().with_as_child(true);

    assert!(props.as_child, "as_child flag should be stored");
    assert!(
        props.interactive,
        "as_child should make the card interactive for focus/hover states"
    );
    assert_eq!(
        props.sense,
        Sense::click(),
        "as_child should adopt click sense to mirror link/button usage"
    );
}

#[test]
fn card_supports_radix_variants_and_sizes() {
    for variant in [
        CardVariant::Surface,
        CardVariant::Classic,
        CardVariant::Ghost,
    ] {
        let props = CardProps::default().with_variant(variant);
        assert_eq!(props.variant, variant, "variant should round-trip");
    }

    for size in [
        CardSize::Size1,
        CardSize::Size2,
        CardSize::Size3,
        CardSize::Size4,
        CardSize::Size5,
    ] {
        let props = CardProps::default().with_size(size);
        assert_eq!(props.size, size, "size should round-trip");
    }
}
