//! Theme-aware progress indicators.
//!
//! The builder and public configuration types live in `types`. Geometry and
//! semantic color resolution stay in focused private modules, while canvas
//! drawing and animation are isolated in `render`.

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    Progress, ProgressOrientation, ProgressRadius, ProgressSize, ProgressState, ProgressVariant,
    progress,
};
