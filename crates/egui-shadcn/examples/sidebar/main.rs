#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{Align, Layout, RichText};
use egui_shadcn::{
    SidebarGroupLabelProps, SidebarGroupProps, SidebarMenuButtonProps, SidebarMenuButtonSize,
    SidebarProps, SidebarProviderProps, Theme, sidebar, sidebar_content, sidebar_footer,
    sidebar_group, sidebar_group_content, sidebar_group_label, sidebar_header, sidebar_menu,
    sidebar_menu_button, sidebar_menu_item, sidebar_provider, sidebar_trigger,
};

struct SidebarDemo {
    theme: Theme,
    sidebar_open: bool,
}

impl SidebarDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            sidebar_open: true,
        }
    }
}

impl App for SidebarDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            sidebar_provider(
                ui,
                SidebarProviderProps::new(
                    ui.make_persistent_id("sidebar-demo"),
                    &mut self.sidebar_open,
                )
                .expanded_width(240.0)
                .collapsed_width(64.0),
                |ui, sidebar_ctx| {
                    ui.horizontal(|layout| {
                        let _ = sidebar(
                            layout,
                            &self.theme,
                            sidebar_ctx,
                            SidebarProps::new(),
                            |sidebar_ui, sidebar_ctx| {
                                sidebar_header(sidebar_ui, sidebar_ctx, |header_ui, _ctx| {
                                    header_ui.horizontal(|row| {
                                        row.label(RichText::new("Acme Inc").strong().size(14.0));
                                    });
                                });

                                sidebar_content(sidebar_ui, sidebar_ctx, |content_ui, ctx| {
                                    content_ui.spacing_mut().item_spacing.y = 12.0;

                                    sidebar_group(
                                        content_ui,
                                        ctx,
                                        SidebarGroupProps::new(),
                                        |group_ui| {
                                            sidebar_group_label(
                                                group_ui,
                                                &self.theme,
                                                ctx,
                                                SidebarGroupLabelProps::new("Navigation"),
                                            );
                                            sidebar_group_content(group_ui, ctx, |group_ui| {
                                                sidebar_menu(group_ui, |menu_ui| {
                                                    sidebar_menu_item(menu_ui, |item_ui| {
                                                        let _ = sidebar_menu_button(
                                                            item_ui,
                                                            &self.theme,
                                                            ctx,
                                                            SidebarMenuButtonProps::new("Overview")
                                                                .active(true),
                                                        );
                                                    });
                                                    sidebar_menu_item(menu_ui, |item_ui| {
                                                        let _ = sidebar_menu_button(
                                                            item_ui,
                                                            &self.theme,
                                                            ctx,
                                                            SidebarMenuButtonProps::new("Projects"),
                                                        );
                                                    });
                                                    sidebar_menu_item(menu_ui, |item_ui| {
                                                        let _ = sidebar_menu_button(
                                                            item_ui,
                                                            &self.theme,
                                                            ctx,
                                                            SidebarMenuButtonProps::new("Tasks"),
                                                        );
                                                    });
                                                    sidebar_menu_item(menu_ui, |item_ui| {
                                                        let _ = sidebar_menu_button(
                                                            item_ui,
                                                            &self.theme,
                                                            ctx,
                                                            SidebarMenuButtonProps::new("Calendar"),
                                                        );
                                                    });
                                                });
                                            });
                                        },
                                    );

                                    sidebar_group(
                                        content_ui,
                                        ctx,
                                        SidebarGroupProps::new(),
                                        |group_ui| {
                                            sidebar_group_label(
                                                group_ui,
                                                &self.theme,
                                                ctx,
                                                SidebarGroupLabelProps::new("Settings"),
                                            );
                                            sidebar_group_content(group_ui, ctx, |group_ui| {
                                                sidebar_menu(group_ui, |menu_ui| {
                                                    sidebar_menu_item(menu_ui, |item_ui| {
                                                        let _ = sidebar_menu_button(
                                                            item_ui,
                                                            &self.theme,
                                                            ctx,
                                                            SidebarMenuButtonProps::new("Profile"),
                                                        );
                                                    });
                                                    sidebar_menu_item(menu_ui, |item_ui| {
                                                        let _ = sidebar_menu_button(
                                                            item_ui,
                                                            &self.theme,
                                                            ctx,
                                                            SidebarMenuButtonProps::new(
                                                                "Preferences",
                                                            ),
                                                        );
                                                    });
                                                });
                                            });
                                        },
                                    );
                                });

                                sidebar_footer(sidebar_ui, sidebar_ctx, |footer_ui, ctx| {
                                    sidebar_menu(footer_ui, |menu_ui| {
                                        sidebar_menu_item(menu_ui, |item_ui| {
                                            let _ = sidebar_menu_button(
                                                item_ui,
                                                &self.theme,
                                                ctx,
                                                SidebarMenuButtonProps::new("Log out")
                                                    .size(SidebarMenuButtonSize::Sm),
                                            );
                                        });
                                    });
                                });
                            },
                        );

                        layout.add_space(16.0);
                        layout.vertical(|content_ui| {
                            content_ui.with_layout(Layout::top_down(Align::Min), |content_ui| {
                                sidebar_trigger(
                                    content_ui,
                                    &self.theme,
                                    sidebar_ctx,
                                    "Toggle sidebar",
                                );
                                content_ui.add_space(12.0);
                                content_ui.heading("Main content");
                                content_ui.add_space(8.0);
                                content_ui.label(
                                    RichText::new(
                                        "Use the toggle button to collapse or expand the sidebar.",
                                    )
                                    .size(13.0),
                                );
                            });
                        });
                    });
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Sidebar example",
        options,
        Box::new(|_cc| Ok(Box::new(SidebarDemo::new()))),
    )
}
