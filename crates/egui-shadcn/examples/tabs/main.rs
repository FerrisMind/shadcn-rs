#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui_shadcn::{
    CardProps, CardVariant, ControlSize, ControlVariant, Input, InputSize, InputType, Label,
    TabItem, TabsProps, TabsVariant, Theme, button, card, tabs,
};

struct TabsDemo {
    theme: Theme,
    active: String,

    name: String,
    username: String,
    current_password: String,
    new_password: String,
}

impl TabsDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            active: "account".to_string(),
            name: "Pedro Duarte".to_string(),
            username: "@peduarte".to_string(),
            current_password: String::new(),
            new_password: String::new(),
        }
    }
}

impl App for TabsDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.set_max_width(384.0);

                let items = [
                    TabItem::new("account", "Account"),
                    TabItem::new("password", "Password"),
                ];

                let _ = tabs(
                    ui,
                    &self.theme,
                    TabsProps::new(ui.make_persistent_id("tabs-demo"), &items, &mut self.active)
                        .with_variant(TabsVariant::Soft)
                        .scrollable(false),
                    |content_ui, active_tab| match active_tab.id.as_str() {
                        "password" => render_password_tab(
                            content_ui,
                            &self.theme,
                            &mut self.current_password,
                            &mut self.new_password,
                        ),
                        _ => render_account_tab(
                            content_ui,
                            &self.theme,
                            &mut self.name,
                            &mut self.username,
                        ),
                    },
                );
            });
        });
    }
}

fn render_account_tab(ui: &mut egui::Ui, theme: &Theme, name: &mut String, username: &mut String) {
    card(
        ui,
        theme,
        CardProps::default()
            .with_variant(CardVariant::Outline)
            .with_padding(egui::vec2(24.0, 24.0))
            .with_rounding(egui::CornerRadius::same(12)),
        |card_ui| {
            card_ui.spacing_mut().item_spacing.y = 16.0;

            card_ui.vertical(|header| {
                header.spacing_mut().item_spacing.y = 4.0;
                header.label(
                    egui::RichText::new("Account")
                        .text_style(egui::TextStyle::Button)
                        .size(18.0)
                        .strong(),
                );
                header.label(
                    egui::RichText::new(
                        "Make changes to your account here. Click save when you're done.",
                    )
                    .color(theme.palette.muted_foreground),
                );
            });

            card_ui.vertical(|content| {
                content.spacing_mut().item_spacing.y = 24.0;

                render_field(
                    content,
                    theme,
                    "tabs-demo-name",
                    "Name",
                    name,
                    InputType::Text,
                );
                render_field(
                    content,
                    theme,
                    "tabs-demo-username",
                    "Username",
                    username,
                    InputType::Text,
                );
            });

            let _ = button(
                card_ui,
                theme,
                "Save changes",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
        },
    );
}

fn render_password_tab(
    ui: &mut egui::Ui,
    theme: &Theme,
    current_password: &mut String,
    new_password: &mut String,
) {
    card(
        ui,
        theme,
        CardProps::default()
            .with_variant(CardVariant::Outline)
            .with_padding(egui::vec2(24.0, 24.0))
            .with_rounding(egui::CornerRadius::same(12)),
        |card_ui| {
            card_ui.spacing_mut().item_spacing.y = 16.0;

            card_ui.vertical(|header| {
                header.spacing_mut().item_spacing.y = 4.0;
                header.label(
                    egui::RichText::new("Password")
                        .text_style(egui::TextStyle::Button)
                        .size(18.0)
                        .strong(),
                );
                header.label(
                    egui::RichText::new(
                        "Change your password here. After saving, you'll be logged out.",
                    )
                    .color(theme.palette.muted_foreground),
                );
            });

            card_ui.vertical(|content| {
                content.spacing_mut().item_spacing.y = 24.0;

                render_field(
                    content,
                    theme,
                    "tabs-demo-current",
                    "Current password",
                    current_password,
                    InputType::Password,
                );
                render_field(
                    content,
                    theme,
                    "tabs-demo-new",
                    "New password",
                    new_password,
                    InputType::Password,
                );
            });

            let _ = button(
                card_ui,
                theme,
                "Save password",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
        },
    );
}

fn render_field(
    ui: &mut egui::Ui,
    theme: &Theme,
    id_suffix: &str,
    label_text: &str,
    value: &mut String,
    input_type: InputType,
) {
    ui.vertical(|field| {
        field.spacing_mut().item_spacing.y = 12.0;
        let input_id = field.make_persistent_id(id_suffix);

        Label::new(label_text)
            .for_id(input_id)
            .size(ControlSize::Sm)
            .show(field, theme);
        Input::new(input_id)
            .input_type(input_type)
            .size(InputSize::Size2)
            .width(field.available_width())
            .show(field, theme, value);
    });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Tabs example",
        options,
        Box::new(|_cc| Ok(Box::new(TabsDemo::new()))),
    )
}
