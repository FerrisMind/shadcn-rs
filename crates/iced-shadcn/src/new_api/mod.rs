//! Experimental builder-first APIs.
//!
//! `new_api` is an opt-in namespace for crate APIs that are still evolving.
//! Components here keep semantics in `iced-shadcn` and may use `twill` as the
//! internal styling substrate.

pub mod button;

pub use button::{Button, ButtonRadius, ButtonSize, ButtonVariant};
