//! Iced text rendering for form-specific typography tokens.
//!
//! Form descriptions/errors/legends intentionally differ from the Field family:
//! form validation messages use `font-medium`, while Field errors stay
//! `font-normal` (matching shadcn-svelte).

use crate::components::field::FieldErrorItem;
use crate::fonts::iced_font;
use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::text::{Fragment, LineHeight};
use crate::iced_compat::widget::{column, container, row, text as iced_text};
use crate::iced_compat::{Element, Length, Padding};
use crate::recipes::iced_font_weight;
use crate::theme::Theme;
use shadcn_common::TypeRecipe;

/// Builds muted form description text from [`FormRecipe::description`].
pub(super) fn build_description<'a, Message>(
    content: Fragment<'a>,
    theme: &'a Theme,
    width: Length,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let recipe = theme.style.form().description;
    build_typed_text(
        content,
        theme,
        &recipe,
        color.unwrap_or(theme.palette.muted_foreground),
        width,
    )
}

/// Builds a form legend from [`FormRecipe::legend`].
pub(super) fn build_legend<'a, Message>(
    content: Fragment<'a>,
    theme: &'a Theme,
    width: Length,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let recipe = theme.style.form().legend;
    build_typed_text(
        content,
        theme,
        &recipe,
        color.unwrap_or(theme.palette.foreground),
        width,
    )
}

/// Builds validation messages from [`FormRecipe::error`].
pub(super) fn build_errors<'a, Message>(
    errors: Vec<FieldErrorItem>,
    theme: &'a Theme,
    width: Length,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let recipe = theme.style.form().error;
    let messages: Vec<String> = errors
        .iter()
        .filter_map(FieldErrorItem::message)
        .filter(|message| !message.is_empty())
        .map(str::to_owned)
        .collect();

    match messages.as_slice() {
        [] => crate::iced_compat::widget::Space::new().into(),
        [message] => build_typed_text(
            message.clone().into(),
            theme,
            &recipe,
            theme.palette.destructive,
            width,
        ),
        _ => {
            let items = messages.into_iter().map(|message| {
                row![
                    typed_plain::<Message>(
                        theme,
                        &recipe,
                        "•".into(),
                        theme.palette.destructive,
                        8.0
                    ),
                    typed_plain::<Message>(
                        theme,
                        &recipe,
                        message.into(),
                        theme.palette.destructive,
                        Length::Fill,
                    ),
                ]
                .spacing(4)
                .align_y(Vertical::Top)
                .width(Length::Fill)
                .into()
            });

            container(column(items).spacing(4).width(Length::Fill))
                .padding(Padding {
                    left: 16.0,
                    ..Padding::ZERO
                })
                .width(width)
                .into()
        }
    }
}

fn build_typed_text<'a, Message>(
    content: Fragment<'a>,
    theme: &'a Theme,
    recipe: &TypeRecipe,
    color: crate::iced_compat::Color,
    width: Length,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(recipe.weight);
    iced_text(content)
        .size(recipe.size_px)
        .line_height(LineHeight::Absolute(recipe.line_height_px.into()))
        .font(font)
        .color(color)
        .width(width)
        .into()
}

fn typed_plain<'a, Message>(
    theme: &'a Theme,
    recipe: &TypeRecipe,
    content: Fragment<'a>,
    color: crate::iced_compat::Color,
    width: impl Into<Length>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    build_typed_text(content, theme, recipe, color, width.into())
}
