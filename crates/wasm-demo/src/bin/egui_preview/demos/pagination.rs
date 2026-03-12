use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ButtonSize, PaginationLinkProps, PaginationProps, pagination, pagination_content,
    pagination_link, pagination_next, pagination_previous,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let page_id = ui.make_persistent_id("preview-pagination-page");
    let mut current_page = ui
        .data_mut(|d| d.get_persisted::<usize>(page_id).unwrap_or(2))
        .clamp(1, 5);
    ui.horizontal_centered(|row| {
        pagination(
            row,
            PaginationProps::new(5, &mut current_page),
            |pui, props| {
                pagination_content(pui, |content| {
                    let _ = pagination_previous(content, &app.theme, props);
                    let _ = pagination_link(
                        content,
                        &app.theme,
                        props,
                        PaginationLinkProps::new(1, "1").size(ButtonSize::Icon),
                    );
                    let _ = pagination_link(
                        content,
                        &app.theme,
                        props,
                        PaginationLinkProps::new(2, "2").size(ButtonSize::Icon),
                    );
                    let _ = pagination_link(
                        content,
                        &app.theme,
                        props,
                        PaginationLinkProps::new(3, "3").size(ButtonSize::Icon),
                    );
                    let _ = pagination_next(content, &app.theme, props);
                });
            },
        );
    });
    ui.data_mut(|d| d.insert_persisted(page_id, current_page));
}
