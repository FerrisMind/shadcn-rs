//! Composition rendering for [`super::PhoneInput`].

use shadcn_common::{
    CountryCode, PhoneCountry, apply_country_change, apply_input_change, auto_placeholder,
    is_phone_valid, parse_phone_input, phone_countries, phone_country, sort_countries,
};
use twill_core::prelude::{Padding, Spacing};

use crate::components::button::{Button, ButtonSize, ButtonVariant, CornerFlatten};
use crate::components::command::{Command, CommandEmpty, CommandItem};
use crate::components::input::Input;
use crate::components::popover::{Popover, PopoverAlign, PopoverSide};
use crate::fonts::iced_font;
use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::{container, row, text};
use crate::iced_compat::{Background, Border, Color, Element, Length};
use crate::theme::Theme;

use super::icon;
use super::{PhoneInput, PhoneInputChange};

/// Builds the joined country-selector + phone field element.
pub(super) fn build<'a, Message>(input: PhoneInput<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let PhoneInput {
        theme,
        value,
        country,
        default_country,
        placeholder,
        name: _,
        options,
        disabled,
        readonly,
        required: _,
        invalid,
        width,
        open,
        query,
        order,
        on_change,
        on_open_change,
        on_query_change,
        on_submit,
    } = input;

    let recipe = theme.style.phone_input();
    let resolved_country = country.or(default_country);
    let selected = resolved_country.and_then(phone_country);

    let parsed = parse_phone_input(value, resolved_country);
    let show_invalid = invalid
        .unwrap_or_else(|| !value.trim().is_empty() && !parsed.valid && !is_phone_valid(value));

    let resolved_placeholder = placeholder
        .map(str::to_owned)
        .or_else(|| {
            if options.auto_placeholder {
                auto_placeholder(resolved_country)
            } else {
                None
            }
        })
        .unwrap_or_default();

    let mut countries = phone_countries();
    sort_countries(&mut countries, Some(order));

    let country_trigger = build_trigger(theme, selected, disabled, &recipe);

    let mut command = Command::new(theme)
        .query(query)
        .placeholder("Search...")
        .width(Length::Fixed(recipe.popover_width_px))
        .max_height(recipe.list_height_px)
        .show_border(false)
        .show_shadow(false)
        .empty(CommandEmpty::new("No country found."));

    if let Some(on_query_change) = on_query_change {
        command = command.on_query_change(on_query_change);
    }

    for country_row in &countries {
        let item = CommandItem::new(
            country_row.iso2,
            format!("{} {}", country_row.flag_emoji(), country_row.name),
        )
        .shortcut(country_row.dial_label())
        .keywords([
            country_row.iso2.as_str().to_owned(),
            country_row.dial_code.to_string(),
            country_row.dial_label(),
        ])
        .checked(resolved_country == Some(country_row.iso2))
        .disabled(disabled || readonly);
        command = command.item(item);
    }

    if let Some(on_change) = on_change.clone() {
        let current_value = value.to_owned();
        let previous = resolved_country;
        command = command.on_select(move |iso: CountryCode| {
            let detailed = apply_country_change(&current_value, previous, iso, options);
            on_change(PhoneInputChange::from_detailed(detailed, Some(false)))
        });
    }

    let mut popover = Popover::new(country_trigger, command, theme)
        .side(PopoverSide::Bottom)
        .align(PopoverAlign::Start)
        .width(recipe.popover_width_px)
        .content_padding(0.0)
        .disabled(disabled || readonly);

    if let Some(open) = open {
        popover = popover.open(open);
    }
    if let Some(on_open_change) = on_open_change {
        popover = popover.on_open_change(on_open_change);
    }

    // Keep pack Input radii/fills from theme; only flatten the joined edge
    // (`rounded-l-none`), matching Form's "compose pack children" rule.
    let mut field = Input::new(theme)
        .value(value)
        .placeholder(resolved_placeholder)
        .width(Length::Fill)
        .disabled(disabled || readonly)
        .invalid(show_invalid)
        .style_override(|mut style, _status| {
            style.border.radius.top_left = 0.0;
            style.border.radius.bottom_left = 0.0;
            style
        });

    if let Some(on_submit) = on_submit {
        field = field.on_submit(on_submit);
    }

    if let Some(on_change) = on_change {
        field = field.on_input(move |raw| {
            let detailed = apply_input_change(&raw, resolved_country, options);
            let mut change = PhoneInputChange::from_detailed(detailed, None);
            if !change.valid {
                change.value = raw;
            }
            on_change(change)
        });
    }

    row![popover, field]
        .spacing(-recipe.joint_overlap_px)
        .align_y(Vertical::Center)
        .width(width)
        .into()
}

fn build_trigger<'a, Message>(
    theme: &'a Theme,
    selected: Option<PhoneCountry>,
    disabled: bool,
    recipe: &shadcn_common::PhoneInputRecipe,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let flag_bg = with_alpha(theme.palette.foreground, 0.2);
    let flag_label = selected.map(PhoneCountry::flag_emoji).unwrap_or_default();
    let flag = container(
        text(flag_label)
            .size(11)
            .font(iced_font(theme.font_pack().sans)),
    )
    .width(Length::Fixed(recipe.flag_width_px))
    .height(Length::Fixed(recipe.flag_height_px))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_| container::Style {
        background: Some(Background::Color(flag_bg)),
        border: Border {
            radius: 2.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    });

    let mut content = row![flag]
        .spacing(recipe.trigger_gap_px)
        .align_y(Vertical::Center);

    if !disabled {
        let chevron_color = with_alpha(theme.palette.muted_foreground, 0.5);
        content = content.push(icon::chevrons_up_down(
            recipe.chevron_size_px,
            chevron_color,
        ));
    }

    // Height / radius / fill come from the pack Button recipe on `theme`
    // (Rhea → Rhea outline control). Only flatten the joined trailing edge.
    Button::new(content, theme)
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Default)
        .disabled(disabled)
        .width(Length::Shrink)
        .flatten_corners(CornerFlatten {
            top_left: false,
            top_right: true,
            bottom_right: true,
            bottom_left: false,
        })
        .padding(Padding::individual(
            Spacing::S0,
            Spacing::S3,
            Spacing::S0,
            Spacing::S3,
        ))
        .expect("S0/S3 padding is finite")
        .into()
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha,
        ..color
    }
}
