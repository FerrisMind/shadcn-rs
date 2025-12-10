# Input example (egui-shadcn)

This demo showcases the `Input` component with parity to shadcn/ui and radix-ui themes:
- Basic types: text, email, password (with eye icon), search.
- Variants: Surface, Classic, Soft.
- Sizes: Size1 (24px), Size2 (32px), Size3 (40px).
- States: invalid ring, disabled, read-only.
- Slots: leading/trailing icons (lucide), dual slots.
- Extras: max length counter, custom accent color, radius presets (none/sm/md/lg/full).

## Run
```bash
cargo run --example input
```
Use `--release` for smoother repainting if needed:
```bash
cargo run --example input --release
```

## Notes
- Icons come from `lucide_icons` and are embedded; no extra assets required.
- The demo applies a dark background and injects the lucide font at startup.

