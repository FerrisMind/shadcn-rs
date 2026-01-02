#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    ControlSize, ControlVariant, Toast, ToastPosition, ToastPromise, ToastVariant, Toaster, button,
};
use lucide_icons::LUCIDE_FONT_BYTES;

struct PendingPromise {
    started_at: f64,
    promise: ToastPromise,
}

struct ToastExample {
    theme: egui_shadcn::Theme,
    pending_promise: Option<PendingPromise>,
}

impl ToastExample {
    fn new() -> Self {
        Self {
            theme: egui_shadcn::Theme::default(),
            pending_promise: None,
        }
    }
}

impl App for ToastExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        ensure_lucide_font(ctx);

        CentralPanel::default().show(ctx, |ui| {
            let toaster = Toaster::get_or_init(ctx);
            toaster.set_position(ToastPosition::BottomRight);

            ui.heading("Toast / Sonner");
            ui.add_space(16.0);

            ui.vertical(|ui| {
                render_section_title(
                    ui,
                    &self.theme,
                    "Toast demo",
                    "A basic toast with title and description.",
                );
                if button(
                    ui,
                    &self.theme,
                    "Show toast",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                )
                .clicked()
                {
                    toaster.show(
                        Toast::new("Scheduled: Catch up")
                            .with_description("Friday, February 10, 2023 at 5:57 PM"),
                    );
                }
            });

            ui.add_space(24.0);

            ui.vertical(|ui| {
                render_section_title(
                    ui,
                    &self.theme,
                    "Toast types",
                    "All variants, plus a promise-style update.",
                );
                ui.horizontal_wrapped(|row| {
                    row.spacing_mut().item_spacing.x = 8.0;

                    if button(
                        row,
                        &self.theme,
                        "Default",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(Toast::new("Event has been created"));
                    }

                    if button(
                        row,
                        &self.theme,
                        "Success",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(
                            Toast::new("Event has been created")
                                .with_variant(ToastVariant::Success),
                        );
                    }

                    if button(
                        row,
                        &self.theme,
                        "Info",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(
                            Toast::new("Be at the area 10 minutes before the event time")
                                .with_variant(ToastVariant::Info),
                        );
                    }

                    if button(
                        row,
                        &self.theme,
                        "Warning",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(
                            Toast::new("Event start time cannot be earlier than 8am")
                                .with_variant(ToastVariant::Warning),
                        );
                    }

                    if button(
                        row,
                        &self.theme,
                        "Error",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(
                            Toast::new("Event has not been created")
                                .with_variant(ToastVariant::Error),
                        );
                    }

                    if button(
                        row,
                        &self.theme,
                        "Loading",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(
                            Toast::new("Loading...")
                                .with_variant(ToastVariant::Loading)
                                .duration_ms(1500),
                        );
                    }

                    if button(
                        row,
                        &self.theme,
                        "Promise",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        let now = ctx.input(|i| i.time);
                        let promise = toaster.promise(Toast::new("Loading..."));
                        self.pending_promise = Some(PendingPromise {
                            started_at: now,
                            promise,
                        });
                    }
                });
            });

            if let Some(pending) = &self.pending_promise {
                let now = ctx.input(|i| i.time);
                if now - pending.started_at > 2.0
                    && let Some(pending) = self.pending_promise.take()
                {
                    pending.promise.success(
                        &toaster,
                        Toast::new("Event has been created")
                            .with_variant(ToastVariant::Success)
                            .with_description("All changes were applied."),
                    );
                }
            }

            toaster.render(ui, &self.theme);
        });
    }
}

fn render_section_title(
    ui: &mut egui::Ui,
    theme: &egui_shadcn::Theme,
    title: &str,
    subtitle: &str,
) {
    ui.label(egui::RichText::new(title).size(14.0).strong());
    ui.label(
        egui::RichText::new(subtitle)
            .size(12.0)
            .color(theme.palette.muted_foreground),
    );
    ui.add_space(8.0);
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
        .entry(FontFamily::Name("lucide".into()))
        .or_default()
        .insert(0, "lucide".into());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Toast example",
        options,
        Box::new(|_cc| Ok(Box::new(ToastExample::new()))),
    )
}
