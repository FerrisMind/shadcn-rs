use super::super::app::EguiPreviewApp;
use eframe::egui::{self, Align, FontFamily, FontId, RichText, Sense, Ui};
use egui_shadcn::{
    AccordionItemProps, AccordionProps, SeparatorProps, TextProps, accordion, accordion_item,
    separator, text,
};
use lucide_icons::Icon;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    if compact {
        ui.vertical_centered(|center| {
            center.set_min_width(240.0);
            center.set_max_width(240.0);
            for (i, key) in ["item-1", "item-2", "item-3", "item-4"].iter().enumerate() {
                let is_open = app.accordion_value.as_deref() == Some(*key);
                let row_response = center
                    .allocate_ui_with_layout(
                        egui::vec2(240.0, 30.0),
                        egui::Layout::left_to_right(Align::Center),
                        |row| {
                            row.set_width(240.0);
                            let row_id = row.make_persistent_id(format!("preview-acc-row-{key}"));
                            let row_rect = row.max_rect();
                            let is_row_hovered = row.ctx().input(|i| {
                                i.pointer
                                    .hover_pos()
                                    .map(|p| row_rect.contains(p))
                                    .unwrap_or(false)
                            });

                            row.horizontal(|row| {
                                row.set_width(240.0);
                                let link_text = if is_row_hovered {
                                    RichText::new("the quick brown fox")
                                        .color(app.theme.palette.primary)
                                        .underline()
                                        .size(14.0)
                                } else {
                                    RichText::new("the quick brown fox")
                                        .color(app.theme.palette.foreground)
                                        .size(14.0)
                                };
                                row.label(link_text);
                                row.with_layout(
                                    egui::Layout::right_to_left(Align::Center),
                                    |right| {
                                        right.label(
                                            RichText::new(if is_open {
                                                Icon::ChevronUp.unicode()
                                            } else {
                                                Icon::ChevronDown.unicode()
                                            })
                                            .font(FontId::new(
                                                14.0,
                                                FontFamily::Name("lucide".into()),
                                            ))
                                            .color(app.theme.palette.foreground),
                                        );
                                    },
                                );
                            });
                            row.interact(row_rect, row_id, Sense::click())
                        },
                    )
                    .inner;

                if row_response.clicked() {
                    if is_open {
                        app.accordion_value = None;
                    } else {
                        app.accordion_value = Some((*key).to_owned());
                    }
                }

                if is_open {
                    center.add_space(2.0);
                    let _ = text(
                        center,
                        &app.theme,
                        TextProps::new("jumps over the lazy dog.")
                            .size(12.0)
                            .color(egui_shadcn::TypographyColor::Muted),
                    );
                    center.add_space(6.0);
                }

                if i < 3 {
                    let _ = separator(
                        center,
                        &app.theme,
                        SeparatorProps::default().length(240.0).thickness(1.0),
                    );
                    center.add_space(6.0);
                }
            }
            center.add_space(10.0);
        });
        return;
    }

    accordion(
        ui,
        &app.theme,
        AccordionProps::new("preview-accordion", &mut app.accordion_value)
            .collapsible(true)
            .default_value("item-1"),
        |acc_ui, ctx| {
            accordion_item(
                acc_ui,
                &app.theme,
                ctx,
                AccordionItemProps::new("item-1"),
                |t_ui, _| text(t_ui, &app.theme, TextProps::new("the quick brown fox")),
                |c_ui| {
                    let _ = text(c_ui, &app.theme, TextProps::new("jumps over the lazy dog."));
                },
            );
            accordion_item(
                acc_ui,
                &app.theme,
                ctx,
                AccordionItemProps::new("item-2"),
                |t_ui, _| text(t_ui, &app.theme, TextProps::new("lorem ipsum")),
                |c_ui| {
                    let _ = text(c_ui, &app.theme, TextProps::new("dolor sit amet."));
                },
            );
        },
    );
}
