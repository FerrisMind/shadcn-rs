## Label Component Example (egui-shadcn)

A comprehensive demonstration of the `Label` component with all features, variants, sizes, states, and configurations.

### Features Demonstrated

✅ **Input Association** - Labels linked to input fields with automatic focus management
✅ **Semantic Variants** - Default, Secondary, Muted, and Destructive variants with appropriate colors
✅ **Size Options** - Small, Medium, and Large sizes with responsive font scaling
✅ **Description Text** - Helper/hint text with lower visual emphasis
✅ **Required Indicator** - Visual indicator for required fields (asterisk *)
✅ **Disabled State** - Disabled labels with reduced opacity
✅ **Hover Interactions** - Cursor changes to pointer on hover (for enabled labels)
✅ **Accessibility** - Proper labeling for form controls and accessibility compliance

### Component Capabilities

1. **Form Labeling** - Link labels to form inputs with automatic focus management
2. **Visual Hierarchy** - Use variants and sizes to establish content hierarchy
3. **User Guidance** - Description text provides context and validation hints
4. **State Management** - Required, disabled, and error states for form workflows
5. **Theme Integration** - Full theming support with semantic colors

### Run

```bash
cargo run --example label
```

Use `--release` for optimized rendering:
```bash
cargo run --example label --release
```

### Usage Examples

#### Basic Label
```rust
Label::new("Your name")
    .show(ui, &theme);
```

#### Label Linked to Input
```rust
let input_id = ui.make_persistent_id("email");
Label::new("Email")
    .for_id(input_id)
    .show(ui, &theme);
Input::new(input_id).show(ui, &theme, &mut email);
```

#### Label with Description
```rust
Label::new("Password")
    .required(true)
    .description("Use at least 8 characters with mixed case and numbers.")
    .show(ui, &theme);
```

#### Variant Examples
```rust
// Default (primary emphasis)
Label::new("Main label").show(ui, &theme);

// Secondary (medium emphasis)
Label::new("Secondary text")
    .variant(LabelVariant::Secondary)
    .show(ui, &theme);

// Muted (low emphasis)
Label::new("Helper text")
    .variant(LabelVariant::Muted)
    .show(ui, &theme);

// Destructive (error/warning)
Label::new("Required action")
    .variant(LabelVariant::Destructive)
    .show(ui, &theme);
```

#### Size Examples
```rust
Label::new("Small").size(ControlSize::Sm).show(ui, &theme);
Label::new("Medium").size(ControlSize::Md).show(ui, &theme); // default
Label::new("Large").size(ControlSize::Lg).show(ui, &theme);
```

### Implementation Details

The Label component provides:
- **Semantic color mapping** based on variants and theme palette
- **Proper typography** with size-aware font configuration
- **Disabled state styling** with opacity reduction
- **Focus management** for linked form controls
- **Description text support** with appropriate visual distinction
- **Complete accessibility** with proper semantic labeling

### API Reference

See the [Label documentation](../../src/label.rs) for detailed API reference and type definitions.
