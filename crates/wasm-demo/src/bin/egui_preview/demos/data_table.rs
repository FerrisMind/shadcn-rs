use super::super::app::EguiPreviewApp;
use eframe::egui::{Id, RichText, Ui};
use egui_shadcn::{DataTableAlign, DataTableColumn, DataTableProps, SortValue, data_table};

struct Task {
    id: &'static str,
    title: &'static str,
    status: &'static str,
}

const TASKS: [Task; 6] = [
    Task {
        id: "TASK-1",
        title: "Review PR",
        status: "In Progress",
    },
    Task {
        id: "TASK-2",
        title: "Design tokens",
        status: "Todo",
    },
    Task {
        id: "TASK-3",
        title: "Fix pagination",
        status: "Todo",
    },
    Task {
        id: "TASK-4",
        title: "Release notes",
        status: "Done",
    },
    Task {
        id: "TASK-5",
        title: "Accessibility",
        status: "Todo",
    },
    Task {
        id: "TASK-6",
        title: "Support inbox",
        status: "Done",
    },
];

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let columns = vec![
        DataTableColumn::new("id", "ID", |ui, row: &Task| {
            ui.label(row.id);
        })
        .sort_by(|row| SortValue::Str(row.id.to_owned()))
        .filter_by(|row| row.id.to_owned())
        .hideable(false)
        .width(90.0),
        DataTableColumn::new("title", "Title", |ui, row: &Task| {
            ui.label(row.title);
        })
        .sort_by(|row| SortValue::Str(row.title.to_owned()))
        .filter_by(|row| row.title.to_owned())
        .width(if compact { 120.0 } else { 220.0 }),
        DataTableColumn::new("status", "Status", |ui, row: &Task| {
            ui.label(row.status);
        })
        .sort_by(|row| SortValue::Str(row.status.to_owned()))
        .filter_by(|row| row.status.to_owned())
        .align(DataTableAlign::Right)
        .width(110.0),
    ];

    let response = data_table(
        ui,
        &app.theme,
        DataTableProps::new(Id::new("preview-data-table"), columns, &TASKS)
            .page_size(if compact { 3 } else { 5 })
            .filter_placeholder("Filter tasks..."),
    );

    ui.add_space(6.0);
    ui.label(
        RichText::new(format!("{} row(s)", response.filtered_rows))
            .size(12.0)
            .color(app.theme.palette.muted_foreground),
    );
}
