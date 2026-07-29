//! Theme-aware loading placeholder.
//!
//! The public builder and configuration types live in `types`, shape
//! calculations in `geometry`, canvas drawing in `render`, and behavioral
//! checks in `tests`.

mod geometry;
mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{Skeleton, SkeletonAnimation, SkeletonFill, SkeletonRadius, SkeletonShape};
