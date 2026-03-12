use super::super::app::EguiPreviewApp;
use eframe::egui::{self, ImageSource, RichText, Ui};
use egui_shadcn::{AvatarProps, AvatarShape, AvatarSize, avatar};

const AVATAR_ICON: ImageSource<'static> =
    egui::include_image!("../../../../../egui-shadcn/assets/icons/shadcn-egui/icon2.svg");

fn avatar_tile(ui: &mut Ui, label: &str, add_avatar: impl FnOnce(&mut Ui)) {
    ui.vertical_centered(|col| {
        col.set_min_width(92.0);
        col.set_max_width(92.0);
        col.label(RichText::new(label).size(12.0));
        col.add_space(8.0);
        add_avatar(col);
    });
}

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let tile_width = 92.0;
    if compact {
        let row_gap = 14.0;
        let block_width = tile_width * 2.0 + row_gap;
        let _ = ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |host| {
                host.horizontal(|line| {
                    let left_padding =
                        ((line.available_width() - block_width) * 0.5 - 14.0).max(0.0);
                    line.add_space(left_padding);
                    line.vertical(|center| {
                        center.set_min_width(block_width);
                        center.set_max_width(block_width);
                        center.spacing_mut().item_spacing = egui::vec2(14.0, 12.0);
                        center.horizontal(|row| {
                            row.set_min_width(block_width);
                            row.set_max_width(block_width);
                            row.spacing_mut().item_spacing.x = row_gap;
                            avatar_tile(row, "Basic", |ui| {
                                avatar(
                                    ui,
                                    &app.theme,
                                    AvatarProps::new("EA")
                                        .size(AvatarSize::Size6)
                                        .image(AVATAR_ICON),
                                );
                            });
                            avatar_tile(row, "Rounded", |ui| {
                                avatar(
                                    ui,
                                    &app.theme,
                                    AvatarProps::new("EA")
                                        .size(AvatarSize::Size6)
                                        .shape(AvatarShape::Rounded)
                                        .image(AVATAR_ICON),
                                );
                            });
                        });
                        center.horizontal(|row| {
                            row.set_min_width(block_width);
                            row.set_max_width(block_width);
                            row.spacing_mut().item_spacing.x = row_gap;
                            avatar_tile(row, "Error", |ui| {
                                avatar(
                                    ui,
                                    &app.theme,
                                    AvatarProps::new("JK")
                                        .size(AvatarSize::Size6)
                                        .color(app.theme.palette.destructive),
                                );
                            });
                            avatar_tile(row, "Large", |ui| {
                                avatar(
                                    ui,
                                    &app.theme,
                                    AvatarProps::new("DX")
                                        .size(AvatarSize::Size7)
                                        .image(AVATAR_ICON),
                                );
                            });
                        });
                    });
                });
            },
        );
        return;
    }

    let row_gap = 20.0;
    let block_width = tile_width * 4.0 + row_gap * 3.0;
    let _ = ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 0.0),
        egui::Layout::top_down(egui::Align::Center),
        |center| {
            center.set_min_width(block_width);
            center.set_max_width(block_width);
            center.horizontal(|row| {
                row.set_min_width(block_width);
                row.set_max_width(block_width);
                row.spacing_mut().item_spacing.x = row_gap;
                avatar_tile(row, "Basic", |ui| {
                    avatar(
                        ui,
                        &app.theme,
                        AvatarProps::new("EA")
                            .size(AvatarSize::Size6)
                            .image(AVATAR_ICON),
                    );
                });
                avatar_tile(row, "Rounded", |ui| {
                    avatar(
                        ui,
                        &app.theme,
                        AvatarProps::new("EA")
                            .size(AvatarSize::Size6)
                            .shape(AvatarShape::Rounded)
                            .image(AVATAR_ICON),
                    );
                });
                avatar_tile(row, "Error", |ui| {
                    avatar(
                        ui,
                        &app.theme,
                        AvatarProps::new("JK")
                            .size(AvatarSize::Size6)
                            .color(app.theme.palette.destructive),
                    );
                });
                avatar_tile(row, "Large", |ui| {
                    avatar(
                        ui,
                        &app.theme,
                        AvatarProps::new("DX")
                            .size(AvatarSize::Size8)
                            .image(AVATAR_ICON),
                    );
                });
            });
        },
    );
}
