#[cfg(target_arch = "wasm32")]
use super::route;
use super::ui_component;
use super::ui_home;
use crate::egui_preview::{ensure_image_loaders, ensure_lucide_font};
use eframe::{App, Frame, egui};
use egui_shadcn::{ColorPalette, Theme};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Component(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ComponentTab {
    Demo,
    Code,
}

impl ComponentTab {
    #[cfg(target_arch = "wasm32")]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Demo => "demo",
            Self::Code => "code",
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "demo" => Some(Self::Demo),
            "code" => Some(Self::Code),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstallTab {
    Automatic,
    Manual,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct EguiPreviewApp {
    pub theme: Theme,
    pub theme_mode: ThemeMode,
    pub screen: Screen,
    pub tab: ComponentTab,
    pub install_tab: InstallTab,
    pub search: String,
    pub progress_value: f32,
    pub email: String,
    pub project_name: String,
    pub framework: Option<String>,
    pub checkbox_enabled: bool,
    pub switch_enabled: bool,
    pub accordion_value: Option<String>,
    pub tabs_value: String,
    pub alert_open: bool,
    pub select_value: Option<String>,
    pub radio_value: String,
    pub otp_value: String,
}

impl EguiPreviewApp {
    fn themed_palette(mode: ThemeMode) -> ColorPalette {
        match mode {
            ThemeMode::Dark => {
                let mut palette = ColorPalette::dark();
                // Match references/components/preview dark theme tokens.
                palette.background = egui::Color32::from_rgb(0x00, 0x00, 0x00); // --primary-color
                palette.card = egui::Color32::from_rgb(0x0A, 0x0A, 0x0A); // --primary-color-2
                palette.popover = palette.card;
                palette.muted = egui::Color32::from_rgb(0x0E, 0x0E, 0x0E); // --primary-color-1
                palette.accent = egui::Color32::from_rgb(0x14, 0x13, 0x13); // --primary-color-3
                palette.border = egui::Color32::from_rgb(0x23, 0x23, 0x23); // --primary-color-6
                palette.input = palette.border;
                palette.foreground = egui::Color32::from_rgb(0xD4, 0xD4, 0xD4); // --secondary-color-4
                palette.card_foreground = palette.foreground;
                palette.popover_foreground = palette.foreground;
                palette.muted_foreground = egui::Color32::from_rgb(0xA1, 0xA1, 0xA1); // --secondary-color-5
                palette
            }
            ThemeMode::Light => ColorPalette::light(),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme_mode = match self.theme_mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
        self.theme = Theme::new(Self::themed_palette(self.theme_mode));
    }

    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let (screen, tab) = route::read_initial_route();
        #[cfg(not(target_arch = "wasm32"))]
        let (screen, tab) = (Screen::Home, ComponentTab::Demo);

        let theme_mode = ThemeMode::Dark;

        Self {
            theme: Theme::new(Self::themed_palette(theme_mode)),
            theme_mode,
            screen,
            tab,
            install_tab: InstallTab::Automatic,
            search: String::new(),
            progress_value: 64.0,
            email: String::new(),
            project_name: String::new(),
            framework: None,
            checkbox_enabled: true,
            switch_enabled: true,
            accordion_value: None,
            tabs_value: "account".to_owned(),
            alert_open: false,
            select_value: None,
            radio_value: "starter".to_owned(),
            otp_value: String::new(),
        }
    }
}

impl App for EguiPreviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        #[cfg(target_arch = "wasm32")]
        route::sync_url(self.screen, self.tab);

        ensure_lucide_font(ctx);
        ensure_image_loaders(ctx);

        egui::TopBottomPanel::top("topbar")
            .frame(egui::Frame::default().fill(self.theme.palette.background))
            .show(ctx, |ui| {
                ui_home::render_topbar(self, ui);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme.palette.background))
            .show(ctx, |ui| match self.screen {
                Screen::Home => ui_home::render_home(self, ui),
                Screen::Component(index) => ui_component::render_component(self, ui, index),
            });
    }
}
