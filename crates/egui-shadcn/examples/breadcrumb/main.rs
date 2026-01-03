#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, Id, RichText};
use egui_shadcn::{
    BreadcrumbProps, DropdownMenuItemProps, DropdownMenuProps, DropdownMenuTriggerProps, Theme,
    breadcrumb, breadcrumb_ellipsis, breadcrumb_item, breadcrumb_link, breadcrumb_list,
    breadcrumb_page, breadcrumb_separator, dropdown_menu, dropdown_menu_item,
    dropdown_menu_trigger,
};

struct BreadcrumbExample {
    theme: Theme,
}

impl BreadcrumbExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for BreadcrumbExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Breadcrumb");
            ui.label("Navigation chain with separators and collapse behavior.");
            ui.add_space(16.0);

            ui.spacing_mut().item_spacing.y = 20.0;

            render_section(ui, &self.theme, "Breadcrumb demo", "Ellipsis dropdown.");
            render_breadcrumb_demo(ui, &self.theme);

            render_section(ui, &self.theme, "Breadcrumb ellipsis", "Collapsed items.");
            render_breadcrumb_ellipsis(ui, &self.theme);

            render_section(
                ui,
                &self.theme,
                "Breadcrumb dropdown",
                "Dropdown in the middle.",
            );
            render_breadcrumb_dropdown(ui, &self.theme);

            render_section(ui, &self.theme, "Breadcrumb separator", "Custom separator.");
            render_breadcrumb_separator(ui, &self.theme);

            render_section(ui, &self.theme, "Breadcrumb link", "Clickable links.");
            render_breadcrumb_link(ui, &self.theme);

            render_section(
                ui,
                &self.theme,
                "Breadcrumb responsive",
                "Collapse on narrow widths.",
            );
            render_breadcrumb_responsive(ui, &self.theme);
        });
    }
}

fn render_section(ui: &mut egui::Ui, theme: &Theme, title: &str, desc: &str) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.label(
            RichText::new(desc)
                .color(theme.palette.muted_foreground)
                .size(12.0),
        );
        ui.add_space(8.0);
    });
}

fn render_breadcrumb_demo(ui: &mut egui::Ui, theme: &Theme) {
    breadcrumb(ui, theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Home");
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                let trigger = dropdown_menu_trigger(
                    ui,
                    DropdownMenuTriggerProps::new(Id::new("breadcrumb-demo-ellipsis")),
                    |ui| breadcrumb_ellipsis(ui, ctx),
                );
                let _ = dropdown_menu(
                    ui,
                    theme,
                    DropdownMenuProps::new(&trigger.response),
                    |menu_ui| {
                        if dropdown_menu_item(
                            menu_ui,
                            theme,
                            DropdownMenuItemProps::new("Documentation"),
                        )
                        .clicked()
                        {
                            menu_ui.close();
                        }
                        if dropdown_menu_item(menu_ui, theme, DropdownMenuItemProps::new("Themes"))
                            .clicked()
                        {
                            menu_ui.close();
                        }
                        if dropdown_menu_item(menu_ui, theme, DropdownMenuItemProps::new("GitHub"))
                            .clicked()
                        {
                            menu_ui.close();
                        }
                    },
                );
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Components");
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, "Breadcrumb");
            });
        });
    });
}

fn render_breadcrumb_ellipsis(ui: &mut egui::Ui, theme: &Theme) {
    breadcrumb(ui, theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Home");
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_ellipsis(ui, ctx);
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Components");
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, "Breadcrumb");
            });
        });
    });
}

fn render_breadcrumb_dropdown(ui: &mut egui::Ui, theme: &Theme) {
    breadcrumb(ui, theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Home");
            });
            breadcrumb_separator(ui, ctx, Some("/".into()));
            breadcrumb_item(ui, ctx, |ui| {
                let trigger = dropdown_menu_trigger(
                    ui,
                    DropdownMenuTriggerProps::new(Id::new("breadcrumb-dropdown-trigger")),
                    |ui| breadcrumb_link(ui, ctx, "Components v"),
                );
                let _ = dropdown_menu(
                    ui,
                    theme,
                    DropdownMenuProps::new(&trigger.response),
                    |menu_ui| {
                        if dropdown_menu_item(
                            menu_ui,
                            theme,
                            DropdownMenuItemProps::new("Documentation"),
                        )
                        .clicked()
                        {
                            menu_ui.close();
                        }
                        if dropdown_menu_item(menu_ui, theme, DropdownMenuItemProps::new("Themes"))
                            .clicked()
                        {
                            menu_ui.close();
                        }
                        if dropdown_menu_item(menu_ui, theme, DropdownMenuItemProps::new("GitHub"))
                            .clicked()
                        {
                            menu_ui.close();
                        }
                    },
                );
            });
            breadcrumb_separator(ui, ctx, Some("/".into()));
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, "Breadcrumb");
            });
        });
    });
}

fn render_breadcrumb_separator(ui: &mut egui::Ui, theme: &Theme) {
    breadcrumb(ui, theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Home");
            });
            breadcrumb_separator(ui, ctx, Some("/".into()));
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Components");
            });
            breadcrumb_separator(ui, ctx, Some("/".into()));
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, "Breadcrumb");
            });
        });
    });
}

fn render_breadcrumb_link(ui: &mut egui::Ui, theme: &Theme) {
    breadcrumb(ui, theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Home");
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Components");
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, "Breadcrumb");
            });
        });
    });
}

fn render_breadcrumb_responsive(ui: &mut egui::Ui, theme: &Theme) {
    let items = [
        "Home",
        "Documentation",
        "Building Your Application",
        "Data Fetching",
        "Caching and Revalidating",
    ];
    let show_ellipsis = ui.available_width() < 420.0 && items.len() > 3;

    breadcrumb(ui, theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, items[0]);
            });
            breadcrumb_separator(ui, ctx, None);

            if show_ellipsis {
                breadcrumb_item(ui, ctx, |ui| {
                    let trigger = dropdown_menu_trigger(
                        ui,
                        DropdownMenuTriggerProps::new(Id::new("breadcrumb-responsive-ellipsis")),
                        |ui| breadcrumb_ellipsis(ui, ctx),
                    );
                    let _ = dropdown_menu(
                        ui,
                        theme,
                        DropdownMenuProps::new(&trigger.response),
                        |menu_ui| {
                            for label in items.iter().skip(1).take(items.len().saturating_sub(2)) {
                                if dropdown_menu_item(
                                    menu_ui,
                                    theme,
                                    DropdownMenuItemProps::new(label),
                                )
                                .clicked()
                                {
                                    menu_ui.close();
                                }
                            }
                        },
                    );
                });
                breadcrumb_separator(ui, ctx, None);
            } else {
                for label in items.iter().skip(1).take(items.len().saturating_sub(2)) {
                    breadcrumb_item(ui, ctx, |ui| {
                        breadcrumb_link(ui, ctx, *label);
                    });
                    breadcrumb_separator(ui, ctx, None);
                }
            }

            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, items[items.len() - 2]);
            });
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, items[items.len() - 1]);
            });
        });
    });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Breadcrumb example",
        options,
        Box::new(|_cc| Ok(Box::new(BreadcrumbExample::new()))),
    )
}
