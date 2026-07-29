//! Theme adapters for `iced-shadcn-v2`.
//!
//! Theme resolution is split by responsibility: `palette` owns iced color
//! conversion, `tokens` owns semantic theme configuration, and
//! `typography` owns font selections.

mod palette;
mod tokens;
mod typography;

#[cfg(test)]
mod tests;

pub use palette::Palette;
pub use tokens::Theme;
