//! Item component - generic list/grid item.
//!
//! # Example
//! ```ignore
//! item(ui, &theme, ItemProps::new("Item title").description("Description"));
//! ```

use crate::theme::Theme;
use egui::{Color32, Sense, Ui};

/// A generic list/grid item component.
#[derive(Clone, Debug)]
pub struct ItemProps<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub disabled: bool,
    pub selected: bool,
}

impl<'a> ItemProps<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            disabled: false,
            selected: false,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// Render a generic list item.
pub fn item(ui: &mut Ui, theme: &Theme, props: ItemProps<'_>) -> egui::Response {
    let text_color = if props.disabled {
        theme.palette.muted_foreground
    } else {
        theme.palette.foreground
    };

    let desired_size = egui::vec2(
        ui.available_width(),
        if props.description.is_some() {
            52.0
        } else {
            36.0
        },
    );

    let (rect, resp) = ui.allocate_exact_size(
        desired_size,
        if props.disabled {
            Sense::hover()
        } else {
            Sense::click()
        },
    );

    let bg = if props.selected {
        theme.palette.accent
    } else if resp.hovered() && !props.disabled {
        Color32::from_rgba_unmultiplied(
            theme.palette.accent.r(),
            theme.palette.accent.g(),
            theme.palette.accent.b(),
            128,
        )
    } else {
        Color32::TRANSPARENT
    };

    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(6), bg);

    let text_rect = rect.shrink2(egui::vec2(12.0, 8.0));
    let title_pos = text_rect.left_top();

    ui.painter().text(
        title_pos,
        egui::Align2::LEFT_TOP,
        props.title,
        egui::FontId::proportional(14.0),
        text_color,
    );

    if let Some(desc) = props.description {
        let desc_pos = title_pos + egui::vec2(0.0, 20.0);
        ui.painter().text(
            desc_pos,
            egui::Align2::LEFT_TOP,
            desc,
            egui::FontId::proportional(12.0),
            theme.palette.muted_foreground,
        );
    }

    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_props_builder() {
        let props = ItemProps::new("Item title")
            .description("Item description")
            .disabled(false)
            .selected(true);

        assert_eq!(props.title, "Item title");
        assert_eq!(props.description, Some("Item description"));
        assert!(!props.disabled);
        assert!(props.selected);
    }
}
