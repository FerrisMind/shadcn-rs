#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewPage {
    Home,
    Button,
    Badge,
    Progress,
    Stepper,
    Input,
}

impl PreviewPage {
    pub const ALL: [Self; 6] = [
        Self::Home,
        Self::Button,
        Self::Badge,
        Self::Progress,
        Self::Stepper,
        Self::Input,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Button => "Button",
            Self::Badge => "Badge",
            Self::Progress => "Progress",
            Self::Stepper => "Stepper",
            Self::Input => "Input",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Home => "The shadcn-rs design system landing page.",
            Self::Button => "Variants, sizes and states.",
            Self::Badge => "Status labels, variants and colors.",
            Self::Progress => "Determinate and loading indicators.",
            Self::Stepper => "Wizard-style step navigation and keyboard control.",
            Self::Input => "Form fields and helper text.",
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::Home => include_str!("home.rs"),
            Self::Button => include_str!("../../../../iced-shadcn/examples/button/main.rs"),
            Self::Badge => include_str!("../../../../iced-shadcn/examples/badge/main.rs"),
            Self::Progress => include_str!("../../../../iced-shadcn/examples/progress/main.rs"),
            Self::Stepper => include_str!("../../../../iced-shadcn/examples/stepper/main.rs"),
            Self::Input => include_str!("../../../../iced-shadcn/examples/input/main.rs"),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Button => "button",
            Self::Badge => "badge",
            Self::Progress => "progress",
            Self::Stepper => "stepper",
            Self::Input => "input",
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "home" => Some(Self::Home),
            "button" => Some(Self::Button),
            "badge" => Some(Self::Badge),
            "progress" => Some(Self::Progress),
            "stepper" => Some(Self::Stepper),
            "input" => Some(Self::Input),
            _ => None,
        }
    }
}
