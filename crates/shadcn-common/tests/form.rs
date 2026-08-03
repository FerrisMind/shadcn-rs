use shadcn_common::{
    FieldConstraints, FieldValue, FormState, ValidationMode, email, max_length, min_length,
    pattern, required,
};

#[test]
fn form_state_tracks_values_and_validation_lifecycle() {
    let mut form = FormState::new(ValidationMode::OnSubmit);
    form.field_with_constraints(
        "username",
        required("Username is required"),
        FieldConstraints::new().required(true),
    );

    assert_eq!(
        form.value("username"),
        Some(&FieldValue::Text(String::new()))
    );
    assert!(!form.is_submitting());
    assert!(!form.validate());
    assert!(form.submit_attempted());
    assert_eq!(form.error("username"), Some("Username is required"));

    assert!(form.set_text("username", "alice"));
    assert!(form.is_valid());
    assert_eq!(
        form.value("username"),
        Some(&FieldValue::Text("alice".to_owned()))
    );
}

#[test]
fn validation_modes_cover_change_blur_and_touched() {
    let mut on_change = FormState::new(ValidationMode::OnChange);
    on_change.field_with_constraints("name", required("Required"), FieldConstraints::default());
    on_change.set_text("name", "initial");
    on_change.set_text("name", "");
    assert_eq!(on_change.error("name"), Some("Required"));

    let mut on_blur = FormState::new(ValidationMode::OnBlur);
    on_blur.field_with_constraints("name", required("Required"), FieldConstraints::default());
    on_blur.set_text("name", "");
    assert!(on_blur.error("name").is_none());
    on_blur.blur("name");
    assert_eq!(on_blur.error("name"), Some("Required"));

    let mut on_touched = FormState::new(ValidationMode::OnTouched);
    on_touched.field_with_constraints("name", required("Required"), FieldConstraints::default());
    on_touched.set_text("name", "");
    assert!(on_touched.error("name").is_none());
    on_touched.blur("name");
    assert_eq!(on_touched.error("name"), Some("Required"));
}

#[test]
fn field_constraints_and_builtin_validators_are_composable() {
    let constraints = FieldConstraints::new()
        .required(true)
        .min_length(3)
        .max_length(12)
        .pattern("^[a-z]+$");
    assert!(constraints.is_required());
    assert_eq!(constraints.min_length_value(), Some(3));
    assert_eq!(constraints.max_length_value(), Some(12));
    assert_eq!(constraints.pattern_value(), Some("^[a-z]+$"));

    let mut form = FormState::new(ValidationMode::OnChange);
    form.field_with_constraints(
        "email",
        shadcn_common::compose(vec![
            required("Email is required"),
            email("Enter a valid email"),
        ]),
        FieldConstraints::new().required(true),
    );
    form.field_with_constraints(
        "handle",
        shadcn_common::compose(vec![
            min_length(3, "Too short"),
            max_length(8, "Too long"),
            pattern("^[a-z]+$", "Lowercase letters only"),
        ]),
        FieldConstraints::default(),
    );

    form.set_text("email", "valid@example.com");
    form.set_text("email", "invalid");
    form.set_text("handle", "abc");
    form.set_text("handle", "A");
    assert_eq!(form.error("email"), Some("Enter a valid email"));
    assert_eq!(form.error("handle"), Some("Too short"));
}

#[test]
fn field_state_exposes_stable_ids_for_accessible_composition() {
    let mut form = FormState::default();
    form.field_with_constraints(
        "profile.username",
        required("Required"),
        FieldConstraints::new().required(true),
    );

    let field = form
        .field_state("profile.username")
        .expect("registered field");
    let ids = field.ids();
    assert_eq!(ids.control(), "profile-username");
    assert_eq!(ids.label(), "profile-username-label");
    assert_eq!(ids.description(), "profile-username-description");
    assert_eq!(ids.errors(), "profile-username-errors");
}
