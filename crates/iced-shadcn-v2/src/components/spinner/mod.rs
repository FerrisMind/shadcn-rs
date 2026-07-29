//! Canvas-based loading indicators.
//!
//! The public builder and configuration types live in `types`. Canvas
//! drawing and frame scheduling are isolated in `render`, while behavioral
//! checks are kept in `tests`.

mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{Spinner, SpinnerSize, SpinnerState, SpinnerVariant, spinner};
