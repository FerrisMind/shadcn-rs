#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::scroll_area::ScrollSource;
use egui::{Align, CentralPanel, CornerRadius, Layout, RichText, Stroke, StrokeKind, vec2};
use egui_shadcn::{
    CarouselContentProps, CarouselItemProps, CarouselOptions, CarouselOrientation, CarouselProps,
    ControlSize, ControlVariant, Theme, button, carousel, carousel_content, carousel_item,
    carousel_next, carousel_previous,
};

struct CarouselExample {
    theme: Theme,
    api_index: usize,
}

impl CarouselExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            api_index: 0,
        }
    }
}

impl App for CarouselExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .scroll_source(ScrollSource {
                    drag: false,
                    ..ScrollSource::ALL
                })
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.heading("Carousel");
                    ui.label("Slider with navigation controls and swipe support.");
                    ui.add_space(16.0);

                    ui.spacing_mut().item_spacing.y = 24.0;

                    render_section(
                        ui,
                        &self.theme,
                        "Carousel demo",
                        "Basic carousel with navigation buttons.",
                    );
                    render_carousel_demo(ui, &self.theme);

                    render_section(
                        ui,
                        &self.theme,
                        "Carousel orientation",
                        "Vertical carousel layout.",
                    );
                    render_carousel_orientation(ui, &self.theme);

                    render_section(
                        ui,
                        &self.theme,
                        "Carousel size",
                        "Partial slides with item basis.",
                    );
                    render_carousel_size(ui, &self.theme);

                    render_section(
                        ui,
                        &self.theme,
                        "Carousel spacing",
                        "Wider spacing between slides.",
                    );
                    render_carousel_spacing(ui, &self.theme);

                    render_section(
                        ui,
                        &self.theme,
                        "Carousel API",
                        "Controlled index with button selection.",
                    );
                    render_carousel_api(ui, &self.theme, &mut self.api_index);

                    render_section(
                        ui,
                        &self.theme,
                        "Carousel plugin",
                        "Autoplay with looping enabled.",
                    );
                    render_carousel_autoplay(ui, &self.theme);
                });
        });
    }
}

fn render_section(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.label(
            RichText::new(description)
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(8.0);
    });
}

fn render_carousel_demo(ui: &mut egui::Ui, theme: &Theme) {
    let slides = ["Dashboard", "Templates", "Insights", "Settings"];
    let _ = render_carousel(
        ui,
        theme,
        "carousel-demo",
        CarouselOrientation::Horizontal,
        CarouselOptions::default(),
        CarouselContentProps::new().size(vec2(360.0, 160.0)),
        &slides,
    );
}

fn render_carousel_orientation(ui: &mut egui::Ui, theme: &Theme) {
    let slides = ["Top", "Middle", "Bottom"];
    let _ = render_carousel(
        ui,
        theme,
        "carousel-vertical",
        CarouselOrientation::Vertical,
        CarouselOptions::default(),
        CarouselContentProps::new().size(vec2(240.0, 220.0)),
        &slides,
    );
}

fn render_carousel_size(ui: &mut egui::Ui, theme: &Theme) {
    let slides = ["Alpha", "Beta", "Gamma", "Delta"];
    let _ = render_carousel(
        ui,
        theme,
        "carousel-size",
        CarouselOrientation::Horizontal,
        CarouselOptions::default(),
        CarouselContentProps::new()
            .size(vec2(360.0, 160.0))
            .item_basis(0.7),
        &slides,
    );
}

fn render_carousel_spacing(ui: &mut egui::Ui, theme: &Theme) {
    let slides = ["Primary", "Secondary", "Tertiary"];
    let _ = render_carousel(
        ui,
        theme,
        "carousel-spacing",
        CarouselOrientation::Horizontal,
        CarouselOptions::default(),
        CarouselContentProps::new()
            .size(vec2(360.0, 160.0))
            .spacing(32.0),
        &slides,
    );
}

fn render_carousel_api(ui: &mut egui::Ui, theme: &Theme, index: &mut usize) {
    let slides = ["Overview", "Revenue", "Orders", "Insights"];
    let button_labels = ["1", "2", "3", "4"];
    let max_index = slides.len().saturating_sub(1);
    *index = (*index).min(max_index);

    let response = carousel(
        ui,
        theme,
        CarouselProps::new(egui::Id::new("carousel-api")),
        |ui, ctx| {
            ctx.current_index = *index;
            ui.with_layout(Layout::left_to_right(Align::Center), |row| {
                row.spacing_mut().item_spacing.x = 12.0;
                carousel_previous(row, theme, ctx);
                let _ = carousel_content(
                    row,
                    theme,
                    ctx,
                    CarouselContentProps::new().size(vec2(360.0, 160.0)),
                    |content_ui, ctx| render_slides(content_ui, theme, ctx, &slides),
                );
                carousel_next(row, theme, ctx);
            });
        },
    );

    *index = response.index;

    ui.add_space(8.0);
    ui.horizontal(|row| {
        row.spacing_mut().item_spacing.x = 8.0;
        row.label(
            RichText::new(format!(
                "Slide {} of {}",
                response.index + 1,
                response.count.max(1)
            ))
            .size(12.0)
            .color(theme.palette.muted_foreground),
        );

        for (idx, label) in button_labels.iter().enumerate() {
            let variant = if idx == *index {
                ControlVariant::Primary
            } else {
                ControlVariant::Secondary
            };
            if button(row, theme, *label, variant, ControlSize::Sm, true).clicked() {
                *index = idx;
            }
        }
    });
}

fn render_carousel_autoplay(ui: &mut egui::Ui, theme: &Theme) {
    let slides = ["Play", "Pause", "Repeat", "Shuffle"];
    let opts = CarouselOptions::default()
        .autoplay(true)
        .looped(true)
        .autoplay_delay_ms(1800.0);
    let _ = render_carousel(
        ui,
        theme,
        "carousel-autoplay",
        CarouselOrientation::Horizontal,
        opts,
        CarouselContentProps::new().size(vec2(360.0, 160.0)),
        &slides,
    );
}

fn render_carousel(
    ui: &mut egui::Ui,
    theme: &Theme,
    id: &str,
    orientation: CarouselOrientation,
    opts: CarouselOptions,
    content_props: CarouselContentProps,
    labels: &[&str],
) -> egui_shadcn::CarouselResponse<()> {
    carousel(
        ui,
        theme,
        CarouselProps::new(egui::Id::new(id))
            .orientation(orientation)
            .opts(opts),
        |ui, ctx| match ctx.orientation {
            CarouselOrientation::Horizontal => {
                ui.with_layout(Layout::left_to_right(Align::Center), |row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    carousel_previous(row, theme, ctx);
                    let _ = carousel_content(row, theme, ctx, content_props, |content_ui, ctx| {
                        render_slides(content_ui, theme, ctx, labels)
                    });
                    carousel_next(row, theme, ctx);
                });
            }
            CarouselOrientation::Vertical => {
                ui.with_layout(Layout::top_down(Align::Center), |col| {
                    col.spacing_mut().item_spacing.y = 12.0;
                    carousel_previous(col, theme, ctx);
                    let _ = carousel_content(col, theme, ctx, content_props, |content_ui, ctx| {
                        render_slides(content_ui, theme, ctx, labels)
                    });
                    carousel_next(col, theme, ctx);
                });
            }
        },
    )
}

fn render_slides(
    ui: &mut egui::Ui,
    theme: &Theme,
    ctx: &mut egui_shadcn::CarouselContext,
    labels: &[&str],
) {
    for (index, label) in labels.iter().enumerate() {
        let _ = carousel_item(ui, ctx, CarouselItemProps::new(index), |item_ui| {
            render_slide(item_ui, theme, index, label);
        });
    }
}

fn render_slide(ui: &mut egui::Ui, theme: &Theme, index: usize, label: &str) {
    let rect = ui.available_rect_before_wrap();
    let rounding = CornerRadius::from(theme.radius.r4);
    ui.painter().rect_filled(rect, rounding, theme.palette.card);
    ui.painter().rect_stroke(
        rect,
        rounding,
        Stroke::new(1.0, theme.palette.border),
        StrokeKind::Inside,
    );

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(label)
                        .size(20.0)
                        .strong()
                        .color(theme.palette.card_foreground),
                );
                ui.label(
                    RichText::new(format!("Slide {}", index + 1))
                        .size(12.0)
                        .color(theme.palette.muted_foreground),
                );
            });
        });
    });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Carousel example",
        options,
        Box::new(|_cc| Ok(Box::new(CarouselExample::new()))),
    )
}
