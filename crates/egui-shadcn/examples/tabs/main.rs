//! Пример Tabs, повторяющий демо shadcn/ui с табами Account/Password и формой в Card.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{
    CardProps, CardVariant, ControlSize, ControlVariant, InputProps, InputSize, InputType,
    LabelProps, TabItem, TabsProps, TabsVariant, Theme, button, card, label_with_props, tabs,
    text_input_with_props,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.set_max_width(400.0);
                ui.spacing_mut().item_spacing.y = 24.0;

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
                        "password" => {
                            render_password_tab(
                                content_ui,
                                &self.theme,
                                &mut self.current_password,
                                &mut self.new_password,
                            );
                        }
                        _ => {
                            render_account_tab(
                                content_ui,
                                &self.theme,
                                &mut self.name,
                                &mut self.username,
                            );
                        }
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
        CardProps::default().with_variant(CardVariant::Surface),
        |card_ui| {
            card_ui.vertical(|content_ui| {
                content_ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 16.0);

                render_card_header(
                    content_ui,
                    theme,
                    "Account",
                    "Make changes to your account here. Click save when you're done.",
                );

                render_card_content(content_ui, |fields_ui| {
                    render_labeled_input(
                        fields_ui,
                        theme,
                        "tabs-demo-name",
                        "Name",
                        name,
                        InputType::Text,
                    );
                    render_labeled_input(
                        fields_ui,
                        theme,
                        "tabs-demo-username",
                        "Username",
                        username,
                        InputType::Text,
                    );
                });

                render_card_footer(content_ui, theme, "Save changes");
            });
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
        CardProps::default().with_variant(CardVariant::Surface),
        |card_ui| {
            card_ui.vertical(|content_ui| {
                content_ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 16.0);

                render_card_header(
                    content_ui,
                    theme,
                    "Password",
                    "Change your password here. After saving, you'll be logged out.",
                );

                render_card_content(content_ui, |fields_ui| {
                    render_labeled_input(
                        fields_ui,
                        theme,
                        "tabs-demo-current-password",
                        "Current password",
                        current_password,
                        InputType::Password,
                    );
                    render_labeled_input(
                        fields_ui,
                        theme,
                        "tabs-demo-new-password",
                        "New password",
                        new_password,
                        InputType::Password,
                    );
                });

                render_card_footer(content_ui, theme, "Save password");
            });
        },
    );
}

fn render_card_header(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    ui.vertical(|header_ui| {
        header_ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 4.0);
        header_ui.label(
            egui::RichText::new(title)
                .text_style(egui::TextStyle::Button)
                .size(18.0)
                .strong(),
        );
        header_ui.label(egui::RichText::new(description).color(theme.palette.muted_foreground));
    });
}

fn render_card_content(ui: &mut egui::Ui, render_fields: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|content_ui| {
        content_ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 24.0);
        content_ui.vertical(|fields_ui| {
            fields_ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 12.0);
            render_fields(fields_ui);
        });
    });
}

fn render_card_footer(ui: &mut egui::Ui, theme: &Theme, label: &str) {
    button(
        ui,
        theme,
        label,
        ControlVariant::Primary,
        ControlSize::Md,
        true,
    );
}

fn render_labeled_input(
    ui: &mut egui::Ui,
    theme: &Theme,
    id_suffix: &str,
    label_text: &str,
    value: &mut String,
    input_type: InputType,
) {
    ui.vertical(|field_ui| {
        field_ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 4.0);
        let input_id = field_ui.make_persistent_id(id_suffix);
        label_with_props(
            field_ui,
            theme,
            LabelProps::new(label_text).for_id(input_id),
        );
        text_input_with_props(
            field_ui,
            theme,
            InputProps::new(input_id, value)
                .input_type(input_type)
                .size(InputSize::Size3)
                .width(field_ui.available_width()),
        );
    });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Tabs example",
        options,
        Box::new(|_cc| Ok(Box::new(TabsDemo::new()))),
    )
}
