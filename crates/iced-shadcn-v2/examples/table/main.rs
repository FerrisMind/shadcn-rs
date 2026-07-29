//! Interactive playground for iced-shadcn-v2::Table.
//!
//! The first showcase is a direct port of shadcn-svelte's table-demo.
//! The remaining sections exercise the compositional iced API: selected and
//! hoverable rows, arbitrary iced content, spanning cells, responsive sizing,
//! horizontal overflow, alignment, and typography overrides.
//!
//! Run with cargo run -p iced-shadcn-v2 --example table.

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonSize, ButtonVariant, FontHeading, FontId, FontPack,
    FontWeight, RadiusId, StyleId, Table, TableBody, TableCaption, TableCell, TableCellAlignment,
    TableFooter, TableHead, TableHeader, TableRow, Theme, ThemeMode, fonts, iced_font,
};

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
    selected_row: Option<usize>,
    hoverable_rows: bool,
    force_overflow: bool,
    pressed_count: u32,
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
    SelectRow(usize),
    ClearSelection,
    ToggleHoverable,
    ToggleOverflow,
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            selected_row: None,
            hoverable_rows: true,
            force_overflow: false,
            pressed_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Table".to_owned()
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
            Message::SelectRow(index) => {
                self.selected_row = if self.selected_row == Some(index) {
                    None
                } else {
                    Some(index)
                };
            }
            Message::ClearSelection => {
                self.selected_row = None;
            }
            Message::ToggleHoverable => {
                self.hoverable_rows = !self.hoverable_rows;
            }
            Message::ToggleOverflow => {
                self.force_overflow = !self.force_overflow;
            }
            Message::Pressed => {
                self.pressed_count += 1;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;
        let font = iced_font(theme.font_pack().sans);

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                p.muted_foreground,
                theme.font_pack(),
            ),
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
                "radius lg={:.0}px · style={} · sans={} · heading={}",
                theme.radius_scale().lg_px,
                theme.style_id().as_str(),
                theme.font_pack().sans.title(),
                theme.font_heading().title(),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(8);

        let swatches = row![
            swatch("bg", p.background, p.border),
            swatch("fg", p.foreground, p.border),
            swatch("primary", p.primary, p.border),
            swatch("secondary", p.secondary, p.border),
            swatch("muted", p.muted, p.border),
            swatch("destructive", p.destructive, p.border),
            swatch("border", p.border, p.foreground),
        ]
        .spacing(8)
        .wrap();

        let table_controls = row![
            Button::text("Select first row", theme)
                .size(ButtonSize::Sm)
                .variant(ButtonVariant::Outline)
                .on_press(Message::SelectRow(0)),
            Button::text("Clear selection", theme)
                .size(ButtonSize::Sm)
                .variant(ButtonVariant::Ghost)
                .on_press(Message::ClearSelection),
            Button::text(
                if self.hoverable_rows {
                    "Row hover: on"
                } else {
                    "Row hover: off"
                },
                theme,
            )
            .size(ButtonSize::Sm)
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleHoverable),
            Button::text(
                if self.force_overflow {
                    "Width: min 1100px"
                } else {
                    "Width: responsive"
                },
                theme,
            )
            .size(ButtonSize::Sm)
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleOverflow),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let selection_description = match self.selected_row {
            Some(index) => format!("Selected row: {}", index + 1),
            None => "Selected row: none".to_owned(),
        };

        let content = column![
            text("iced-shadcn-v2 Table")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte parity: caption · header/body/footer · spans · arbitrary iced content")
                .size(14)
                .font(font)
                .color(p.muted_foreground),
            text(format!(
                "{} · hover={} · overflow={} · pressed actions={}",
                selection_description,
                if self.hoverable_rows { "on" } else { "off" },
                if self.force_overflow {
                    "min-width"
                } else {
                    "responsive"
                },
                self.pressed_count,
            ))
            .size(14)
            .font(font)
            .color(p.foreground),
            controls,
            section_label("Palette", p.muted_foreground, theme.font_pack()),
            swatches,
            section_label("Row behavior", p.muted_foreground, theme.font_pack()),
            text(
                "Move the pointer over any body row to see the muted/50 hover surface. Turn Row hover off to compare.",
            )
            .size(13)
            .font(font)
            .color(p.muted_foreground),
            table_controls,
            section_label(
                "Reference demo · shadcn-svelte table-demo",
                p.muted_foreground,
                theme.font_pack(),
            ),
            invoice_table(theme, self.hoverable_rows),
            section_label(
                "Selected and hoverable rows",
                p.muted_foreground,
                theme.font_pack(),
            ),
            selected_table(theme, self.selected_row, self.hoverable_rows),
            section_label(
                "Arbitrary iced content in cells",
                p.muted_foreground,
                theme.font_pack(),
            ),
            composed_content_table(theme, self.hoverable_rows),
            section_label(
                "Footer, colspan, and empty state",
                p.muted_foreground,
                theme.font_pack(),
            ),
            colspan_table(theme, self.hoverable_rows),
            section_label(
                "Responsive layout and horizontal overflow",
                p.muted_foreground,
                theme.font_pack(),
            ),
            text(
                if self.force_overflow {
                    "The next table uses min_width(1100.0) and can be scrolled horizontally."
                } else {
                    "The next table uses fluid columns; toggle Width to inspect the scrollable variant."
                },
            )
            .size(13)
            .font(font)
            .color(p.muted_foreground),
            width_table(theme, self.force_overflow, self.hoverable_rows),
            section_label(
                "Cell API · alignment, widths, and typography",
                p.muted_foreground,
                theme.font_pack(),
            ),
            formatting_table(theme, self.hoverable_rows),
        ]
        .spacing(16)
        .max_width(960)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn invoice_table<'a>(theme: &'a Theme, hoverable: bool) -> Element<'a, Message> {
    let body = INVOICES
        .into_iter()
        .map(|(invoice, status, method, amount)| {
            TableRow::new(theme)
                .hoverable(hoverable)
                .cell(TableCell::text(invoice, theme).font_weight(FontWeight::Medium))
                .cell(TableCell::text(status, theme))
                .cell(TableCell::text(method, theme))
                .cell(TableCell::text(amount, theme).align_x(TableCellAlignment::End))
        });

    Table::new(theme)
        .column_widths([
            Length::Fixed(100.0),
            Length::Fill,
            Length::Fill,
            Length::Fill,
        ])
        .caption(TableCaption::text("A list of your recent invoices.", theme))
        .header(
            TableHeader::new(theme).push(
                TableRow::new(theme)
                    .head(TableHead::text("Invoice", theme))
                    .head(TableHead::text("Status", theme))
                    .head(TableHead::text("Method", theme))
                    .head(TableHead::text("Amount", theme).align_x(TableCellAlignment::End)),
            ),
        )
        .body(TableBody::new(theme).extend(body))
        .footer(
            TableFooter::new(theme).push(
                TableRow::new(theme)
                    .hoverable(false)
                    .cell(TableCell::text("Total", theme).span(3))
                    .cell(TableCell::text("$2,500.00", theme).align_x(TableCellAlignment::End)),
            ),
        )
        .into()
}

fn selected_table<'a>(
    theme: &'a Theme,
    selected_row: Option<usize>,
    hoverable: bool,
) -> Element<'a, Message> {
    let body = PROJECTS
        .into_iter()
        .enumerate()
        .map(|(index, (project, owner, status))| {
            let is_selected = selected_row == Some(index);
            let action_variant = if is_selected {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Outline
            };

            TableRow::new(theme)
                .selected(is_selected)
                .hoverable(hoverable)
                .cell(TableCell::text(project, theme).font_weight(FontWeight::Medium))
                .cell(TableCell::text(owner, theme))
                .cell(TableCell::text(status, theme))
                .cell(TableCell::new(
                    Button::text(if is_selected { "Selected" } else { "Select" }, theme)
                        .size(ButtonSize::Sm)
                        .variant(action_variant)
                        .on_press(Message::SelectRow(index)),
                    theme,
                ))
        });

    Table::new(theme)
        .column_widths([
            Length::Fixed(140.0),
            Length::Fill,
            Length::Fill,
            Length::Fixed(112.0),
        ])
        .caption(TableCaption::text(
            "Each action button selects its row; the row keeps the semantic selected surface.",
            theme,
        ))
        .header(
            TableHeader::new(theme).push(
                TableRow::new(theme)
                    .hoverable(false)
                    .head(TableHead::text("Project", theme))
                    .head(TableHead::text("Owner", theme))
                    .head(TableHead::text("Status", theme))
                    .head(TableHead::text("Action", theme)),
            ),
        )
        .body(TableBody::new(theme).extend(body))
        .footer(
            TableFooter::new(theme).push(
                TableRow::new(theme).hoverable(false).cell(
                    TableCell::text(
                        match selected_row {
                            Some(index) => format!("Active selection: {}", PROJECTS[index].0),
                            None => "Active selection: none".to_owned(),
                        },
                        theme,
                    )
                    .span(4),
                ),
            ),
        )
        .into()
}

fn composed_content_table<'a>(theme: &'a Theme, hoverable: bool) -> Element<'a, Message> {
    let p = &theme.palette;
    let sans = iced_font(theme.font_pack().sans);

    let rows = [
        ("Design system", "12 components", "Published"),
        ("Documentation", "48 pages", "In review"),
        ("Examples", "19 playgrounds", "Published"),
    ];

    let body = rows.into_iter().map(|(name, details, state)| {
        let title = column![
            text(name).font(sans).size(14).color(p.foreground),
            text(details).font(sans).size(12).color(p.muted_foreground),
        ]
        .spacing(2);

        TableRow::new(theme)
            .hoverable(hoverable)
            .cell(TableCell::new(title, theme))
            .cell(TableCell::text(state, theme).font_weight(FontWeight::Medium))
            .cell(TableCell::new(
                Button::text("Open", theme)
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                theme,
            ))
    });

    Table::new(theme)
        .column_widths([Length::FillPortion(2), Length::Fill, Length::Fixed(96.0)])
        .caption(TableCaption::text(
            "TableCell::new accepts regular iced elements, not only text fragments.",
            theme,
        ))
        .header(
            TableHeader::new(theme).push(
                TableRow::new(theme)
                    .hoverable(false)
                    .head(TableHead::text("Resource", theme))
                    .head(TableHead::text("State", theme))
                    .head(TableHead::text("", theme)),
            ),
        )
        .body(TableBody::new(theme).extend(body))
        .into()
}

fn colspan_table<'a>(theme: &'a Theme, hoverable: bool) -> Element<'a, Message> {
    Table::new(theme)
        .column_widths([
            Length::Fixed(140.0),
            Length::Fill,
            Length::Fixed(140.0),
            Length::Fill,
        ])
        .caption(TableCaption::text(
            "Spanning cells map directly to HTML colspan semantics.",
            theme,
        ))
        .header(
            TableHeader::new(theme).push(
                TableRow::new(theme)
                    .hoverable(false)
                    .head(TableHead::text("Archive", theme))
                    .head(TableHead::text("Owner", theme))
                    .head(TableHead::text("Count", theme))
                    .head(TableHead::text("Last updated", theme)),
            ),
        )
        .body(
            TableBody::new(theme).push(
                TableRow::new(theme).hoverable(hoverable).height(72.0).cell(
                    TableCell::text("No archived invoices", theme)
                        .span(4)
                        .align_x(TableCellAlignment::Center),
                ),
            ),
        )
        .footer(
            TableFooter::new(theme).push(
                TableRow::new(theme)
                    .hoverable(false)
                    .cell(TableCell::text("Summary", theme).span(2))
                    .cell(TableCell::text("0", theme).align_x(TableCellAlignment::Center))
                    .cell(TableCell::text("Nothing archived", theme)),
            ),
        )
        .into()
}

fn width_table<'a>(
    theme: &'a Theme,
    force_overflow: bool,
    hoverable: bool,
) -> Element<'a, Message> {
    let min_width = if force_overflow { 1_100.0 } else { 0.0 };
    let body = WIDTH_ROWS
        .into_iter()
        .map(|(invoice, customer, description, method, amount)| {
            TableRow::new(theme)
                .hoverable(hoverable)
                .cell(TableCell::text(invoice, theme).font_weight(FontWeight::Medium))
                .cell(TableCell::text(customer, theme))
                .cell(TableCell::text(description, theme))
                .cell(TableCell::text(method, theme))
                .cell(TableCell::text(amount, theme).align_x(TableCellAlignment::End))
        });

    Table::new(theme)
        .min_width(min_width)
        .column_widths([
            Length::Fixed(100.0),
            Length::FillPortion(2),
            Length::FillPortion(3),
            Length::FillPortion(2),
            Length::Fixed(120.0),
        ])
        .header(
            TableHeader::new(theme).push(
                TableRow::new(theme)
                    .hoverable(false)
                    .head(TableHead::text("Invoice", theme))
                    .head(TableHead::text("Customer", theme))
                    .head(TableHead::text("Description", theme))
                    .head(TableHead::text("Method", theme))
                    .head(TableHead::text("Amount", theme).align_x(TableCellAlignment::End)),
            ),
        )
        .body(TableBody::new(theme).extend(body))
        .into()
}

fn formatting_table<'a>(theme: &'a Theme, hoverable: bool) -> Element<'a, Message> {
    let mono = iced_font(theme.font_pack().mono);

    Table::new(theme)
        .column_widths([
            Length::Fixed(140.0),
            Length::Fixed(160.0),
            Length::Fill,
            Length::Fixed(160.0),
        ])
        .caption(TableCaption::text(
            "Per-cell width, font, weight, size, line-height, padding, and alignment remain composable.",
            theme,
        ))
        .header(TableHeader::new(theme).push(
            TableRow::new(theme)
                .hoverable(hoverable)
                .head(TableHead::text("Property", theme))
                .head(TableHead::text("Value", theme).align_x(TableCellAlignment::Center))
                .head(TableHead::text("Notes", theme))
                .head(TableHead::text("Align", theme).align_x(TableCellAlignment::Center)),
        ))
        .body(TableBody::new(theme).extend([
            TableRow::new(theme)
                .hoverable(hoverable)
                .cell(TableCell::text("font_weight", theme).font_weight(FontWeight::Medium))
                .cell(
                    TableCell::text("Bold", theme)
                        .font(mono)
                        .font_weight(FontWeight::Bold)
                        .align_x(TableCellAlignment::Center),
                )
                .cell(TableCell::text("Semantic weight maps through shadcn-common.", theme))
                .cell(TableCell::text("center", theme).align_x(TableCellAlignment::Center)),
            TableRow::new(theme)
                .hoverable(hoverable)
                .cell(TableCell::text("text_size", theme).font_weight(FontWeight::Medium))
                .cell(
                    TableCell::text("15 px", theme)
                        .text_size(15.0)
                        .align_x(TableCellAlignment::Center),
                )
                .cell(TableCell::text("Numeric values are normalized at the builder boundary.", theme))
                .cell(TableCell::text("end", theme).align_x(TableCellAlignment::End)),
            TableRow::new(theme)
                .hoverable(false)
                .cell(TableCell::text("line_height", theme).font_weight(FontWeight::Medium))
                .cell(
                    TableCell::text("20 px", theme)
                        .line_height(20.0)
                        .align_x(TableCellAlignment::Center),
                )
                .cell(TableCell::text("Explicit line height keeps dense rows predictable.", theme))
                .cell(TableCell::text("start", theme).align_x(TableCellAlignment::Start)),
        ]))
        .into()
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
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(p.background),
                text_color: p.foreground,
                placeholder_color: p.muted_foreground,
                handle_color: p.muted_foreground,
                border: Border {
                    color: p.input,
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

fn swatch<'a>(label: &'static str, fill: Color, border: Color) -> Element<'a, Message> {
    column![
        container(text(""))
            .width(36)
            .height(36)
            .style(move |_| container::Style {
                background: Some(Background::Color(fill)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(label).size(10).color(border),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
        }
    }
}

const INVOICES: [(&str, &str, &str, &str); 7] = [
    ("INV001", "Paid", "Credit Card", "$250.00"),
    ("INV002", "Pending", "PayPal", "$150.00"),
    ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
    ("INV004", "Paid", "Credit Card", "$450.00"),
    ("INV005", "Paid", "PayPal", "$550.00"),
    ("INV006", "Pending", "Bank Transfer", "$200.00"),
    ("INV007", "Unpaid", "Credit Card", "$300.00"),
];

const PROJECTS: [(&str, &str, &str); 4] = [
    ("Nova UI", "FerrisMind", "Published"),
    ("Theme tokens", "Design team", "In review"),
    ("Iced port", "Platform", "Draft"),
    ("Docs refresh", "Developer experience", "Published"),
];

const WIDTH_ROWS: [(&str, &str, &str, &str, &str); 3] = [
    (
        "INV008",
        "Northwind Trading",
        "Annual design-system maintenance",
        "Bank Transfer",
        "$1,240.00",
    ),
    (
        "INV009",
        "Acme Research Laboratories",
        "Component accessibility audit",
        "Credit Card",
        "$860.00",
    ),
    (
        "INV010",
        "Contoso Product Studio",
        "Custom table integration and support",
        "PayPal",
        "$2,150.00",
    ),
];

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
