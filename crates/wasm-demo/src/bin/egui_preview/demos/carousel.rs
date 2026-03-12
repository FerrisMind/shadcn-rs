use super::super::app::EguiPreviewApp;
use eframe::egui::{Align, Id, Layout, RichText, Ui, vec2};
use egui_shadcn::{
    CarouselContentProps, CarouselItemProps, CarouselOptions, CarouselOrientation, CarouselProps,
    carousel, carousel_content, carousel_item, carousel_next, carousel_previous,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let slides = if compact {
        ["A", "B", "C"]
    } else {
        ["Overview", "Templates", "Insights"]
    };

    let content_size = if compact {
        vec2(220.0, 120.0)
    } else {
        vec2(360.0, 160.0)
    };

    ui.with_layout(Layout::top_down(Align::Center), |center| {
        let _ = carousel(
            center,
            &app.theme,
            CarouselProps::new(Id::new("preview-carousel"))
                .orientation(CarouselOrientation::Horizontal)
                .opts(CarouselOptions::default()),
            |ui, ctx| {
                ui.with_layout(Layout::left_to_right(Align::Center), |row| {
                    carousel_previous(row, &app.theme, ctx);
                    let _ = carousel_content(
                        row,
                        &app.theme,
                        ctx,
                        CarouselContentProps::new().size(content_size),
                        |content_ui, ctx| {
                            for (idx, label) in slides.iter().enumerate() {
                                let _ = carousel_item(
                                    content_ui,
                                    ctx,
                                    CarouselItemProps::new(idx),
                                    |item_ui| {
                                        item_ui.centered_and_justified(|ui| {
                                            ui.label(RichText::new(*label).strong());
                                        });
                                    },
                                );
                            }
                        },
                    );
                    carousel_next(row, &app.theme, ctx);
                });
            },
        );
    });
}
