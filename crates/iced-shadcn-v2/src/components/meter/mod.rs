//! Theme-aware meter indicators.
//!
//! Port of shadcn-svelte-extras [`Meter`](https://www.shadcn-svelte-extras.com/docs/components/meter)
//! on top of bits-ui `value` / `min` / `max`. Geometry and fill-tone math live in
//! [`shadcn_common::meter`] / [`shadcn_common::meter_recipe`] so egui can reuse them.
//!
//! Extras has no per-pack Meter style table (same pattern as Form). Choosing
//! Rhea (or Nova, …) on the shared [`crate::theme::Theme`] styles Meter by
//! resolving that pack's palette / accents — `theme.palette.primary`,
//! `theme.palette.destructive`, Button chrome in demos — all via
//! `theme.style_id()`. Pass the same `&Theme` into every meter instance.

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{Meter, MeterOrientation, MeterRadius, MeterSize, MeterState, meter};
