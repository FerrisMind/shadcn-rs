//! Пример Button, содержащий все референсы shadcn/ui для Button.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui::{
    FontData, FontDefinitions, FontFamily, FontId, RichText, text::LayoutJob, text::TextFormat,
};
use egui_shadcn::{
    Button, ButtonRadius, ButtonSize, ButtonVariant, ControlSize, ControlVariant, SeparatorProps,
    Theme, button, separator,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct ButtonDemo {
    theme: Theme,
}

impl ButtonDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded");
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

fn lucide_icon(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string()).font(FontId::new(size, FontFamily::Proportional))
}

fn icon_with_text(icon: Icon, icon_size: f32, text: &str) -> egui::WidgetText {
    let mut job = LayoutJob::default();
    job.append(
        &icon.unicode().to_string(),
        0.0,
        TextFormat {
            font_id: FontId::new(icon_size, FontFamily::Proportional),
            ..Default::default()
        },
    );
    job.append(
        &format!(" {}", text),
        0.0,
        TextFormat {
            font_id: FontId::new(14.0, FontFamily::Proportional),
            ..Default::default()
        },
    );
    job.into()
}

impl App for ButtonDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;

                // button-default
                let _ = button(
                    ui,
                    &self.theme,
                    "Button",
                    ControlVariant::Primary,
                    ControlSize::Md,
                    true,
                );

                // button-outline
                let _ = button(
                    ui,
                    &self.theme,
                    "Outline",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                );

                // button-secondary
                let _ = button(
                    ui,
                    &self.theme,
                    "Secondary",
                    ControlVariant::Secondary,
                    ControlSize::Md,
                    true,
                );

                // button-ghost
                let _ = button(
                    ui,
                    &self.theme,
                    "Ghost",
                    ControlVariant::Ghost,
                    ControlSize::Md,
                    true,
                );

                // button-destructive
                let _ = button(
                    ui,
                    &self.theme,
                    "Destructive",
                    ControlVariant::Destructive,
                    ControlSize::Md,
                    true,
                );

                // button-link
                let _ = button(
                    ui,
                    &self.theme,
                    "Link",
                    ControlVariant::Link,
                    ControlSize::Md,
                    true,
                );

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                // button-demo (outline + icon)
                ui.horizontal_wrapped(|row| {
                    row.spacing_mut().item_spacing = egui::Vec2::new(8.0, 8.0);
                    let _ = button(
                        row,
                        &self.theme,
                        "Button",
                        ControlVariant::Outline,
                        ControlSize::Md,
                        true,
                    );
                    let _ = button(
                        row,
                        &self.theme,
                        lucide_icon(Icon::ArrowUp, 16.0),
                        ControlVariant::Outline,
                        ControlSize::Icon,
                        true,
                    )
                    .on_hover_text("Submit");
                });

                ui.add_space(8.0);

                // button-size
                ui.horizontal_wrapped(|col| {
                    col.spacing_mut().item_spacing = egui::Vec2::new(24.0, 12.0);

                    col.vertical(|pair| {
                        pair.spacing_mut().item_spacing = egui::Vec2::new(8.0, 8.0);
                        let _ = button(
                            pair,
                            &self.theme,
                            "Small",
                            ControlVariant::Outline,
                            ControlSize::Sm,
                            true,
                        );
                        let _ = button(
                            pair,
                            &self.theme,
                            lucide_icon(Icon::ArrowUpRight, 16.0),
                            ControlVariant::Outline,
                            ControlSize::IconSm,
                            true,
                        )
                        .on_hover_text("Submit");
                    });

                    col.vertical(|pair| {
                        pair.spacing_mut().item_spacing = egui::Vec2::new(8.0, 8.0);
                        let _ = button(
                            pair,
                            &self.theme,
                            "Default",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            true,
                        );
                        let _ = button(
                            pair,
                            &self.theme,
                            lucide_icon(Icon::ArrowUpRight, 16.0),
                            ControlVariant::Outline,
                            ControlSize::Icon,
                            true,
                        )
                        .on_hover_text("Submit");
                    });

                    col.vertical(|pair| {
                        pair.spacing_mut().item_spacing = egui::Vec2::new(8.0, 8.0);
                        let _ = button(
                            pair,
                            &self.theme,
                            "Large",
                            ControlVariant::Outline,
                            ControlSize::Lg,
                            true,
                        );
                        let _ = button(
                            pair,
                            &self.theme,
                            lucide_icon(Icon::ArrowUpRight, 16.0),
                            ControlVariant::Outline,
                            ControlSize::IconLg,
                            true,
                        )
                        .on_hover_text("Submit");
                    });
                });

                ui.add_space(8.0);

                // button-icon
                let _ = button(
                    ui,
                    &self.theme,
                    lucide_icon(Icon::CircleFadingArrowUp, 16.0),
                    ControlVariant::Outline,
                    ControlSize::Icon,
                    true,
                );

                // button-with-icon
                let _ = Button::new(icon_with_text(Icon::GitBranch, 16.0, "New Branch"))
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .show(ui, &self.theme);

                // button-rounded
                let _ = Button::new(lucide_icon(Icon::ArrowUp, 16.0))
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Icon)
                    .radius(ButtonRadius::Full)
                    .show(ui, &self.theme);

                // button-loading
                let _ = Button::new("Submit")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .loading(true)
                    .enabled(false)
                    .show(ui, &self.theme);

                // button-as-child (в egui нет Link, демонстрируем саму кнопку)
                let _ = Button::new("Login")
                    .variant(ButtonVariant::Default)
                    .size(ButtonSize::Default)
                    .show(ui, &self.theme);
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Button example",
        options,
        Box::new(|_cc| Ok(Box::new(ButtonDemo::new()))),
    )
}
