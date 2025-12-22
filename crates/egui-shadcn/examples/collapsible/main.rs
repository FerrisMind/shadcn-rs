#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/logging.rs"]
mod logging;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{Color32, CornerRadius, FontId, Margin, RichText, Stroke, Vec2, vec2};
use egui_shadcn::{
    Button, ButtonSize, ButtonVariant, CollapsibleProps, Theme, collapsible, icon_chevrons_up_down,
};

fn repo_item(ui: &mut egui::Ui, theme: &Theme, text: &str) {
    let rounding = CornerRadius::same(theme.radius.r2.round() as u8);
    let frame = egui::Frame::NONE
        .stroke(Stroke::new(1.0, theme.palette.border))
        .corner_radius(rounding)
        .inner_margin(Margin::symmetric(16, 8));

    frame.show(ui, |ui| {
        ui.label(RichText::new(text).font(FontId::monospace(14.0)));
    });
}

struct CollapsibleDemo {
    theme: Theme,
    open: bool,
    debug_clicks: u64,
    debug_trigger_rect: Option<egui::Rect>,
}

impl CollapsibleDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            open: false,
            debug_clicks: 0,
            debug_trigger_rect: None,
        }
    }
}

impl App for CollapsibleDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        let theme = self.theme.clone();
        let debug_input = std::env::var_os("SHADCN_DEBUG_INPUT").is_some();

        if debug_input {
            let (primary_clicked, primary_down, hover_pos, interact_pos, events) = ctx.input(|i| {
                (
                    i.pointer.primary_clicked(),
                    i.pointer.primary_down(),
                    i.pointer.hover_pos(),
                    i.pointer.interact_pos(),
                    i.events.clone(),
                )
            });

            if primary_down || primary_clicked {
                log::info!(
                    "input state: primary_down={primary_down} primary_clicked={primary_clicked} hover_pos={hover_pos:?} interact_pos={interact_pos:?} pixels_per_point={}",
                    ctx.pixels_per_point()
                );
                for e in events.iter().filter(|e| {
                    matches!(
                        e,
                        egui::Event::PointerMoved(_)
                            | egui::Event::PointerButton { .. }
                            | egui::Event::Touch { .. }
                            | egui::Event::MouseWheel { .. }
                    )
                }) {
                    log::info!("input event: {e:?}");
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_main_align(egui::Align::Min),
                |ui| {
                    ui.add_space(80.0);

                    if debug_input {
                        let (hover_pos, interact_pos, primary_down, primary_clicked) =
                            ctx.input(|i| {
                                (
                                    i.pointer.hover_pos(),
                                    i.pointer.interact_pos(),
                                    i.pointer.primary_down(),
                                    i.pointer.primary_clicked(),
                                )
                            });
                        ui.label(format!(
                            "debug_input: hover_pos={hover_pos:?} interact_pos={interact_pos:?} primary_down={primary_down} primary_clicked={primary_clicked}"
                        ));
                        ui.label(format!(
                            "open={} pixels_per_point={} content_rect={:?}",
                            self.open,
                            ctx.pixels_per_point(),
                            ctx.content_rect()
                        ));
                        if ui
                            .button(format!("debug egui::button clicks={}", self.debug_clicks))
                            .clicked()
                        {
                            self.debug_clicks += 1;
                            log::info!("debug egui button clicked => {}", self.debug_clicks);
                        }
                        ui.add_space(12.0);
                    }

                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::Center)
                            .with_main_align(egui::Align::Min),
                        |ui| {
                            ui.set_width(350.0);
                            ui.spacing_mut().item_spacing = vec2(8.0, 8.0);

                            let mut trigger_rect: Option<egui::Rect> = None;

                            let id = ui.make_persistent_id("collapsible-demo");
                            collapsible(
                                ui,
                                &theme,
                                CollapsibleProps::new(id, &mut self.open)
                                    .with_animation(true)
                                    .animation_ms(300.0),
                                |ui, api| {
                                    egui::Frame::NONE
                                        .inner_margin(Margin::symmetric(16, 0))
                                        .show(ui, |ui| {
                                            ui.with_layout(
                                                egui::Layout::left_to_right(egui::Align::Center)
                                                    .with_main_justify(true),
                                                |ui| {
                                                    ui.spacing_mut().item_spacing.x = 16.0;
                                                    ui.label(
                                                        RichText::new(
                                                            "@peduarte starred 3 repositories",
                                                        )
                                                        .size(14.0)
                                                        .strong(),
                                                    );

                                                    let button = Button::new("")
                                                        .variant(ButtonVariant::Ghost)
                                                        .size(ButtonSize::IconSm)
                                                        .icon(&icon_chevrons_up_down);
                                                    let response = api
                                                        .trigger(ui, |ui| button.show(ui, &theme));
                                                    trigger_rect = Some(response.rect);
                                                },
                                            );
                                        });

                                    repo_item(ui, &theme, "@radix-ui/primitives");

                                    let _ = api.content(ui, |ui| {
                                        ui.with_layout(
                                            egui::Layout::top_down(egui::Align::Min)
                                                .with_main_align(egui::Align::Min),
                                            |ui| {
                                                ui.spacing_mut().item_spacing = Vec2::splat(8.0);
                                                repo_item(ui, &theme, "@radix-ui/colors");
                                                repo_item(ui, &theme, "@stitches/react");
                                            },
                                        );
                                    });
                                },
                            );

                            if debug_input {
                                self.debug_trigger_rect = trigger_rect;
                                if let Some(rect) = self.debug_trigger_rect {
                                    ui.label(format!("trigger_rect={rect:?}"));
                                    ui.add_space(8.0);

                                    ui.painter().rect_stroke(
                                        rect,
                                        0.0,
                                        Stroke::new(2.0, Color32::RED),
                                        egui::StrokeKind::Outside,
                                    );
                                }

                                if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                                    ui.painter().circle_stroke(
                                        pos,
                                        6.0,
                                        Stroke::new(2.0, Color32::LIGHT_GREEN),
                                    );
                                }
                            }
                        },
                    );
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let log_path = logging::init_file_logger("collapsible")
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<failed to open log file>".to_string());
    let options = icon::native_options();
    eframe::run_native(
        "Collapsible example",
        options,
        Box::new(move |_cc| {
            log::info!("logging to {log_path}");
            Ok(Box::new(CollapsibleDemo::new()))
        }),
    )
}
