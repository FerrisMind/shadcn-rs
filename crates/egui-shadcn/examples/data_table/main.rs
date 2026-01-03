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
use egui_shadcn::{DataTableAlign, DataTableColumn, DataTableProps, SortValue, Theme, data_table};

struct Task {
    id: &'static str,
    title: &'static str,
    status: &'static str,
    priority: &'static str,
}

const TASKS: [Task; 12] = [
    Task {
        id: "TASK-1001",
        title: "Review pull requests",
        status: "In Progress",
        priority: "High",
    },
    Task {
        id: "TASK-1002",
        title: "Update design tokens",
        status: "Todo",
        priority: "Medium",
    },
    Task {
        id: "TASK-1003",
        title: "Fix pagination layout",
        status: "Todo",
        priority: "Low",
    },
    Task {
        id: "TASK-1004",
        title: "Ship monthly report",
        status: "Done",
        priority: "High",
    },
    Task {
        id: "TASK-1005",
        title: "Add keyboard shortcuts",
        status: "In Progress",
        priority: "Medium",
    },
    Task {
        id: "TASK-1006",
        title: "Audit accessibility",
        status: "Todo",
        priority: "High",
    },
    Task {
        id: "TASK-1007",
        title: "Sync marketing copy",
        status: "Done",
        priority: "Low",
    },
    Task {
        id: "TASK-1008",
        title: "Prepare onboarding docs",
        status: "In Progress",
        priority: "Medium",
    },
    Task {
        id: "TASK-1009",
        title: "Refine table styles",
        status: "Todo",
        priority: "Medium",
    },
    Task {
        id: "TASK-1010",
        title: "Clean up build scripts",
        status: "Done",
        priority: "Low",
    },
    Task {
        id: "TASK-1011",
        title: "Write release notes",
        status: "In Progress",
        priority: "High",
    },
    Task {
        id: "TASK-1012",
        title: "Resolve support tickets",
        status: "Todo",
        priority: "High",
    },
];

struct DataTableExample {
    theme: Theme,
}

impl DataTableExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for DataTableExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Data Table");
            ui.label("Sortable, filterable, and paginated rows.");
            ui.add_space(16.0);

            let columns = vec![
                DataTableColumn::new("id", "ID", |ui, task: &Task| {
                    ui.label(task.id);
                })
                .sort_by(|task| SortValue::Str(task.id.to_string()))
                .filter_by(|task| task.id.to_string())
                .hideable(false)
                .width(110.0),
                DataTableColumn::new("title", "Title", |ui, task: &Task| {
                    ui.label(task.title);
                })
                .sort_by(|task| SortValue::Str(task.title.to_string()))
                .filter_by(|task| task.title.to_string())
                .width(260.0),
                DataTableColumn::new("status", "Status", |ui, task: &Task| {
                    ui.label(task.status);
                })
                .sort_by(|task| SortValue::Str(task.status.to_string()))
                .filter_by(|task| task.status.to_string())
                .width(120.0),
                DataTableColumn::new("priority", "Priority", |ui, task: &Task| {
                    ui.label(task.priority);
                })
                .sort_by(|task| SortValue::Str(task.priority.to_string()))
                .filter_by(|task| task.priority.to_string())
                .width(120.0)
                .align(DataTableAlign::Right),
            ];

            let response = data_table(
                ui,
                &self.theme,
                DataTableProps::new(Id::new("tasks-table"), columns, &TASKS)
                    .page_size(5)
                    .filter_placeholder("Filter tasks..."),
            );

            ui.add_space(8.0);
            ui.label(
                RichText::new(format!(
                    "{} of {} row(s) selected",
                    response.selected.len(),
                    response.filtered_rows
                ))
                .color(self.theme.palette.muted_foreground)
                .size(12.0),
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Data Table example",
        options,
        Box::new(|_cc| Ok(Box::new(DataTableExample::new()))),
    )
}
