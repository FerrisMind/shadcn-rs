//! Configuration enums for the sidebar component.

pub use shadcn_common::{
    SidebarCollapsible, SidebarController, SidebarDisplayState, SidebarSide, SidebarVariant,
};

/// Visual treatment of [`super::SidebarMenuButton`] (`variant` prop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SidebarMenuButtonVariant {
    /// Transparent resting fill; accent on hover / active.
    #[default]
    Default,
    /// Hairline border using `sidebar-border`.
    Outline,
}

impl SidebarMenuButtonVariant {
    /// Every supported variant.
    pub const ALL: [Self; 2] = [Self::Default, Self::Outline];
}

/// Size of [`super::SidebarMenuButton`] (`size` prop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SidebarMenuButtonSize {
    /// `h-8 text-sm`.
    #[default]
    Default,
    /// `h-7 text-xs`.
    Sm,
    /// `h-12 text-sm`.
    Lg,
}

impl SidebarMenuButtonSize {
    /// Every supported size.
    pub const ALL: [Self; 3] = [Self::Default, Self::Sm, Self::Lg];
}

/// Size of [`super::SidebarMenuSubButton`] (`size` prop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SidebarMenuSubButtonSize {
    /// `text-sm`.
    #[default]
    Md,
    /// `text-xs`.
    Sm,
}

impl SidebarMenuSubButtonSize {
    /// Every supported size.
    pub const ALL: [Self; 2] = [Self::Md, Self::Sm];
}
