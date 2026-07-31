//! Rendering for the range-calendar.
//!
//! Mirrors the calendar render but replaces day-cell styling with range-aware
//! visuals (accent band for middles, primary endpoints, per-corner radii).

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::text::{Rich, Span};
use crate::iced_compat::widget::{
    Space, button as iced_button, canvas, column, container, row, text as iced_text,
};
use crate::iced_compat::{
    Background, Color, Element, Length, Point, Rectangle, Shadow, Vector, mouse,
};

use shadcn_common::{
    CALENDAR_DISABLED_OPACITY, CALENDAR_HEADER_GAP_PX, CALENDAR_HOVER_ACCENT_ALPHA,
    CALENDAR_MONTHS_GAP_PX, CALENDAR_NAV_ICON_PX, CALENDAR_TEXT_PX, CALENDAR_WEEK_ROW_GAP_PX,
    CALENDAR_WEEKDAY_TEXT_PX, CalendarCaptionLayout, CalendarDayState, CalendarMonthFormat,
    CalendarRecipe, DateParts, RangeDayPosition, calendar_date_in_bounds, calendar_default_years,
    calendar_month_grid, calendar_month_name, calendar_nav_target, calendar_next_disabled,
    calendar_prev_disabled, calendar_recipe, calendar_today_utc, calendar_visible_months,
    calendar_weekday_name, calendar_weekdays, calendar_year_name, days_in_month_of,
    range_calendar_day_pick, range_day_position, range_days_valid,
};
use twill_core::prelude::{Padding as TwillPadding, Spacing};

use super::RangeCalendar;
use crate::components::button::{Button, ButtonVariant};
use crate::components::select::{Select, SelectSize};
use crate::fonts::iced_font;
use crate::iced_compat::font::Weight;
use crate::recipes::component_radius_px;
use crate::theme::Theme;

pub(super) fn build_range_calendar<'a, Message>(
    root: RangeCalendar<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = root.theme;
    let recipe = calendar_recipe(theme.style_id());
    let today = root.today.unwrap_or_else(calendar_today_utc);
    let placeholder = root.placeholder.or(root.value.start).unwrap_or(today);

    let visible = calendar_visible_months(placeholder, root.number_of_months);
    let first_visible = visible[0];
    let last_visible = visible[visible.len() - 1];

    let can_navigate = root.on_placeholder_change.is_some() && !root.disabled;
    let prev_disabled = !can_navigate || calendar_prev_disabled(first_visible, root.min_value);
    let next_disabled = !can_navigate || calendar_next_disabled(last_visible, root.max_value);

    let nav_message = |forward: bool| {
        root.on_placeholder_change.as_ref().map(|callback| {
            callback(calendar_nav_target(
                first_visible,
                forward,
                root.paged_navigation,
                root.number_of_months,
            ))
        })
    };
    let prev_message = (!prev_disabled).then(|| nav_message(false)).flatten();
    let next_message = (!next_disabled).then(|| nav_message(true)).flatten();

    let last_index = visible.len() - 1;
    let mut months: Vec<Element<'a, Message>> = Vec::with_capacity(visible.len());
    for (month_index, month) in visible.iter().copied().enumerate() {
        let leading = (month_index == 0).then(|| {
            build_nav_button(
                &root,
                recipe.cell_size_px,
                false,
                prev_disabled,
                prev_message.clone(),
            )
        });
        let trailing = (month_index == last_index).then(|| {
            build_nav_button(
                &root,
                recipe.cell_size_px,
                true,
                next_disabled,
                next_message.clone(),
            )
        });

        months.push(build_month(
            &root,
            &recipe,
            MonthSlot {
                month,
                month_index,
                placeholder,
                today,
                leading,
                trailing,
            },
        ));
    }

    let months_row = row(months).spacing(CALENDAR_MONTHS_GAP_PX);

    let palette = theme.palette;
    let transparent = root.transparent;
    let bordered = root.bordered;
    let border_radius = component_radius_px(theme, theme.style.button_type().default_radius);

    container(months_row)
        .padding(recipe.pad_px)
        .style(move |_| {
            let mut style = container::Style {
                background: (!transparent).then_some(Background::Color(palette.background)),
                text_color: Some(palette.foreground),
                ..container::Style::default()
            };

            if bordered {
                style.border = crate::iced_compat::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: border_radius.into(),
                };
                style.shadow = Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                };
            }

            style
        })
        .into()
}

struct MonthSlot<'a, Message> {
    month: DateParts,
    month_index: usize,
    placeholder: DateParts,
    today: DateParts,
    leading: Option<Element<'a, Message>>,
    trailing: Option<Element<'a, Message>>,
}

fn build_month<'a, Message>(
    root: &RangeCalendar<'a, Message>,
    recipe: &CalendarRecipe,
    slot: MonthSlot<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let cell = recipe.cell_size_px;
    let MonthSlot {
        month,
        month_index,
        placeholder,
        today,
        leading,
        trailing,
    } = slot;

    let nav_slot = |element: Option<Element<'a, Message>>| {
        element.unwrap_or_else(|| {
            Space::new()
                .width(Length::Fixed(cell))
                .height(Length::Fixed(cell))
                .into()
        })
    };

    let caption = container(build_caption(root, month, month_index, placeholder))
        .width(Length::Fill)
        .height(Length::Fixed(cell))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    let header = row![nav_slot(leading), caption, nav_slot(trailing)]
        .align_y(Vertical::Center)
        .width(Length::Fixed(cell * 7.0));

    let theme = root.theme;
    let weekday_row = row(calendar_weekdays(root.week_starts_on)
        .into_iter()
        .map(|weekday| {
            let label = match root.weekday_label.as_ref() {
                Some(label) => label(weekday),
                None => calendar_weekday_name(weekday, root.weekday_format)
                    .chars()
                    .take(2)
                    .collect(),
            };

            container(
                iced_text(label)
                    .size(CALENDAR_WEEKDAY_TEXT_PX)
                    .font(iced_font(theme.font_pack().sans))
                    .color(theme.palette.muted_foreground),
            )
            .width(Length::Fixed(cell))
            .align_x(Horizontal::Center)
            .into()
        })
        .collect::<Vec<Element<'a, Message>>>());

    let mut grid: Vec<Element<'a, Message>> = vec![weekday_row.into()];
    for week in calendar_month_grid(month, root.week_starts_on, root.fixed_weeks) {
        let cells = week
            .into_iter()
            .map(|date| build_day(root, recipe, date, month, today))
            .collect::<Vec<Element<'a, Message>>>();
        grid.push(row(cells).into());
    }

    column![header, column(grid).spacing(CALENDAR_WEEK_ROW_GAP_PX)]
        .spacing(CALENDAR_MONTHS_GAP_PX)
        .into()
}

fn build_caption<'a, Message>(
    root: &RangeCalendar<'a, Message>,
    month: DateParts,
    month_index: usize,
    placeholder: DateParts,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let month_format = root.month_format.unwrap_or(match root.caption_layout {
        CalendarCaptionLayout::Label => CalendarMonthFormat::Long,
        _ => CalendarMonthFormat::Short,
    });
    let month_text = |value: u8| match root.month_label.as_ref() {
        Some(label) => label(value),
        None => calendar_month_name(value, month_format),
    };
    let year_text = |value: i32| match root.year_label.as_ref() {
        Some(label) => label(value),
        None => calendar_year_name(value, root.year_format),
    };

    match root.caption_layout {
        CalendarCaptionLayout::Label => caption_label(
            root.theme,
            format!("{} {}", month_text(month.month), year_text(month.year)),
        ),
        CalendarCaptionLayout::Dropdown => row![
            build_month_select(root, month, month_index, placeholder, &month_text),
            build_year_select(root, month, placeholder, &year_text),
        ]
        .spacing(CALENDAR_HEADER_GAP_PX)
        .align_y(Vertical::Center)
        .into(),
        CalendarCaptionLayout::DropdownMonths => row![
            build_month_select(root, month, month_index, placeholder, &month_text),
            caption_label(root.theme, year_text(placeholder.year)),
        ]
        .spacing(CALENDAR_HEADER_GAP_PX)
        .align_y(Vertical::Center)
        .into(),
        CalendarCaptionLayout::DropdownYears => row![
            caption_label(root.theme, month_text(placeholder.month)),
            build_year_select(root, month, placeholder, &year_text),
        ]
        .spacing(CALENDAR_HEADER_GAP_PX)
        .align_y(Vertical::Center)
        .into(),
        _ => caption_label(
            root.theme,
            format!("{} {}", month_text(month.month), year_text(month.year)),
        ),
    }
}

fn caption_label<'a, Message: 'a>(theme: &Theme, label: String) -> Element<'a, Message> {
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = Weight::Medium;
    iced_text(label)
        .size(CALENDAR_TEXT_PX)
        .font(font)
        .color(theme.palette.foreground)
        .into()
}

fn build_month_select<'a, Message>(
    root: &RangeCalendar<'a, Message>,
    month: DateParts,
    month_index: usize,
    placeholder: DateParts,
    month_text: &impl Fn(u8) -> String,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let month_numbers: Vec<u8> = match root.months.as_ref() {
        Some(months) => months.clone(),
        None => (1..=12).collect(),
    };

    let mut select: Select<'a, u8, Message> = Select::new(root.theme)
        .items(
            month_numbers
                .into_iter()
                .map(|value| (value, month_text(value))),
        )
        .selected(month.month)
        .deselectable(false)
        .size(SelectSize::Sm)
        .disabled(root.disabled || root.on_placeholder_change.is_none());

    if let Some(callback) = root.on_placeholder_change.clone() {
        select = select.on_select(move |picked: u8| {
            callback(shadcn_common::add_months(
                with_month(placeholder, picked),
                -(month_index as i32),
            ))
        });
    }

    select.into()
}

fn build_year_select<'a, Message>(
    root: &RangeCalendar<'a, Message>,
    month: DateParts,
    placeholder: DateParts,
    year_text: &impl Fn(i32) -> String,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let years: Vec<i32> = match root.years.as_ref() {
        Some(years) => years.clone(),
        None => calendar_default_years(
            placeholder.year,
            root.today.unwrap_or_else(calendar_today_utc).year,
            root.min_value,
            root.max_value,
        ),
    };

    let mut select: Select<'a, i32, Message> = Select::new(root.theme)
        .items(years.into_iter().map(|value| (value, year_text(value))))
        .selected(month.year)
        .deselectable(false)
        .size(SelectSize::Sm)
        .disabled(root.disabled || root.on_placeholder_change.is_none());

    if let Some(callback) = root.on_placeholder_change.clone() {
        select = select.on_select(move |picked: i32| callback(with_year(placeholder, picked)));
    }

    select.into()
}

fn build_nav_button<'a, Message>(
    root: &RangeCalendar<'a, Message>,
    cell: f32,
    forward: bool,
    disabled: bool,
    message: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = root.theme;
    let mut icon_color = nav_icon_color(theme, root.button_variant);
    if disabled {
        icon_color.a *= CALENDAR_DISABLED_OPACITY;
    }

    let icon = canvas(ChevronIcon {
        color: icon_color,
        forward,
    })
    .width(Length::Fixed(CALENDAR_NAV_ICON_PX))
    .height(Length::Fixed(CALENDAR_NAV_ICON_PX));

    Button::icon(icon, theme)
        .variant(root.button_variant)
        .radius(crate::components::button::ButtonRadius::Large)
        .width(Length::Fixed(cell))
        .height(Length::Fixed(cell))
        .padding(TwillPadding::all(Spacing::S0))
        .expect("Spacing::S0 padding is always resolvable")
        .disabled(disabled)
        .on_press_maybe(message)
        .style_override(move |mut style, status| {
            match status {
                iced_button::Status::Active => {
                    style.background = None;
                }
                iced_button::Status::Disabled => {
                    style.background = None;
                    style.border.color.a *= CALENDAR_DISABLED_OPACITY;
                    style.text_color.a *= CALENDAR_DISABLED_OPACITY;
                }
                _ => {}
            }
            style
        })
        .into()
}

fn nav_icon_color(theme: &Theme, variant: ButtonVariant) -> Color {
    let palette = theme.palette;
    match variant {
        ButtonVariant::Default => palette.primary_foreground,
        ButtonVariant::Secondary => palette.secondary_foreground,
        ButtonVariant::Destructive => palette.destructive,
        ButtonVariant::Link | ButtonVariant::Soft => palette.primary,
        _ => palette.foreground,
    }
}

fn build_day<'a, Message>(
    root: &RangeCalendar<'a, Message>,
    recipe: &CalendarRecipe,
    date: DateParts,
    month: DateParts,
    today: DateParts,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme = root.theme;
    let position = range_day_position(date, &root.value, None);

    let state = CalendarDayState {
        selected: position.is_selected(),
        today: date == today,
        outside_month: date.month != month.month || date.year != month.year,
        disabled: root.disabled
            || !calendar_date_in_bounds(date, root.min_value, root.max_value)
            || root
                .is_date_disabled
                .as_ref()
                .is_some_and(|matcher| matcher(date)),
        unavailable: root
            .is_date_unavailable
            .as_ref()
            .is_some_and(|matcher| matcher(date)),
    };

    let interactive = state.is_interactive(
        root.readonly,
        root.disabled,
        root.disable_days_outside_month,
    );

    let message = interactive
        .then(|| {
            let min_days = root.min_days;
            let max_days = root.max_days;
            let pick =
                range_calendar_day_pick(root.value, date, root.prevent_deselect, |start, end| {
                    range_days_valid(start, end, min_days, max_days)
                });
            root.on_value_change
                .as_ref()
                .map(|callback| callback(pick.range))
        })
        .flatten();

    let label = date.day.to_string();
    let font = iced_font(theme.font_pack().sans);
    let content: Element<'a, Message> = if state.unavailable {
        Rich::<(), Message>::with_spans(vec![Span::new(label).strikethrough(true)])
            .size(CALENDAR_TEXT_PX)
            .font(font)
            .into()
    } else {
        iced_text(label).size(CALENDAR_TEXT_PX).font(font).into()
    };

    let cell = recipe.cell_size_px;
    let cell_radius = component_radius_px(theme, recipe.cell_radius);
    let palette = theme.palette;

    let mut day = iced_button(
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .width(Length::Fixed(cell))
    .height(Length::Fixed(cell))
    .padding(0)
    .style(move |_, status| range_day_style(&palette, state, position, cell_radius, status));

    if let Some(message) = message {
        day = day.on_press(message);
    }

    day.into()
}

/// Range-calendar day styling: maps `RangeDayPosition` + `CalendarDayState`
/// onto an iced button style with per-corner radii.
fn range_day_style(
    palette: &crate::theme::Palette,
    state: CalendarDayState,
    position: RangeDayPosition,
    cell_radius: f32,
    status: iced_button::Status,
) -> iced_button::Style {
    let hovered = matches!(
        status,
        iced_button::Status::Hovered | iced_button::Status::Pressed
    );

    let mut background: Option<Color> = None;
    let mut text = palette.foreground;
    // Per-corner: [top_left, top_right, bottom_right, bottom_left]
    let mut radii = [cell_radius; 4];

    match position {
        RangeDayPosition::Start => {
            background = Some(palette.primary);
            text = palette.primary_foreground;
            // Rounded on the start (left) side, square on the end (right) side.
            radii[1] = 0.0; // top_right
            radii[2] = 0.0; // bottom_right
        }
        RangeDayPosition::End => {
            background = Some(palette.primary);
            text = palette.primary_foreground;
            // Rounded on the end (right) side, square on the start (left) side.
            radii[0] = 0.0; // top_left
            radii[3] = 0.0; // bottom_left
        }
        RangeDayPosition::StartEnd => {
            background = Some(palette.primary);
            text = palette.primary_foreground;
            // Fully rounded (single-day range).
        }
        RangeDayPosition::Middle => {
            // Accent band, no rounding.
            background = Some(palette.accent);
            text = palette.accent_foreground;
            radii = [0.0; 4];
        }
        RangeDayPosition::None => {
            // Not in range — apply today / hover / outside-month logic.
            if state.today {
                background = Some(palette.accent);
                text = palette.accent_foreground;
            } else if hovered && !state.disabled && !state.unavailable {
                background = Some(Color {
                    a: palette.accent.a * CALENDAR_HOVER_ACCENT_ALPHA,
                    ..palette.accent
                });
                text = palette.accent_foreground;
            }

            if state.outside_month {
                text = if hovered && !state.disabled {
                    palette.accent_foreground
                } else {
                    palette.muted_foreground
                };
            }

            if state.unavailable {
                text = palette.muted_foreground;
            }

            if state.today && state.disabled {
                text = palette.muted_foreground;
            }
        }
    }

    if state.disabled {
        if !position.is_selected() {
            text = palette.muted_foreground;
        }
        text.a *= CALENDAR_DISABLED_OPACITY;
        if let Some(color) = background.as_mut() {
            color.a *= CALENDAR_DISABLED_OPACITY;
        }
    }

    iced_button::Style {
        background: background
            .filter(|color| color.a > f32::EPSILON)
            .map(Background::Color),
        text_color: text,
        border: crate::iced_compat::Border {
            radius: crate::iced_compat::border::Radius {
                top_left: radii[0],
                top_right: radii[1],
                bottom_right: radii[2],
                bottom_left: radii[3],
            },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

fn with_month(date: DateParts, month: u8) -> DateParts {
    let month = month.clamp(1, 12);
    let max_day = days_in_month_of(DateParts {
        year: date.year,
        month,
        day: 1,
    })
    .max(1);
    DateParts {
        year: date.year,
        month,
        day: date.day.min(max_day),
    }
}

fn with_year(date: DateParts, year: i32) -> DateParts {
    let max_day = days_in_month_of(DateParts {
        year,
        month: date.month,
        day: 1,
    })
    .max(1);
    DateParts {
        year,
        month: date.month,
        day: date.day.min(max_day),
    }
}

#[derive(Debug, Clone, Copy)]
struct ChevronIcon {
    color: Color,
    forward: bool,
}

impl<Message> canvas::Program<Message> for ChevronIcon {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &crate::iced_compat::Renderer,
        _theme: &crate::iced_compat::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.width.min(bounds.height);
        if size <= 0.0 {
            return Vec::new();
        }

        let arm = size * 0.125;
        let reach = size * 0.25;
        let stroke_width = size * (2.0 / 24.0);
        let direction = if self.forward { 1.0 } else { -1.0 };

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.translate(Vector::new(bounds.width / 2.0, bounds.height / 2.0));
        frame.stroke(
            &canvas::Path::new(|builder| {
                builder.move_to(Point::new(-arm * direction, -reach));
                builder.line_to(Point::new(arm * direction, 0.0));
                builder.line_to(Point::new(-arm * direction, reach));
            }),
            canvas::Stroke::default()
                .with_width(stroke_width)
                .with_color(self.color)
                .with_line_cap(canvas::LineCap::Round)
                .with_line_join(canvas::LineJoin::Round),
        );

        vec![frame.into_geometry()]
    }
}
