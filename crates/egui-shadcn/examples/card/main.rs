#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui::{Align2, FontData, FontDefinitions, FontFamily, FontId};
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::{
    Button, ButtonSize, ButtonVariant, CardProps, CardVariant, ControlSize, ControlVariant, Input,
    InputSize, InputType, Label, SelectItem, SelectProps, Theme, button, card, select_with_items,
    switch,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct CardDemo {
    theme: Theme,
    light_mode: bool,
    email: String,
    password: String,
    project_name: String,
    framework: Option<String>,
}

impl CardDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            light_mode: false,
            email: String::new(),
            password: String::new(),
            project_name: String::new(),
            framework: None,
        }
    }
}

impl App for CardDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 16.0;

            ui.horizontal_wrapped(|row| {
                row.spacing_mut().item_spacing = egui::Vec2::new(16.0, 16.0);

                row.vertical(|tile| {
                    tile.set_min_width(344.0);
                    tile.set_max_width(344.0);
                    tile.set_min_height(346.0);
                    render_login_card(tile, &self.theme, &mut self.email, &mut self.password);
                });

                row.vertical(|tile| {
                    tile.set_min_width(344.0);
                    tile.set_max_width(344.0);
                    render_project_card(
                        tile,
                        &self.theme,
                        &mut self.project_name,
                        &mut self.framework,
                    );

                    tile.add_space(12.0);
                    let theme_changed =
                        render_theme_toggle(tile, &self.theme, &mut self.light_mode);
                    if theme_changed {
                        self.theme = Theme::new(if self.light_mode {
                            ColorPalette::light()
                        } else {
                            ColorPalette::dark()
                        });
                        tile.ctx().request_repaint();
                    }
                });
            });
        });
    }
}

fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded_card");
    let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
    if already_set {
        return;
    }

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn render_login_card(ui: &mut egui::Ui, theme: &Theme, email: &mut String, password: &mut String) {
    card(
        ui,
        theme,
        CardProps::default()
            .with_padding(egui::vec2(24.0, 24.0))
            .with_variant(CardVariant::Outline)
            .with_shadow(false),
        |card_ui| {
            card_ui.spacing_mut().item_spacing.y = 20.0;

            card_ui.horizontal(|header| {
                header.vertical(|left| {
                    left.spacing_mut().item_spacing.y = 8.0;
                    left.label(
                        egui::RichText::new("Login to your account")
                            .text_style(egui::TextStyle::Button)
                            .size(14.0)
                            .strong()
                            .color(theme.palette.foreground),
                    );
                    left.label(
                        egui::RichText::new("Enter your email below to login to\nyour account")
                            .color(theme.palette.muted_foreground),
                    );
                });

                header.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |right| {
                    let _ = button(
                        right,
                        theme,
                        "Sign Up",
                        ControlVariant::Link,
                        ControlSize::Sm,
                        true,
                    );
                });
            });

            card_ui.vertical(|content| {
                content.spacing_mut().item_spacing.y = 12.0;

                content.vertical(|field| {
                    field.spacing_mut().item_spacing.y = 8.0;
                    let email_id = field.make_persistent_id("card-email");
                    Label::new("Email").size(ControlSize::Sm).show(field, theme);
                    Input::new(email_id)
                        .input_type(InputType::Email)
                        .placeholder("m@example.com")
                        .size(InputSize::Size2)
                        .width(field.available_width())
                        .show(field, theme, email);
                });

                content.add_space(8.0);
                content.vertical(|field| {
                    field.spacing_mut().item_spacing.y = 8.0;
                    let password_id = field.make_persistent_id("card-password");

                    let available_width = field.available_width();
                    field.horizontal(|row| {
                        row.set_min_width(available_width);
                        Label::new("Password")
                            .size(ControlSize::Sm)
                            .show(row, theme);

                        row.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |right| {
                                let link = egui::RichText::new("Forgot your password?")
                                    .font(ControlSize::Sm.font())
                                    .color(theme.palette.primary);
                                let response =
                                    right.add(egui::Label::new(link).sense(egui::Sense::click()));
                                if response.hovered() {
                                    let y = response.rect.bottom() - 1.0;
                                    right.painter().line_segment(
                                        [
                                            egui::pos2(response.rect.left(), y),
                                            egui::pos2(response.rect.right(), y),
                                        ],
                                        egui::Stroke::new(1.0, theme.palette.primary),
                                    );
                                }
                            },
                        );
                    });

                    Input::new(password_id)
                        .input_type(InputType::Password)
                        .size(InputSize::Size2)
                        .width(field.available_width())
                        .show(field, theme, password);
                });
            });

            card_ui.add_space(4.0);

            card_ui.vertical(|footer| {
                footer.spacing_mut().item_spacing.y = 8.0;
                let full_width = footer.available_width();
                footer.set_min_width(full_width);

                let _ = Button::new("Login")
                    .variant(ButtonVariant::Default)
                    .size(ButtonSize::Default)
                    .min_width(full_width)
                    .show(footer, theme);
                let _ = Button::new("Login with GitHub")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Default)
                    .icon(&github_icon)
                    .min_width(full_width)
                    .show(footer, theme);
            });
        },
    );
}

fn github_icon(painter: &egui::Painter, center: egui::Pos2, size: f32, color: egui::Color32) {
    painter.text(
        center,
        Align2::CENTER_CENTER,
        Icon::Github.unicode(),
        FontId::proportional(size),
        color,
    );
}

fn render_project_card(
    ui: &mut egui::Ui,
    theme: &Theme,
    project_name: &mut String,
    framework: &mut Option<String>,
) {
    card(
        ui,
        theme,
        CardProps::default()
            .with_padding(egui::vec2(24.0, 24.0))
            .with_variant(CardVariant::Outline)
            .with_shadow(false),
        |card_ui| {
            card_ui.spacing_mut().item_spacing.y = 20.0;

            card_ui.vertical(|header| {
                header.spacing_mut().item_spacing.y = 8.0;
                header.label(
                    egui::RichText::new("Create project")
                        .text_style(egui::TextStyle::Button)
                        .size(14.0)
                        .strong()
                        .color(theme.palette.foreground),
                );
                header.label(
                    egui::RichText::new("Deploy your new project in one-click.")
                        .color(theme.palette.muted_foreground),
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
                        .show(field, theme);
                    Input::new(name_id)
                        .placeholder("Name of your project")
                        .size(InputSize::Size2)
                        .width(field.available_width())
                        .show(field, theme, project_name);
                });

                content.vertical(|field| {
                    field.spacing_mut().item_spacing.y = 8.0;
                    let framework_id = field.make_persistent_id("framework-select");
                    Label::new("Framework")
                        .for_id(framework_id)
                        .size(ControlSize::Sm)
                        .show(field, theme);

                    let items = vec![
                        SelectItem::option("next", "Next.js"),
                        SelectItem::option("sveltekit", "SvelteKit"),
                        SelectItem::option("astro", "Astro"),
                        SelectItem::option("nuxt", "Nuxt.js"),
                    ];

                    select_with_items(
                        field,
                        theme,
                        SelectProps::new(framework_id, framework)
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
                    theme,
                    "Cancel",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                );
                footer.with_layout(egui::Layout::right_to_left(egui::Align::Center), |right| {
                    let _ = button(
                        right,
                        theme,
                        "Deploy",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });
            });
        },
    );
}

fn render_theme_toggle(ui: &mut egui::Ui, theme: &Theme, light_mode: &mut bool) -> bool {
    let previous = *light_mode;

    ui.horizontal(|row| {
        row.set_width(row.available_width());
        row.spacing_mut().item_spacing.x = 8.0;

        row.horizontal(|left| {
            left.spacing_mut().item_spacing.x = 6.0;
            let (icon, label) = if *light_mode {
                (Icon::Sun, "Light theme")
            } else {
                (Icon::Moon, "Dark theme")
            };

            left.label(
                egui::RichText::new(icon.unicode())
                    .text_style(egui::TextStyle::Body)
                    .size(16.0)
                    .strong(),
            );
            left.label(
                egui::RichText::new(label)
                    .color(theme.palette.muted_foreground)
                    .size(13.0),
            );
        });

        row.with_layout(egui::Layout::right_to_left(egui::Align::Center), |right| {
            let _ = switch(
                right,
                theme,
                light_mode,
                "",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            );
        });
    });

    previous != *light_mode
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(720.0, 376.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Card example",
        options,
        Box::new(|_cc| Ok(Box::new(CardDemo::new()))),
    )
}
