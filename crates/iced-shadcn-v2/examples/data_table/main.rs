//! Interactive playground for iced-shadcn-v2::DataTable.
//!
//! The left column controls the shared shadcn-common theme. The right column
//! mirrors shadcn-svelte's data-table demo: controlled email filtering,
//! column visibility, row selection, and a sortable Email header.
//!
//! Run with: cargo run -p iced-shadcn-v2 --example data_table

use std::fmt;

use chorale_core::{
    Alignment as ColumnAlignment, CellValue, ColumnDef, ColumnId, FilterKind, FilterValue, RowId,
    SortAction, TableState, deselect_all_visible_page, select_all_visible_page,
    set_column_visibility, set_filter, set_page, set_page_size, set_selection, toggle_sort,
};
use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, ButtonRadius, ButtonSize, ButtonVariant, CheckboxSize, CheckboxVariant,
    DataTable, FontHeading, FontId, FontPack, InputRadius, InputSize, RadiusId, StyleId, Theme,
    ThemeMode, fonts, iced_font,
};

const EMAIL_COLUMN: ColumnId = ColumnId("email");

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    state: TableState<Payment>,
    filter: String,
    filter_input_size: InputSize,
    filter_input_radius: Option<InputRadius>,
    sort_button_variant: ButtonVariant,
    sort_button_size: ButtonSize,
    sort_button_radius: Option<ButtonRadius>,
    columns_button_variant: ButtonVariant,
    columns_button_size: ButtonSize,
    columns_button_radius: Option<ButtonRadius>,
    pagination_button_variant: ButtonVariant,
    pagination_button_size: ButtonSize,
    pagination_button_radius: Option<ButtonRadius>,
    checkbox_variant: CheckboxVariant,
    checkbox_size: CheckboxSize,
    parts_accent: Option<AccentColor>,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    Filter(String),
    Sort(ColumnId, SortAction),
    Page(usize),
    Select(RowId, bool),
    SelectAll(bool),
    ColumnVisibility(ColumnId, bool),
    FilterInputSize(InputSize),
    FilterInputRadius(StyleOption<InputRadius>),
    SortButtonVariant(ButtonVariant),
    SortButtonSize(ButtonSize),
    SortButtonRadius(StyleOption<ButtonRadius>),
    ColumnsButtonVariant(ButtonVariant),
    ColumnsButtonSize(ButtonSize),
    ColumnsButtonRadius(StyleOption<ButtonRadius>),
    PaginationButtonVariant(ButtonVariant),
    PaginationButtonSize(ButtonSize),
    PaginationButtonRadius(StyleOption<ButtonRadius>),
    CheckboxVariant(CheckboxVariant),
    CheckboxSize(CheckboxSize),
    PartsAccent(AccentOpt),
}

impl Default for Example {
    fn default() -> Self {
        let state = TableState::new(sample_rows(), columns());
        let state = set_page_size(&state, 10).expect("the example page size must be non-zero");

        Self {
            theme: Theme::light(),
            state,
            filter: String::new(),
            filter_input_size: InputSize::Default,
            filter_input_radius: None,
            sort_button_variant: ButtonVariant::Ghost,
            sort_button_size: ButtonSize::Default,
            sort_button_radius: None,
            columns_button_variant: ButtonVariant::Outline,
            columns_button_size: ButtonSize::Default,
            columns_button_radius: None,
            pagination_button_variant: ButtonVariant::Outline,
            pagination_button_size: ButtonSize::Sm,
            pagination_button_radius: None,
            checkbox_variant: CheckboxVariant::Surface,
            checkbox_size: CheckboxSize::Xs,
            parts_accent: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Data Table".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::Filter(query) => {
                let filter = if query.is_empty() {
                    None
                } else {
                    Some(FilterValue::Text(query.clone()))
                };
                self.filter = query;
                self.state = set_filter(&self.state, EMAIL_COLUMN, filter);
            }
            Message::Sort(column, action) => {
                self.state = toggle_sort(&self.state, column, action);
            }
            Message::Page(page) => {
                if let Ok(next_state) = set_page(&self.state, page) {
                    self.state = next_state;
                }
            }
            Message::Select(row_id, selected) => {
                self.state = set_selection(&self.state, row_id, selected);
            }
            Message::SelectAll(selected) => {
                self.state = if selected {
                    select_all_visible_page(&self.state)
                } else {
                    deselect_all_visible_page(&self.state)
                };
            }
            Message::ColumnVisibility(column, visible) => {
                self.state = set_column_visibility(&self.state, column, visible);
            }
            Message::FilterInputSize(size) => {
                self.filter_input_size = size;
            }
            Message::FilterInputRadius(radius) => {
                self.filter_input_radius = radius.into_option();
            }
            Message::SortButtonVariant(variant) => {
                self.sort_button_variant = variant;
            }
            Message::SortButtonSize(size) => {
                self.sort_button_size = size;
            }
            Message::SortButtonRadius(radius) => {
                self.sort_button_radius = radius.into_option();
            }
            Message::ColumnsButtonVariant(variant) => {
                self.columns_button_variant = variant;
            }
            Message::ColumnsButtonSize(size) => {
                self.columns_button_size = size;
            }
            Message::ColumnsButtonRadius(radius) => {
                self.columns_button_radius = radius.into_option();
            }
            Message::PaginationButtonVariant(variant) => {
                self.pagination_button_variant = variant;
            }
            Message::PaginationButtonSize(size) => {
                self.pagination_button_size = size;
            }
            Message::PaginationButtonRadius(radius) => {
                self.pagination_button_radius = radius.into_option();
            }
            Message::CheckboxVariant(variant) => {
                self.checkbox_variant = variant;
            }
            Message::CheckboxSize(size) => {
                self.checkbox_size = size;
            }
            Message::PartsAccent(accent) => {
                self.parts_accent = accent.into_option();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;
        let font = iced_font(theme.font_pack().sans);

        let playground = column![
            section_label("Playground", palette.foreground, theme.font_pack()),
            section_label("Theme", palette.muted_foreground, theme.font_pack()),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "radius lg={:.0}px, sans={}, heading={}",
                theme.radius_scale().lg_px,
                theme.font_pack().sans.title(),
                theme.font_heading().title(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
            section_label(
                "Composed parts",
                palette.muted_foreground,
                theme.font_pack()
            ),
            control_select(
                "Filter size",
                &INPUT_SIZES,
                Some(self.filter_input_size),
                Message::FilterInputSize,
                theme,
            ),
            control_select(
                "Filter radius",
                &INPUT_RADII,
                Some(StyleOption::from_option(self.filter_input_radius)),
                Message::FilterInputRadius,
                theme,
            ),
            control_select(
                "Sort variant",
                &BUTTON_VARIANTS,
                Some(self.sort_button_variant),
                Message::SortButtonVariant,
                theme,
            ),
            control_select(
                "Sort size",
                &BUTTON_TEXT_SIZES,
                Some(self.sort_button_size),
                Message::SortButtonSize,
                theme,
            ),
            control_select(
                "Sort radius",
                &BUTTON_RADII,
                Some(StyleOption::from_option(self.sort_button_radius)),
                Message::SortButtonRadius,
                theme,
            ),
            control_select(
                "Columns variant",
                &BUTTON_VARIANTS,
                Some(self.columns_button_variant),
                Message::ColumnsButtonVariant,
                theme,
            ),
            control_select(
                "Columns size",
                &BUTTON_TEXT_SIZES,
                Some(self.columns_button_size),
                Message::ColumnsButtonSize,
                theme,
            ),
            control_select(
                "Columns radius",
                &BUTTON_RADII,
                Some(StyleOption::from_option(self.columns_button_radius)),
                Message::ColumnsButtonRadius,
                theme,
            ),
            control_select(
                "Pagination variant",
                &BUTTON_VARIANTS,
                Some(self.pagination_button_variant),
                Message::PaginationButtonVariant,
                theme,
            ),
            control_select(
                "Pagination size",
                &BUTTON_TEXT_SIZES,
                Some(self.pagination_button_size),
                Message::PaginationButtonSize,
                theme,
            ),
            control_select(
                "Pagination radius",
                &BUTTON_RADII,
                Some(StyleOption::from_option(self.pagination_button_radius)),
                Message::PaginationButtonRadius,
                theme,
            ),
            control_select(
                "Checkbox variant",
                &CHECKBOX_VARIANTS,
                Some(self.checkbox_variant),
                Message::CheckboxVariant,
                theme,
            ),
            control_select(
                "Checkbox size",
                &CHECKBOX_SIZES,
                Some(self.checkbox_size),
                Message::CheckboxSize,
                theme,
            ),
            control_select(
                "Parts accent",
                &ACCENTS,
                Some(AccentOpt::from_option(self.parts_accent)),
                Message::PartsAccent,
                theme,
            ),
        ]
        .spacing(10)
        .width(Length::Fill);

        let mut data_table = DataTable::new(theme, &self.state)
            .filter_value(self.filter.clone())
            .filter_placeholder("Filter emails...")
            .filter_input_size(self.filter_input_size)
            .sort_button_variant(self.sort_button_variant)
            .sort_button_size(self.sort_button_size)
            .columns_button_variant(self.columns_button_variant)
            .columns_button_size(self.columns_button_size)
            .pagination_button_variant(self.pagination_button_variant)
            .pagination_button_size(self.pagination_button_size)
            .checkbox_variant(self.checkbox_variant)
            .checkbox_size(self.checkbox_size)
            .on_global_filter(Message::Filter)
            .on_sort(Message::Sort)
            .on_page(Message::Page)
            .on_select(Message::Select)
            .on_select_all(Message::SelectAll)
            .on_column_visibility(Message::ColumnVisibility);
        if let Some(radius) = self.filter_input_radius {
            data_table = data_table.filter_input_radius(radius);
        }
        if let Some(radius) = self.sort_button_radius {
            data_table = data_table.sort_button_radius(radius);
        }
        if let Some(radius) = self.columns_button_radius {
            data_table = data_table.columns_button_radius(radius);
        }
        if let Some(radius) = self.pagination_button_radius {
            data_table = data_table.pagination_button_radius(radius);
        }
        if let Some(accent) = self.parts_accent {
            data_table = data_table
                .filter_input_color(accent)
                .sort_button_color(accent)
                .columns_button_color(accent)
                .pagination_button_color(accent);
        }
        let data_table: Element<'_, Message> = data_table.into();

        let preview = column![
            text("Data Table")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Filter, sort, select, and hide columns")
                .size(14)
                .font(font)
                .color(palette.muted_foreground),
            data_table,
        ]
        .spacing(16)
        .width(Length::Fill);

        let playground = container(scrollable(playground).height(Length::Fill))
            .width(Length::Fixed(280.0))
            .height(Length::Fill)
            .padding(16)
            .style(move |_| container::Style {
                background: Some(Background::Color(palette.card)),
                border: Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius_scale().lg_px.into(),
                },
                ..container::Style::default()
            });

        container(
            row![
                playground,
                scrollable(preview).width(Length::Fill).height(Length::Fill)
            ]
            .spacing(24)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

#[derive(Clone, Debug)]
struct Payment {
    status: &'static str,
    email: &'static str,
    amount: &'static str,
}

fn columns() -> Vec<ColumnDef<Payment>> {
    vec![
        ColumnDef::new(ColumnId("status"), "Status", |row: &Payment| {
            CellValue::Text(row.status.to_owned())
        }),
        ColumnDef::new(EMAIL_COLUMN, "Email", |row: &Payment| {
            CellValue::Text(row.email.to_owned())
        })
        .sortable()
        .filter(FilterKind::Text),
        ColumnDef::new(ColumnId("amount"), "Amount", |row: &Payment| {
            CellValue::Text(row.amount.to_owned())
        })
        .alignment(ColumnAlignment::Right),
    ]
}

fn sample_rows() -> Vec<(RowId, Payment)> {
    vec![
        payment("Success", "ken99@yahoo.com", "$316.00"),
        payment("Success", "Abe45@gmail.com", "$242.00"),
        payment("Processing", "Monserrat44@gmail.com", "$837.00"),
        payment("Success", "Silas22@gmail.com", "$874.00"),
        payment("Failed", "carmella@hotmail.com", "$721.00"),
    ]
}

fn payment(status: &'static str, email: &'static str, amount: &'static str) -> (RowId, Payment) {
    (
        RowId::new(),
        Payment {
            status,
            email,
            amount,
        },
    )
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fill)
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StyleOption<T> {
    Theme,
    Value(T),
}

impl<T> StyleOption<T> {
    fn from_option(value: Option<T>) -> Self {
        match value {
            None => Self::Theme,
            Some(value) => Self::Value(value),
        }
    }

    fn into_option(self) -> Option<T> {
        match self {
            Self::Theme => None,
            Self::Value(value) => Some(value),
        }
    }
}

impl<T: fmt::Display> fmt::Display for StyleOption<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Theme => formatter.write_str("theme"),
            Self::Value(value) => value.fmt(formatter),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("none"),
            Self::Color(color) => formatter.write_str(color.as_str()),
        }
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];

const INPUT_SIZES: [InputSize; 3] = [InputSize::Sm, InputSize::Default, InputSize::Lg];

const INPUT_RADII: [StyleOption<InputRadius>; 6] = [
    StyleOption::Theme,
    StyleOption::Value(InputRadius::None),
    StyleOption::Value(InputRadius::Small),
    StyleOption::Value(InputRadius::Medium),
    StyleOption::Value(InputRadius::Large),
    StyleOption::Value(InputRadius::Full),
];

const BUTTON_VARIANTS: [ButtonVariant; 8] = [
    ButtonVariant::Default,
    ButtonVariant::Destructive,
    ButtonVariant::Outline,
    ButtonVariant::Secondary,
    ButtonVariant::Ghost,
    ButtonVariant::Link,
    ButtonVariant::Soft,
    ButtonVariant::Surface,
];

const BUTTON_TEXT_SIZES: [ButtonSize; 4] = [
    ButtonSize::Xs,
    ButtonSize::Sm,
    ButtonSize::Default,
    ButtonSize::Lg,
];

const BUTTON_RADII: [StyleOption<ButtonRadius>; 6] = [
    StyleOption::Theme,
    StyleOption::Value(ButtonRadius::None),
    StyleOption::Value(ButtonRadius::Small),
    StyleOption::Value(ButtonRadius::Medium),
    StyleOption::Value(ButtonRadius::Large),
    StyleOption::Value(ButtonRadius::Full),
];

const CHECKBOX_VARIANTS: [CheckboxVariant; 3] = [
    CheckboxVariant::Surface,
    CheckboxVariant::Classic,
    CheckboxVariant::Soft,
];

const CHECKBOX_SIZES: [CheckboxSize; 4] = [
    CheckboxSize::Xs,
    CheckboxSize::Sm,
    CheckboxSize::Md,
    CheckboxSize::Lg,
];
