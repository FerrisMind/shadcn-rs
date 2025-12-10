## Tooltip Component (egui-shadcn)

Complete implementation of the Tooltip component with 100% compatibility with Radix UI and shadcn/ui specifications.

### Features

#### Radix UI Compatibility
- ✅ Default delay duration: 700ms (configurable)
- ✅ Skip delay duration: 300ms (when tooltip was just shown)
- ✅ Max width: 360px default (configurable)
- ✅ Side offset: 4px default
- ✅ Collision padding: 10px default
- ✅ Side positioning: Top, Bottom, Left, Right
- ✅ Alignment: Center, Start, End
- ✅ Arrow support
- ✅ Animation states: Closed, DelayedOpen, InstantOpen
- ✅ Force mount for testing/debugging

#### API

```rust
use egui_shadcn::tooltip::{TooltipProps, TooltipSide, TooltipAlign};

// Basic tooltip with defaults (700ms delay, 360px max width, top position)
let _ = tooltip(
    &button_response,
    ui,
    &theme,
    TooltipProps::new("Tooltip content"),
);

// Custom delay and positioning
let _ = tooltip(
    &button_response,
    ui,
    &theme,
    TooltipProps::new("Custom delay")
        .delay_ms(200)
        .side(TooltipSide::Right)
        .align(TooltipAlign::Start),
);

// With arrow
let _ = tooltip(
    &button_response,
    ui,
    &theme,
    TooltipProps::new("With arrow").show_arrow(true),
);

// Advanced customization
let _ = tooltip(
    &button_response,
    ui,
    &theme,
    TooltipProps::new("Full featured")
        .delay_ms(300)
        .max_width(400.0)
        .side(TooltipSide::Bottom)
        .align(TooltipAlign::Center)
        .side_offset(5.0)
        .collision_padding(15.0)
        .show_arrow(true),
);
```

#### Props Builder

- `new(text)` - Create tooltip with text content
- `delay_ms(u64)` - Delay before showing (Radix default: 700)
- `skip_delay_ms(u64)` - Skip delay when recently shown (Radix default: 300)
- `max_width(f32)` - Maximum width in pixels
- `side(TooltipSide)` - Position relative to anchor: Top, Right, Bottom, Left
- `align(TooltipAlign)` - Alignment: Center, Start, End
- `side_offset(f32)` - Offset from anchor in pixels
- `collision_padding(f32)` - Padding from viewport edges
- `show_arrow(bool)` - Display arrow pointing to anchor
- `arrow_size(width, height)` - Custom arrow dimensions
- `force_mount(bool)` - Force render even when not visible
- `disable_hoverable_content(bool)` - Disable hoverable tooltip content
- `show_when_disabled(bool)` - Show tooltip on disabled elements
- `high_contrast(bool)` - Use high contrast styling
- `position(TooltipPosition)` - Legacy positioning (use side/align instead)
- `offset(Vec2)` - Legacy offset (use side_offset instead)
- `style(TooltipStyle)` - Custom styling
- `persistent_id(Id)` - Custom persistent ID

### Animation States

The tooltip automatically manages animation states:
- **Closed** - Not visible
- **DelayedOpen** - Visible after delay (default 700ms)
- **InstantOpen** - Shown immediately (delay_ms = 0)

### Styling

Tooltips automatically use the theme's color palette:
- Background: Mix of foreground/background (10% high contrast)
- Text: Background color
- Border: Mix of border/foreground (20% blend)
- Rounding: 6px (Radix UI radius-2)
- Default Shadow: Enabled

Custom styles can be applied via `TooltipStyle`:
```rust
use egui_shadcn::tooltip::TooltipStyle;

let style = TooltipStyle::from_palette(&theme.palette, false);
// ... customize fields ...
TooltipProps::new("Text").style(style)
```

### Legacy Support

The component maintains backward compatibility with the old `TooltipPosition` enum:
```rust
// Old API still works
TooltipProps::new("Text").position(TooltipPosition::Right)
```

### Run Example

```bash
cargo run --example tooltip
```

The example demonstrates:
- Basic tooltip with default settings
- Custom delay (200ms)
- Arrow support
- Different side positioning (Right, Top, etc.)
- Alignment options (Start, Center, End)
- Width constraints with text wrapping