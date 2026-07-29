//! Layout wrapper that preserves a width-to-height ratio for child content.
//!
//! Port of the shadcn-svelte `AspectRatio` (bits-ui `AspectRatio.Root`). The
//! public builder and configuration types live in `types`; custom layout and
//! widget construction are isolated in `render`, while behavioral checks are
//! kept in `tests`.

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use render::aspect_ratio;
pub use types::{AspectRatio, MIN_ASPECT_RATIO};
