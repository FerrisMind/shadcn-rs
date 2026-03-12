use super::super::app::EguiPreviewApp;
use eframe::egui::{Color32, RichText, Sense, Ui, UiBuilder, vec2};
use egui_shadcn::{
    ResizableDirection, ResizableHandleProps, ResizablePanelGroupProps, ResizablePanelProps,
    resizable_handle, resizable_panel, resizable_panel_group,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let sizes_id = ui.make_persistent_id("preview-resizable-sizes");
    let mut sizes = ui
        .data(|d| d.get_temp::<Vec<f32>>(sizes_id))
        .unwrap_or_else(|| vec![50.0, 50.0]);

    let demo_size = if compact {
        vec2(260.0, 110.0)
    } else {
        vec2(360.0, 160.0)
    };

    ui.horizontal(|row| {
        row.add_space(((row.available_width() - demo_size.x) * 0.5).max(0.0));
        let (rect, _) = row.allocate_exact_size(demo_size, Sense::hover());
        row.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
            ui.set_clip_rect(rect);
            ui.set_min_size(demo_size);
            ui.set_max_size(demo_size);
            resizable_panel_group(
                ui,
                &app.theme,
                ResizablePanelGroupProps::new("preview-resizable").direction(if compact {
                    ResizableDirection::Horizontal
                } else {
                    ResizableDirection::Vertical
                }),
                &mut sizes,
                |ui, ctx| {
                    resizable_panel(ui, ctx, ResizablePanelProps::new(50.0), 0, |ui| {
                        panel_content(ui, "Panel A", Color32::from_rgb(59, 130, 246));
                    });
                    resizable_handle(
                        ui,
                        &app.theme,
                        ctx,
                        ResizableHandleProps::new().handle(true),
                        0,
                    );
                    resizable_panel(ui, ctx, ResizablePanelProps::new(50.0), 1, |ui| {
                        panel_content(ui, "Panel B", Color32::from_rgb(34, 197, 94));
                    });
                },
            );
        });
    });

    ui.data_mut(|d| d.insert_temp(sizes_id, sizes));
}

fn panel_content(ui: &mut Ui, label: &str, color: Color32) {
    let rect = ui.available_rect_before_wrap();
    ui.painter()
        .rect_filled(rect, 4.0, color.gamma_multiply(0.2));
    ui.centered_and_justified(|ui| {
        ui.label(RichText::new(label).color(color).strong());
    });
}
