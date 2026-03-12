use super::app::{ComponentTab, EguiPreviewApp, InstallTab, Screen};
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
            ui.horizontal(|row| {
                if Button::new("< Back")
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Sm)
                    .show(row, &theme)
                    .clicked()
                {
                    app.screen = Screen::Home;
                }
            });
            ui.add_space(8.0);
            ui.vertical_centered(|center| {
                let _ = heading(center, &theme, HeadingProps::new(title).size(48.0));
            });
            ui.add_space(14.0);

            let mut view_tab = match app.tab {
                ComponentTab::Demo => "demo".to_owned(),
                ComponentTab::Code => "code".to_owned(),
            };
            let view_items = [TabItem::new("demo", "DEMO"), TabItem::new("code", "CODE")];
            let _ = tabs(
                ui,
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
            ui.add_space(10.0);

            if app.tab == ComponentTab::Demo {
                egui_shadcn::card(
                    ui,
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

                ui.add_space(18.0);
                let _ = heading(ui, &theme, HeadingProps::new("Installation").size(34.0));
                ui.add_space(8.0);
                let mut install = match app.install_tab {
                    InstallTab::Automatic => "automatic".to_owned(),
                    InstallTab::Manual => "manual".to_owned(),
                };
                let install_items = [
                    TabItem::new("automatic", "Automatic"),
                    TabItem::new("manual", "Manual"),
                ];
                let _ = tabs(
                    ui,
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
                ui.add_space(10.0);
                match app.install_tab {
                    InstallTab::Automatic => {
                        let _ = text(ui, &theme, TextProps::new("1. Install CLI"));
                        install_command(app, ui, "cargo install shadcn-rs-cli");
                        ui.add_space(8.0);
                        let _ = text(ui, &theme, TextProps::new("2. Add component"));
                        install_command(app, ui, &format!("shadcn-rs add {slug}"));
                    }
                    InstallTab::Manual => {
                        let _ = text(
                            ui,
                            &theme,
                            TextProps::new("1. Add `egui-shadcn` dependency."),
                        );
                        let _ = text(
                            ui,
                            &theme,
                            TextProps::new("2. Create component from example code."),
                        );
                        install_command(app, ui, "use egui_shadcn::*;");
                    }
                }
            } else {
                scroll_area(
                    ui,
                    &theme,
                    ScrollAreaProps::default()
                        .id(Id::new("component-code-scroll"))
                        .direction(ScrollDirection::Vertical),
                    |code_ui| {
                        let _ = egui_shadcn::code(
                            code_ui,
                            &theme,
                            CodeProps::new(component_code(slug)).variant(CodeVariant::Outline),
                        );
                    },
                );
            }
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
