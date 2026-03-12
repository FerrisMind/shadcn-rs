use super::app::{ComponentTab, EguiPreviewApp, InstallTab};
use super::catalog::{COMPONENT_SLUGS, component_code, component_title};
use super::demos::render_component_preview;
use super::ui_home::icon_text;
use eframe::egui::{self, Id, Ui};
use egui_shadcn::{
    Button, ButtonSize, ButtonVariant, CodeProps, CodeVariant, HeadingProps, ScrollAreaProps,
    ScrollDirection, TabItem, TabsProps, TabsVariant, TextProps, heading, scroll_area, tabs, text,
};
use lucide_icons::Icon;

pub fn render_component(app: &mut EguiPreviewApp, ui: &mut Ui, index: usize) {
    let slug = COMPONENT_SLUGS[index];
    let title = component_title(slug);
    let theme = app.theme.clone();

    scroll_area(
        ui,
        &theme,
        ScrollAreaProps::default().direction(ScrollDirection::Vertical),
        |ui| {
            let viewport_width = ui.available_width();
            let content_width = (viewport_width * 0.94).min(1200.0);
            let _ = ui.allocate_ui_with_layout(
                egui::vec2(viewport_width, 0.0),
                egui::Layout::top_down(egui::Align::Center),
                |outer| {
                    let _ = outer.allocate_ui_with_layout(
                        egui::vec2(content_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |content| {
                            content.add_space(64.0);
                            let mut view_tab = match app.tab {
                                ComponentTab::Demo => "demo".to_owned(),
                                ComponentTab::Code => "code".to_owned(),
                            };
                            app.tab = if view_tab == "code" {
                                ComponentTab::Code
                            } else {
                                ComponentTab::Demo
                            };

                            let grid_gap = 20.0;
                            let use_two_columns = content_width >= 900.0;
                            if use_two_columns {
                                let grid_width = content_width.min(1080.0);
                                let column_width = ((grid_width - grid_gap) * 0.5).max(1.0);
                                let _ = content.allocate_ui_with_layout(
                                    egui::vec2(content_width, 0.0),
                                    egui::Layout::top_down(egui::Align::Center),
                                    |centered| {
                                        centered.horizontal_top(|row| {
                                            row.spacing_mut().item_spacing.x = grid_gap;
                                            let _ = row.allocate_ui_with_layout(
                                                egui::vec2(column_width, 0.0),
                                                egui::Layout::top_down(egui::Align::Min),
                                                |left| {
                                            let _ = heading(
                                                left,
                                                &theme,
                                                HeadingProps::new(title).size(42.0),
                                            );
                                            left.add_space(10.0);
                                            let _ = text(
                                                left,
                                                &theme,
                                                TextProps::new(format!("Component: {slug}")),
                                            );
                                            left.add_space(18.0);
                                            let _ = heading(
                                                left,
                                                &theme,
                                                HeadingProps::new("Installation").size(32.0),
                                            );
                                            left.add_space(8.0);
                                            let mut install = match app.install_tab {
                                                InstallTab::Automatic => "automatic".to_owned(),
                                                InstallTab::Manual => "manual".to_owned(),
                                            };
                                            let install_items = [
                                                TabItem::new("automatic", "Automatic"),
                                                TabItem::new("manual", "Manual"),
                                            ];
                                            let _ = tabs(
                                                left,
                                                &theme,
                                                TabsProps::new(
                                                    Id::new("component-install-tabs"),
                                                    &install_items,
                                                    &mut install,
                                                )
                                                .variant(TabsVariant::Soft)
                                                .scrollable(false),
                                                |_content, _active| {},
                                            );
                                            app.install_tab = if install == "manual" {
                                                InstallTab::Manual
                                            } else {
                                                InstallTab::Automatic
                                            };
                                            left.add_space(10.0);
                                            match app.install_tab {
                                                InstallTab::Automatic => {
                                                    let _ = text(
                                                        left,
                                                        &theme,
                                                        TextProps::new("1. Install CLI"),
                                                    );
                                                    install_command(
                                                        app,
                                                        left,
                                                        "cargo install shadcn-rs-cli",
                                                    );
                                                    left.add_space(8.0);
                                                    let _ = text(
                                                        left,
                                                        &theme,
                                                        TextProps::new("2. Add component"),
                                                    );
                                                    install_command(
                                                        app,
                                                        left,
                                                        &format!("shadcn-rs add {slug}"),
                                                    );
                                                }
                                                InstallTab::Manual => {
                                                    let _ = text(
                                                        left,
                                                        &theme,
                                                        TextProps::new(
                                                            "1. Add `egui-shadcn` dependency.",
                                                        ),
                                                    );
                                                    let _ = text(
                                                        left,
                                                        &theme,
                                                        TextProps::new(
                                                            "2. Create component from example code.",
                                                        ),
                                                    );
                                                    install_command(app, left, "use egui_shadcn::*;");
                                                }
                                            }
                                                },
                                            );
                                            let _ = row.allocate_ui_with_layout(
                                                egui::vec2(column_width, 0.0),
                                                egui::Layout::top_down(egui::Align::Min),
                                                |right| {
                                            let view_items = [
                                                TabItem::new("demo", "DEMO"),
                                                TabItem::new("code", "CODE"),
                                            ];
                                            let _ = tabs(
                                                right,
                                                &theme,
                                                TabsProps::new(
                                                    Id::new("component-view-tabs"),
                                                    &view_items,
                                                    &mut view_tab,
                                                )
                                                .variant(TabsVariant::Soft)
                                                .scrollable(false),
                                                |_content, _active| {},
                                            );
                                            app.tab = if view_tab == "code" {
                                                ComponentTab::Code
                                            } else {
                                                ComponentTab::Demo
                                            };
                                            right.add_space(10.0);
                                            if app.tab == ComponentTab::Demo {
                                                egui_shadcn::card(
                                                    right,
                                                    &theme,
                                                    egui_shadcn::CardProps::default()
                                                        .variant(egui_shadcn::CardVariant::Outline)
                                                        .padding(egui::vec2(20.0, 20.0))
                                                        .shadow(false),
                                                    |demo_ui| {
                                                        demo_ui.set_min_height(360.0);
                                                        demo_ui.vertical_centered(|center| {
                                                            render_component_preview(
                                                                app, center, slug, false,
                                                            );
                                                        });
                                                    },
                                                );
                                            } else {
                                                egui_shadcn::card(
                                                    right,
                                                    &theme,
                                                    egui_shadcn::CardProps::default()
                                                        .variant(egui_shadcn::CardVariant::Outline)
                                                        .padding(egui::vec2(12.0, 12.0))
                                                        .shadow(false),
                                                    |code_card| {
                                                        code_card.set_min_height(360.0);
                                                        scroll_area(
                                                            code_card,
                                                            &theme,
                                                            ScrollAreaProps::default()
                                                                .id(Id::new("component-code-scroll"))
                                                                .direction(ScrollDirection::Both),
                                                            |code_ui| {
                                                                let _ = egui_shadcn::code(
                                                                    code_ui,
                                                                    &theme,
                                                                    CodeProps::new(component_code(slug))
                                                                        .variant(CodeVariant::Outline),
                                                                );
                                                            },
                                                        );
                                                    },
                                                );
                                            }
                                                },
                                            );
                                        });
                                    },
                                );
                            } else {
                                let _ = heading(content, &theme, HeadingProps::new(title).size(42.0));
                                content.add_space(10.0);
                                let view_items = [TabItem::new("demo", "DEMO"), TabItem::new("code", "CODE")];
                                let _ = tabs(
                                    content,
                                    &theme,
                                    TabsProps::new(Id::new("component-view-tabs"), &view_items, &mut view_tab)
                                        .variant(TabsVariant::Soft)
                                        .scrollable(false),
                                    |_content, _active| {},
                                );
                                app.tab = if view_tab == "code" {
                                    ComponentTab::Code
                                } else {
                                    ComponentTab::Demo
                                };
                                content.add_space(10.0);
                                if app.tab == ComponentTab::Demo {
                                    egui_shadcn::card(
                                        content,
                                        &theme,
                                        egui_shadcn::CardProps::default()
                                            .variant(egui_shadcn::CardVariant::Outline)
                                            .padding(egui::vec2(20.0, 20.0))
                                            .shadow(false),
                                        |demo_ui| {
                                            demo_ui.set_min_height(360.0);
                                            demo_ui.vertical_centered(|center| {
                                                render_component_preview(app, center, slug, false);
                                            });
                                        },
                                    );
                                } else {
                                    egui_shadcn::card(
                                        content,
                                        &theme,
                                        egui_shadcn::CardProps::default()
                                            .variant(egui_shadcn::CardVariant::Outline)
                                            .padding(egui::vec2(12.0, 12.0))
                                            .shadow(false),
                                        |code_card| {
                                            code_card.set_min_height(360.0);
                                            scroll_area(
                                                code_card,
                                                &theme,
                                                ScrollAreaProps::default()
                                                    .id(Id::new("component-code-scroll"))
                                                    .direction(ScrollDirection::Both),
                                                |code_ui| {
                                                    let _ = egui_shadcn::code(
                                                        code_ui,
                                                        &theme,
                                                        CodeProps::new(component_code(slug))
                                                            .variant(CodeVariant::Outline),
                                                    );
                                                },
                                            );
                                        },
                                    );
                                }
                                content.add_space(18.0);
                                let _ = heading(content, &theme, HeadingProps::new("Installation").size(32.0));
                                content.add_space(8.0);
                                let mut install = match app.install_tab {
                                    InstallTab::Automatic => "automatic".to_owned(),
                                    InstallTab::Manual => "manual".to_owned(),
                                };
                                let install_items = [
                                    TabItem::new("automatic", "Automatic"),
                                    TabItem::new("manual", "Manual"),
                                ];
                                let _ = tabs(
                                    content,
                                    &theme,
                                    TabsProps::new(
                                        Id::new("component-install-tabs"),
                                        &install_items,
                                        &mut install,
                                    )
                                    .variant(TabsVariant::Soft)
                                    .scrollable(false),
                                    |_content, _active| {},
                                );
                                app.install_tab = if install == "manual" {
                                    InstallTab::Manual
                                } else {
                                    InstallTab::Automatic
                                };
                                content.add_space(10.0);
                                match app.install_tab {
                                    InstallTab::Automatic => {
                                        let _ = text(content, &theme, TextProps::new("1. Install CLI"));
                                        install_command(app, content, "cargo install shadcn-rs-cli");
                                        content.add_space(8.0);
                                        let _ = text(
                                            content,
                                            &theme,
                                            TextProps::new("2. Add component"),
                                        );
                                        install_command(app, content, &format!("shadcn-rs add {slug}"));
                                    }
                                    InstallTab::Manual => {
                                        let _ = text(
                                            content,
                                            &theme,
                                            TextProps::new("1. Add `egui-shadcn` dependency."),
                                        );
                                        let _ = text(
                                            content,
                                            &theme,
                                            TextProps::new("2. Create component from example code."),
                                        );
                                        install_command(app, content, "use egui_shadcn::*;");
                                    }
                                }
                            }
                        },
                    );
                },
            );
        },
    );
}

fn install_command(app: &EguiPreviewApp, ui: &mut Ui, value: &str) {
    egui_shadcn::card(
        ui,
        &app.theme,
        egui_shadcn::CardProps::default()
            .variant(egui_shadcn::CardVariant::Outline)
            .padding(egui::vec2(14.0, 10.0))
            .shadow(false),
        |row| {
            row.horizontal(|line| {
                let _ = egui_shadcn::code(
                    line,
                    &app.theme,
                    CodeProps::new(value).variant(CodeVariant::Outline),
                );
                line.with_layout(egui::Layout::right_to_left(egui::Align::Center), |right| {
                    if Button::new(icon_text(Icon::Copy, 14.0))
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Icon)
                        .show(right, &app.theme)
                        .clicked()
                    {
                        right.ctx().copy_text(value.to_owned());
                    }
                });
            });
        },
    );
}
