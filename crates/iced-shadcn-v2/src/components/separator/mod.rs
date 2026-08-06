//! Thin rule that visually or semantically separates content.
//!
//! Port of the shadcn-svelte `Separator` (bits-ui `Separator.Root`). The
//! public builder and configuration types live in `types`; widget
//! construction is isolated in `render`, while behavioral checks are kept
//! in `tests`.

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use render::separator;
pub use types::{Separator, SeparatorOrientation};
