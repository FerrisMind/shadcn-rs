use super::*;

#[test]
fn field_defaults_match_the_vertical_shadcn_layout() {
    let theme = Theme::light();
    let field = Field::<()>::new(&theme);

    assert_eq!(field.orientation, FieldOrientation::Vertical);
    assert_eq!(field.width, Length::Fill);
    assert_eq!(
        field.responsive_breakpoint,
        geometry::DEFAULT_RESPONSIVE_BREAKPOINT
    );
    assert_eq!(field.spacing, None);
    assert!(!field.invalid);
    assert!(!field.disabled);
}

#[test]
fn field_normalizes_invalid_spacing_and_breakpoints() {
    let theme = Theme::light();
    let field = Field::<()>::new(&theme)
        .spacing(f32::NAN)
        .responsive_breakpoint(f32::INFINITY);

    assert_eq!(field.spacing, Some(0.0));
    assert_eq!(field.responsive_breakpoint, 0.0);
}

#[test]
fn field_group_and_content_defaults_are_fill_width() {
    let group = FieldGroup::<()>::new();
    let set = FieldSet::<()>::new();
    let content = FieldContent::<()>::new();

    assert_eq!(group.width, Length::Fill);
    assert_eq!(group.spacing, None);
    assert_eq!(set.width, Length::Fill);
    assert_eq!(set.spacing, None);
    assert_eq!(content.width, Length::Fill);
    assert_eq!(content.spacing, None);
}

#[test]
fn error_items_preserve_optional_messages() {
    let message = FieldErrorItem::new("Choose another username.");
    let empty = FieldErrorItem::empty();

    assert_eq!(message.message(), Some("Choose another username."));
    assert_eq!(empty.message(), None);
    assert_eq!(message.to_string(), "Choose another username.");
    assert_eq!(empty.to_string(), "<empty field error>");
}

#[test]
fn field_error_prefers_custom_content_over_error_items() {
    let theme = Theme::light();
    let error = FieldError::<()>::text("Custom error", &theme)
        .errors([FieldErrorItem::new("Ignored error")]);

    assert!(error.content.is_some());
    assert_eq!(error.errors.len(), 1);
}

#[test]
fn responsive_orientation_is_explicit() {
    assert!(FieldOrientation::Responsive.is_responsive());
    assert!(!FieldOrientation::Horizontal.is_responsive());
}
