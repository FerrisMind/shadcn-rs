//! Internal mirror of the `iced` facade paths used by this crate.
//!
//! The library depends on the granular `iced_core` / `iced_widget` crates
//! instead of the full `iced` facade (which unconditionally pulls in
//! `winit`, `iced_program`, and the async runtime stack). This module
//! re-exports the exact subset of the facade surface the components use,
//! under the same relative paths, so the rest of the crate reads as if it
//! imported from `iced` itself. All types are identical to the ones the
//! facade re-exports, so the public API stays compatible with apps that
//! use `iced` directly.

pub(crate) use iced_core::{
    Background, Border, Color, ContentFit, Event, Font, Length, Padding, Pixels, Point, Radians,
    Rectangle, Shadow, Size, Theme, Transformation, Vector,
};

pub(crate) use iced_core::{alignment, border, font, gradient, mouse, time, touch, window};

pub(crate) use iced_widget::Renderer;

/// Same alias the `iced` facade exposes: defaults to the built-in theme and
/// the multi-backend renderer.
pub(crate) type Element<'a, Message, Theme = iced_core::Theme, Renderer = iced_widget::Renderer> =
    iced_core::Element<'a, Message, Theme, Renderer>;

/// Mirror of `iced::advanced` (which re-exports `iced_core` internals).
pub(crate) mod advanced {
    pub(crate) use iced_core::layout;
    pub(crate) use iced_core::widget;
    pub(crate) use iced_core::{Clipboard, Shell, Widget, overlay, renderer};
}

/// Mirror of `iced::widget` (the facade re-exports `iced_widget` wholesale).
pub(crate) mod widget {
    pub(crate) use iced_widget::*;
}
