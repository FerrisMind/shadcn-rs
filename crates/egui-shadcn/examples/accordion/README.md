# Accordion Example

Demonstrates the Accordion component with Single and Multiple modes, matching the shadcn-ui reference.

## Features

- **Single Mode** (default): Only one item can be open at a time
- **Collapsible Mode**: Allows closing all items
- **Smooth Animations**: Based on the Collapsible component (200ms default)
- **Radix-like API**: Follows Radix UI Themes accordion patterns

## Running

```bash
cargo run -p egui-shadcn --example accordion
```

## API Reference

### AccordionProps

- `accordion_type`: `AccordionType::Single` or `AccordionType::Multiple`
- `collapsible`: Whether the open item can be closed (Single mode only)
- `default_value`: Initially open item value
- `disabled`: Disable the entire accordion
- `animate`: Enable/disable animations (default: true)
- `animation_ms`: Custom animation duration

### AccordionItemProps

- `value`: Unique identifier for the item
- `disabled`: Disable this specific item

## Examples Shown

1. **Product Information** - Default accordion with Product Info, Shipping, Return Policy
2. **FAQ** - Collapsible accordion showing accessibility Q&A

## Reference

Based on:
- [shadcn-ui Accordion Demo](https://ui.shadcn.com/docs/components/accordion)
- [Radix UI Accordion](https://www.radix-ui.com/primitives/docs/components/accordion)
