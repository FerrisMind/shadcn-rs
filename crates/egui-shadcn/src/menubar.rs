//! Menubar component - horizontal application menu bar.
//!
//! # Example
//! ```ignore
//! menubar(ui, &theme, MenubarProps::new(), |ui| {
//!     menubar_menu(ui, &theme, "File", |ui| { /* items */ });
//! });
//! ```

use crate::theme::Theme;
use egui::{Frame, Margin, Response, Sense, Ui};

/// A single top-level menubar menu trigger.
#[derive(Clone, Debug)]
pub struct MenubarMenuProps<'a> {
    pub label: &'a str,
    pub disabled: bool,
}

impl<'a> MenubarMenuProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Properties for the Menubar container.
#[derive(Clone, Copy, Debug, Default)]
pub struct MenubarProps {
    pub disabled: bool,
}

impl MenubarProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Render a menubar container.
pub fn menubar<R>(
    ui: &mut Ui,
    theme: &Theme,
    _props: MenubarProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    Frame::default()
        .fill(theme.palette.background)
        .stroke(egui::Stroke::new(1.0_f32, theme.palette.border))
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(Margin::symmetric(4, 2))
        .show(ui, |bar_ui| {
            bar_ui.horizontal(|row_ui| add_contents(row_ui)).inner
        })
        .inner
}

/// Render a single menubar menu trigger button.
pub fn menubar_menu<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: MenubarMenuProps<'_>,
    id_source: impl std::hash::Hash,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> (Response, Option<R>) {
    let id = ui.make_persistent_id(id_source);
    let open = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));

    let text_color = if props.disabled {
        theme.palette.muted_foreground
    } else {
        theme.palette.foreground
    };

    let label_galley = ui.painter().layout_no_wrap(
        props.label.to_string(),
        egui::FontId::proportional(14.0),
        text_color,
    );
    let label_size = label_galley.size() + egui::vec2(16.0, 8.0);

    let (rect, trigger_resp) = ui.allocate_exact_size(label_size, Sense::click());

    let bg = if open || trigger_resp.hovered() {
        theme.palette.accent
    } else {
        egui::Color32::TRANSPARENT
    };

    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(4), bg);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        props.label,
        egui::FontId::proportional(14.0),
        text_color,
    );

    if trigger_resp.clicked() && !props.disabled {
        ui.data_mut(|d| d.insert_temp(id, !open));
    }

    let popup_result = if open {
        let popup_id = id.with("popup");
        let popup_pos = rect.left_bottom();
        let mut result = None;

        egui::Area::new(popup_id)
            .order(egui::Order::Foreground)
            .fixed_pos(popup_pos)
            .show(ui.ctx(), |popup_ui| {
                Frame::popup(popup_ui.style())
                    .fill(theme.palette.background)
                    .stroke(egui::Stroke::new(1.0_f32, theme.palette.border))
                    .show(popup_ui, |inner_ui| {
                        inner_ui.set_min_width(160.0);
                        result = Some(add_contents(inner_ui));
                    });

                // Close on click outside
                let any_click = popup_ui.input(|i| i.pointer.any_click());
                let interact_pos = popup_ui.input(|i| i.pointer.interact_pos());
                let inside = interact_pos
                    .map(|p| popup_ui.clip_rect().contains(p))
                    .unwrap_or(false);
                if any_click && !inside {
                    ui.data_mut(|d| d.insert_temp(id, false));
                }
            });

        result
    } else {
        None
    };

    (trigger_resp, popup_result)
}

/// Render a menubar menu item.
pub fn menubar_item(ui: &mut Ui, theme: &Theme, label: &str, disabled: bool) -> Response {
    let text_color = if disabled {
        theme.palette.muted_foreground
    } else {
        theme.palette.foreground
    };

    let size_galley = ui.painter().layout_no_wrap(
        label.to_string(),
        egui::FontId::proportional(14.0),
        text_color,
    );
    let size = size_galley.size() + egui::vec2(16.0, 8.0);

    let (rect, resp) = ui.allocate_exact_size(
        size,
        if disabled {
            Sense::hover()
        } else {
            Sense::click()
        },
    );

    let bg = if resp.hovered() && !disabled {
        theme.palette.accent
    } else {
        egui::Color32::TRANSPARENT
    };

    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(4), bg);
    ui.painter().text(
        rect.left_center() + egui::vec2(8.0, 0.0),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(14.0),
        text_color,
    );

    resp
}

/// Render a menubar separator.
pub fn menubar_separator(ui: &mut Ui, theme: &Theme) {
    let available = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(available, 1.0), Sense::hover());
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(0), theme.palette.border);
    ui.add_space(2.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menubar_props_default() {
        let props = MenubarProps::new();
        assert!(!props.disabled);
    }

    #[test]
    fn menubar_menu_props_builder() {
        let props = MenubarMenuProps::new("File").disabled(true);
        assert_eq!(props.label, "File");
        assert!(props.disabled);
    }
}
