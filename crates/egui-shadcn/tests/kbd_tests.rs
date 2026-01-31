//! Tests for the Kbd component in egui-shadcn

use egui_shadcn::KbdProps;

#[test]
fn kbd_props_new_has_correct_defaults() {
    let props = KbdProps::new("Ctrl");
    assert_eq!(props.text.text(), "Ctrl");
    assert_eq!(props.size, None);
}

#[test]
fn kbd_props_builder_pattern_works() {
    let props = KbdProps::new("Shift").size(16.0);
    assert_eq!(props.text.text(), "Shift");
    assert_eq!(props.size, Some(16.0));
}

#[test]
fn kbd_props_supports_chaining() {
    let props = KbdProps::new("Alt").size(14.0);

    assert_eq!(props.text.text(), "Alt");
    assert_eq!(props.size, Some(14.0));
}

#[test]
fn kbd_props_accepts_special_characters() {
    let props = KbdProps::new("⌘");
    assert_eq!(props.text.text(), "⌘");

    let props = KbdProps::new("⇧");
    assert_eq!(props.text.text(), "⇧");

    let props = KbdProps::new("⌥");
    assert_eq!(props.text.text(), "⌥");

    let props = KbdProps::new("⌃");
    assert_eq!(props.text.text(), "⌃");
}

#[test]
fn kbd_props_accepts_function_keys() {
    for i in 1..=12 {
        let props = KbdProps::new(format!("F{}", i));
        assert_eq!(props.text.text(), format!("F{}", i));
    }
}

#[test]
fn kbd_props_accepts_arrow_keys() {
    let arrows = vec!["↑", "↓", "←", "→"];
    for arrow in arrows {
        let props = KbdProps::new(arrow);
        assert_eq!(props.text.text(), arrow);
    }
}

#[test]
fn kbd_props_accepts_navigation_keys() {
    let keys = vec!["Home", "End", "PgUp", "PgDn", "Ins", "Del"];
    for key in keys {
        let props = KbdProps::new(key);
        assert_eq!(props.text.text(), key);
    }
}

#[test]
fn kbd_props_accepts_common_shortcuts() {
    let shortcuts = vec![
        ("Ctrl", "C"),
        ("Ctrl", "V"),
        ("Ctrl", "X"),
        ("Ctrl", "Z"),
        ("Ctrl", "S"),
        ("Ctrl", "K"),
    ];

    for (modifier, key) in shortcuts {
        let modifier_props = KbdProps::new(modifier);
        let key_props = KbdProps::new(key);

        assert_eq!(modifier_props.text.text(), modifier);
        assert_eq!(key_props.text.text(), key);
    }
}

#[test]
fn kbd_props_size_variants() {
    let sizes = vec![10.0, 11.0, 12.0, 13.0, 14.0, 16.0, 18.0, 20.0, 24.0];

    for size in sizes {
        let props = KbdProps::new("Test").size(size);
        assert_eq!(props.size, Some(size));
    }
}
