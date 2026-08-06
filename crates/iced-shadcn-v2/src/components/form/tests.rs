use super::*;
use shadcn_common::{FieldConstraints, FormState, ValidationMode, form_recipe, required};

#[test]
fn control_props_follow_shared_field_state() {
    let mut form = FormState::new(ValidationMode::OnSubmit);
    form.field_with_constraints(
        "username",
        required("Username is required"),
        FieldConstraints::new().required(true),
    );
    assert!(!form.validate());

    let field = form
        .field_state("username")
        .expect("registered field should be available");
    let props = FormControlProps::from_field(field);

    assert_eq!(props.control_id(), Some("username"));
    assert_eq!(props.label_id(), Some("username-label"));
    assert_eq!(props.description_id(), Some("username-description"));
    assert_eq!(props.errors_id(), Some("username-errors"));
    assert_eq!(
        props.described_by_ids(),
        &[
            "username-description".to_owned(),
            "username-errors".to_owned()
        ]
    );
    assert!(props.is_invalid());
    assert!(props.is_required());
    assert!(!props.is_disabled());
}

#[test]
fn form_field_can_resolve_state_by_name() {
    let mut form = FormState::new(ValidationMode::OnSubmit);
    form.field("email", required("Email is required"));

    let theme = Theme::light();
    let field = FormField::<()>::from_state("email", &form, &theme);

    assert!(format!("{field:?}").contains("email"));
}

#[test]
fn form_label_marks_invalid_state() {
    let theme = Theme::light();
    let label = FormLabel::<()>::text("Username", &theme).invalid(true);
    assert!(format!("{label:?}").contains("invalid: true"));
}

#[test]
fn form_recipe_matches_shadcn_svelte_spacing_and_type() {
    let recipe = form_recipe(shadcn_common::StyleId::Nova);
    assert!((recipe.form_gap_px - 24.0).abs() < f32::EPSILON);
    assert!((recipe.field_gap_px - 8.0).abs() < f32::EPSILON);
    assert!((recipe.fieldset_gap_px - 8.0).abs() < f32::EPSILON);
    assert!((recipe.description.size_px - 14.0).abs() < f32::EPSILON);
    assert!((recipe.description.line_height_px - 20.0).abs() < f32::EPSILON);
    assert!((recipe.error.line_height_px - 20.0).abs() < f32::EPSILON);
    assert!((recipe.legend.line_height_px - 14.0).abs() < f32::EPSILON);
}

#[test]
fn form_children_follow_theme_style_pack() {
    // Form.json is pack-invariant, but composed Label/Button recipes are not.
    // Selecting Rhea on the shared Theme must surface Rhea recipes to parts.
    let vega = Theme::light().with_style(shadcn_common::StyleId::Vega);
    let rhea = Theme::light().with_style(shadcn_common::StyleId::Rhea);
    let sera = Theme::light().with_style(shadcn_common::StyleId::Sera);

    assert_eq!(vega.style.form(), rhea.style.form());
    assert_eq!(
        vega.style.label(shadcn_common::LabelContext::Field),
        rhea.style.label(shadcn_common::LabelContext::Field)
    );
    // Sera's field label recipe differs — proves Theme.style_id drives parts.
    assert_ne!(
        vega.style.label(shadcn_common::LabelContext::Field),
        sera.style.label(shadcn_common::LabelContext::Field)
    );
    assert_ne!(vega.style.button_type(), rhea.style.button_type());

    let _label = FormLabel::<()>::text("Username", &rhea);
    let _button = FormButton::<()>::text("Submit", &rhea);
    assert_eq!(rhea.style_id(), shadcn_common::StyleId::Rhea);
}

#[test]
fn form_fieldset_uses_recipe_gap() {
    let theme = Theme::light();
    let set = FormFieldset::<()>::new(&theme);
    assert!(format!("{set:?}").contains("spacing"));
}
