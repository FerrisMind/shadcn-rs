//! Пример Card, содержащий `card-demo` и `card-with-form` из shadcn/ui.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{
    CardProps, ControlSize, ControlVariant, Input, InputSize, InputType, Label, SelectItem,
    SelectProps, Theme, button, card, select_with_items,
};

struct CardDemo {
    theme: Theme,
    email: String,
    password: String,
    project_name: String,
    framework: Option<String>,
}

impl CardDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            email: String::new(),
            password: String::new(),
            project_name: String::new(),
            framework: None,
        }
    }
}

impl App for CardDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.scope(|ui| {
                ui.set_max_width(384.0);
                ui.spacing_mut().item_spacing.y = 24.0;

                // card-demo (Login)
                card(ui, &self.theme, CardProps::default(), |card_ui| {
                    card_ui.spacing_mut().item_spacing.y = 16.0;

                    card_ui.horizontal(|header| {
                        header.vertical(|left| {
                            left.spacing_mut().item_spacing.y = 4.0;
                            left.label(
                                egui::RichText::new("Login to your account")
                                    .text_style(egui::TextStyle::Button)
                                    .size(18.0)
                                    .strong(),
                            );
                            left.label(
                                egui::RichText::new(
                                    "Enter your email below to login to your account",
                                )
                                .color(self.theme.palette.muted_foreground),
                            );
                        });

                        header.with_layout(
                            egui::Layout::right_to_left(egui::Align::TOP),
                            |right| {
                                let _ = button(
                                    right,
                                    &self.theme,
                                    "Sign Up",
                                    ControlVariant::Link,
                                    ControlSize::Sm,
                                    true,
                                );
                            },
                        );
                    });

                    card_ui.vertical(|content| {
                        content.spacing_mut().item_spacing.y = 24.0;

                        content.vertical(|field| {
                            field.spacing_mut().item_spacing.y = 8.0;
                            let email_id = field.make_persistent_id("card-email");
                            Label::new("Email")
                                .for_id(email_id)
                                .size(ControlSize::Sm)
                                .show(field, &self.theme);
                            Input::new(email_id)
                                .input_type(InputType::Email)
                                .placeholder("m@example.com")
                                .size(InputSize::Size2)
                                .width(field.available_width())
                                .show(field, &self.theme, &mut self.email);
                        });

                        content.vertical(|field| {
                            field.spacing_mut().item_spacing.y = 8.0;
                            let password_id = field.make_persistent_id("card-password");

                            field.horizontal(|row| {
                                Label::new("Password")
                                    .for_id(password_id)
                                    .size(ControlSize::Sm)
                                    .show(row, &self.theme);
                                row.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |right| {
                                        let _ = button(
                                            right,
                                            &self.theme,
                                            "Forgot your password?",
                                            ControlVariant::Link,
                                            ControlSize::Sm,
                                            true,
                                        );
                                    },
                                );
                            });

                            Input::new(password_id)
                                .input_type(InputType::Password)
                                .size(InputSize::Size2)
                                .width(field.available_width())
                                .show(field, &self.theme, &mut self.password);
                        });
                    });

                    card_ui.vertical(|footer| {
                        footer.spacing_mut().item_spacing.y = 8.0;
                        let _ = button(
                            footer,
                            &self.theme,
                            "Login",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        let _ = button(
                            footer,
                            &self.theme,
                            "Login with Google",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            true,
                        );
                    });
                });

                // card-with-form (Create project)
                card(ui, &self.theme, CardProps::default(), |card_ui| {
                    card_ui.spacing_mut().item_spacing.y = 16.0;

                    card_ui.vertical(|header| {
                        header.spacing_mut().item_spacing.y = 4.0;
                        header.label(
                            egui::RichText::new("Create project")
                                .text_style(egui::TextStyle::Button)
                                .size(18.0)
                                .strong(),
                        );
                        header.label(
                            egui::RichText::new("Deploy your new project in one-click.")
                                .color(self.theme.palette.muted_foreground),
                        );
                    });

                    card_ui.vertical(|content| {
                        content.spacing_mut().item_spacing.y = 24.0;

                        content.vertical(|field| {
                            field.spacing_mut().item_spacing.y = 8.0;
                            let name_id = field.make_persistent_id("project-name");
                            Label::new("Name")
                                .for_id(name_id)
                                .size(ControlSize::Sm)
                                .show(field, &self.theme);
                            Input::new(name_id)
                                .placeholder("Name of your project")
                                .size(InputSize::Size2)
                                .width(field.available_width())
                                .show(field, &self.theme, &mut self.project_name);
                        });

                        content.vertical(|field| {
                            field.spacing_mut().item_spacing.y = 8.0;
                            let framework_id = field.make_persistent_id("framework-select");
                            Label::new("Framework")
                                .for_id(framework_id)
                                .size(ControlSize::Sm)
                                .show(field, &self.theme);

                            let items = vec![
                                SelectItem::option("next", "Next.js"),
                                SelectItem::option("sveltekit", "SvelteKit"),
                                SelectItem::option("astro", "Astro"),
                                SelectItem::option("nuxt", "Nuxt.js"),
                            ];

                            select_with_items(
                                field,
                                &self.theme,
                                SelectProps::new(framework_id, &mut self.framework)
                                    .placeholder("Select")
                                    .width(field.available_width()),
                                &items,
                            );
                        });
                    });

                    card_ui.horizontal(|footer| {
                        footer.spacing_mut().item_spacing.x = 8.0;
                        let _ = button(
                            footer,
                            &self.theme,
                            "Cancel",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            true,
                        );
                        footer.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |right| {
                                let _ = button(
                                    right,
                                    &self.theme,
                                    "Deploy",
                                    ControlVariant::Primary,
                                    ControlSize::Md,
                                    true,
                                );
                            },
                        );
                    });
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Card example",
        options,
        Box::new(|_cc| Ok(Box::new(CardDemo::new()))),
    )
}
