use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Space, column, row, stack};
use iced::{Color, Element, Length, Padding};

use crate::button::{
    ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, ShadcnButton, button, button_content,
};
use crate::card::{CardProps, CardSize, CardVariant, card};
use crate::progress::{
    ProgressOrientation, ProgressProps, ProgressSize, ProgressVariant, progress,
};
use crate::theme::Theme;
use crate::tokens::AccentColor;
use crate::tokens::{accent_color, accent_foreground, accent_low, accent_text};
use crate::typography::{TextAs, TextProps, TextSize, TextWeight, text as typography_text};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StepperOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepperItemState {
    Active,
    Completed,
    Inactive,
}

#[derive(Clone, Copy, Debug)]
pub struct StepperProps {
    pub orientation: StepperOrientation,
    pub item_spacing: f32,
    pub content_spacing: f32,
    pub indicator_size: f32,
    pub indicator_padding_x: f32,
    pub separator_thickness: f32,
}

impl Default for StepperProps {
    fn default() -> Self {
        Self {
            orientation: StepperOrientation::Horizontal,
            item_spacing: 12.0,
            content_spacing: 6.0,
            indicator_size: 32.0,
            indicator_padding_x: 0.0,
            separator_thickness: 1.0,
        }
    }
}

impl StepperProps {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn orientation(mut self, orientation: StepperOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    pub fn item_spacing(mut self, item_spacing: f32) -> Self {
        self.item_spacing = item_spacing.max(0.0);
        self
    }

    #[must_use]
    pub fn content_spacing(mut self, content_spacing: f32) -> Self {
        self.content_spacing = content_spacing.max(0.0);
        self
    }

    #[must_use]
    pub fn indicator_size(mut self, indicator_size: f32) -> Self {
        self.indicator_size = indicator_size.max(8.0);
        self
    }

    #[must_use]
    pub fn indicator_padding_x(mut self, indicator_padding_x: f32) -> Self {
        self.indicator_padding_x = indicator_padding_x.max(0.0);
        self
    }

    #[must_use]
    pub fn separator_thickness(mut self, separator_thickness: f32) -> Self {
        self.separator_thickness = separator_thickness.max(1.0);
        self
    }
}

pub struct StepperTrigger<'a, Message> {
    indicator: Element<'a, Message>,
    title: Element<'a, Message>,
    description: Option<Element<'a, Message>>,
}

impl<'a, Message> StepperTrigger<'a, Message> {
    #[must_use]
    pub fn new(
        indicator: impl Into<Element<'a, Message>>,
        title: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            indicator: indicator.into(),
            title: title.into(),
            description: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<Element<'a, Message>>) -> Self {
        self.description = Some(description.into());
        self
    }
}

pub struct StepperItem<'a, Message> {
    _id: String,
    trigger: StepperTrigger<'a, Message>,
    separator: Option<Element<'a, Message>>,
    disabled: bool,
}

impl<'a, Message> StepperItem<'a, Message> {
    #[must_use]
    pub fn new(id: impl Into<String>, trigger: StepperTrigger<'a, Message>) -> Self {
        Self {
            _id: id.into(),
            trigger,
            separator: None,
            disabled: false,
        }
    }

    #[must_use]
    pub fn separator(mut self, separator: impl Into<Element<'a, Message>>) -> Self {
        self.separator = Some(separator.into());
        self
    }

    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
struct StepperItemMeta {
    disabled: bool,
}

fn resolve_step(items: &[StepperItemMeta], step: usize) -> usize {
    if items.is_empty() {
        return 1;
    }

    let requested = step.clamp(1, items.len());
    if !items[requested - 1].disabled {
        return requested;
    }

    items
        .iter()
        .position(|item| !item.disabled)
        .map(|index| index + 1)
        .unwrap_or(requested)
}

fn step_state(step: usize, current_step: usize) -> StepperItemState {
    if step < current_step {
        StepperItemState::Completed
    } else if step == current_step {
        StepperItemState::Active
    } else {
        StepperItemState::Inactive
    }
}

fn can_increment_step(current_step: usize, total_steps: usize) -> bool {
    total_steps > 0 && current_step < total_steps
}

fn can_decrement_step(current_step: usize) -> bool {
    current_step > 1
}

fn is_progress_filled(item_state: StepperItemState) -> bool {
    matches!(item_state, StepperItemState::Completed)
}

fn is_indicator_accented(item_state: StepperItemState) -> bool {
    is_progress_filled(item_state) || matches!(item_state, StepperItemState::Active)
}

fn layout_card_props() -> CardProps {
    CardProps::new()
        .variant(CardVariant::Ghost)
        .size(CardSize::Size1)
        .show_shadow(false)
        .padding(0.0)
        .background(Color::TRANSPARENT)
        .border_color(Color::TRANSPARENT)
        .radius(0.0)
}

fn layout_card<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &Theme,
) -> iced::widget::container::Container<'a, Message> {
    card(content, layout_card_props(), theme)
}

fn separator_for_state<'a, Message: 'a>(
    item_state: StepperItemState,
    props: StepperProps,
    theme: &Theme,
) -> Element<'a, Message> {
    progress(
        ProgressProps::new()
            .size(ProgressSize::Size1)
            .variant(ProgressVariant::Soft)
            .orientation(match props.orientation {
                StepperOrientation::Horizontal => ProgressOrientation::Horizontal,
                StepperOrientation::Vertical => ProgressOrientation::Vertical,
            })
            .color(AccentColor::Gray)
            .high_contrast(false)
            .radius(ButtonRadius::None)
            .value(if is_progress_filled(item_state) {
                100.0
            } else {
                0.0
            }),
        theme,
    )
    .into()
}

fn layered_separator<'a, Message: 'a>(
    completed: bool,
    props: StepperProps,
    theme: &Theme,
) -> Element<'a, Message> {
    let base = separator_for_state(StepperItemState::Inactive, props, theme);
    if completed {
        stack(vec![
            base,
            separator_for_state(StepperItemState::Completed, props, theme),
        ])
        .into()
    } else {
        base
    }
}

fn indicator_shell<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    item_state: StepperItemState,
    props: StepperProps,
    theme: &Theme,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let width = props.indicator_size + (props.indicator_padding_x * 2.0);
    let is_accented = is_indicator_accented(item_state);
    let background = if is_accented {
        accent_color(&theme.palette, AccentColor::Gray)
    } else {
        accent_low(&theme.palette, AccentColor::Gray)
    };
    let foreground = if is_accented {
        accent_foreground(&theme.palette, AccentColor::Gray)
    } else {
        accent_text(&theme.palette, AccentColor::Gray)
    };

    let indicator_props = ButtonProps::new()
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Size1)
        .color(AccentColor::Gray)
        .radius(ButtonRadius::Full);
    let centered_content = card(
        content,
        layout_card_props()
            .background(background)
            .text_color(foreground)
            .radius(props.indicator_size),
        theme,
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill);
    button_content(centered_content, on_press, indicator_props, theme)
        .width(Length::Fixed(width))
        .height(Length::Fixed(props.indicator_size))
        .padding(0)
        .into()
}

fn render_trigger_content<'a, Message: Clone + 'a>(
    trigger: StepperTrigger<'a, Message>,
    item_state: StepperItemState,
    props: StepperProps,
    theme: &Theme,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let description = trigger.description.map(|description| {
        let muted = theme.palette.muted_foreground;
        layout_card(description, theme).style(move |_t| iced::widget::container::Style {
            text_color: Some(muted),
            ..Default::default()
        })
    });

    let text_column = if let Some(description) = description {
        column![trigger.title, description]
            .spacing(props.content_spacing)
            .align_x(match props.orientation {
                StepperOrientation::Horizontal => Horizontal::Center,
                StepperOrientation::Vertical => Horizontal::Left,
            })
    } else {
        column![trigger.title]
            .spacing(props.content_spacing)
            .align_x(match props.orientation {
                StepperOrientation::Horizontal => Horizontal::Center,
                StepperOrientation::Vertical => Horizontal::Left,
            })
    };

    let indicator = indicator_shell(trigger.indicator, item_state, props, theme, on_press);

    match props.orientation {
        StepperOrientation::Horizontal => column![indicator, text_column]
            .spacing(props.content_spacing)
            .align_x(Horizontal::Center)
            .into(),
        StepperOrientation::Vertical => row![indicator, text_column]
            .spacing(props.content_spacing)
            .align_y(Vertical::Center)
            .into(),
    }
}

fn render_item<'a, Message: Clone + 'a>(
    item: StepperItem<'a, Message>,
    current_step: usize,
    props: StepperProps,
    theme: &Theme,
    on_press: Option<Message>,
    index: usize,
    total_items: usize,
) -> Element<'a, Message> {
    let item_state = step_state(index + 1, current_step);
    let is_last = index + 1 == total_items;
    let StepperItem {
        trigger, separator, ..
    } = item;

    let separator = if is_last {
        None
    } else {
        Some(separator.unwrap_or_else(|| separator_for_state(item_state, props, theme)))
    };

    let item_body: Element<'a, Message> = match props.orientation {
        StepperOrientation::Horizontal => {
            let trigger_content =
                render_trigger_content(trigger, item_state, props, theme, on_press);

            let item_width = Length::FillPortion(1);
            let trigger_layer: Element<'a, Message> = layout_card(trigger_content, theme)
                .width(Length::Fill)
                .height(Length::Shrink)
                .align_x(Horizontal::Center)
                .into();

            if let Some(separator) = separator {
                let indicator_width = props.indicator_size + (props.indicator_padding_x * 2.0);
                let separator_offset = (props.indicator_size - props.separator_thickness) / 2.0;
                let separator_layer: Element<'a, Message> = layout_card(separator, theme)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(Vertical::Top)
                    .padding(Padding {
                        top: separator_offset.max(0.0),
                        right: 0.0,
                        bottom: 0.0,
                        left: (indicator_width + props.item_spacing).max(0.0),
                    })
                    .into();

                layout_card(stack(vec![trigger_layer, separator_layer]), theme)
                    .width(item_width)
                    .into()
            } else {
                layout_card(trigger_layer, theme).width(item_width).into()
            }
        }
        StepperOrientation::Vertical => {
            let StepperTrigger {
                indicator,
                title,
                description,
            } = trigger;
            let description = description.map(|description| {
                let muted = theme.palette.muted_foreground;
                layout_card(description, theme).style(move |_t| iced::widget::container::Style {
                    text_color: Some(muted),
                    ..Default::default()
                })
            });

            let text_column = if let Some(description) = description {
                column![title, description]
                    .spacing(props.content_spacing)
                    .align_x(Horizontal::Left)
            } else {
                column![title]
                    .spacing(props.content_spacing)
                    .align_x(Horizontal::Left)
            };

            let indicator_column_width = props.indicator_size + (props.indicator_padding_x * 2.0);
            let indicator = indicator_shell(indicator, item_state, props, theme, on_press);
            let mut indicator_column = column![indicator]
                .spacing(props.item_spacing)
                .align_x(Horizontal::Center)
                .width(Length::Fixed(indicator_column_width));

            if let Some(separator) = separator {
                let separator_height = props.indicator_size.max(props.item_spacing);
                let separator = layout_card(separator, theme)
                    .width(Length::Fixed(props.separator_thickness.max(1.0)))
                    .height(Length::Fixed(separator_height))
                    .center_x(Length::Fill);
                indicator_column = indicator_column.push(separator);
            }

            layout_card(
                row![indicator_column, text_column]
                    .spacing(props.content_spacing)
                    .align_y(Vertical::Top)
                    .width(Length::Fill),
                theme,
            )
            .width(Length::Fill)
            .into()
        }
    };
    item_body
}

#[must_use]
pub fn stepper_indicator<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    content.into()
}

#[must_use]
pub fn stepper_title<'a, Message: Clone + 'a>(
    content: impl Into<String>,
    theme: &Theme,
) -> Element<'a, Message> {
    typography_text(
        content.into(),
        TextProps::new()
            .as_tag(TextAs::Label)
            .size(TextSize::Size2)
            .weight(TextWeight::Medium)
            .high_contrast(true),
        theme,
    )
    .into()
}

#[must_use]
pub fn stepper_description<'a, Message: Clone + 'a>(
    content: impl Into<String>,
    theme: &Theme,
) -> Element<'a, Message> {
    typography_text(
        content.into(),
        TextProps::new().as_tag(TextAs::P).size(TextSize::Size1),
        theme,
    )
    .into()
}

#[must_use]
pub fn stepper_trigger<'a, Message: Clone + 'a>(
    indicator: impl Into<Element<'a, Message>>,
    title: impl Into<Element<'a, Message>>,
) -> StepperTrigger<'a, Message> {
    StepperTrigger::new(indicator, title)
}

#[must_use]
pub fn stepper_item<'a, Message: Clone + 'a>(
    id: impl Into<String>,
    trigger: StepperTrigger<'a, Message>,
) -> StepperItem<'a, Message> {
    StepperItem::new(id, trigger)
}

#[must_use]
pub fn stepper_nav<'a, Message: Clone + 'a, F>(
    items: Vec<StepperItem<'a, Message>>,
    step: usize,
    on_step_change: Option<F>,
    props: StepperProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(usize) -> Message + 'a,
{
    let on_step_change = on_step_change.as_ref();
    let item_meta: Vec<StepperItemMeta> = items
        .iter()
        .map(|item| StepperItemMeta {
            disabled: item.disabled,
        })
        .collect();

    if items.is_empty() {
        return layout_card(
            Space::new().width(Length::Shrink).height(Length::Shrink),
            theme,
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .into();
    }

    let resolved_step = resolve_step(&item_meta, step);
    let nav_content: Element<'a, Message> = match props.orientation {
        StepperOrientation::Horizontal => {
            let total_items = items.len();
            let mut item_columns: Vec<Element<'a, Message>> = Vec::with_capacity(total_items);

            for (index, item) in items.into_iter().enumerate() {
                let item_state = step_state(index + 1, resolved_step);
                let is_first = index == 0;
                let is_last = index + 1 == total_items;
                let on_press = if item.disabled {
                    None
                } else {
                    on_step_change.map(|callback| callback(index + 1))
                };
                let StepperItem {
                    trigger, separator, ..
                } = item;
                let StepperTrigger {
                    indicator,
                    title,
                    description,
                } = trigger;

                let indicator = indicator_shell(indicator, item_state, props, theme, on_press);

                let left_segment: Element<'a, Message> = if is_first {
                    Space::new()
                        .width(Length::Fill)
                        .height(Length::Fixed(props.separator_thickness.max(1.0)))
                        .into()
                } else {
                    layered_separator(index < resolved_step, props, theme)
                };

                let right_segment: Element<'a, Message> = if is_last {
                    Space::new()
                        .width(Length::Fill)
                        .height(Length::Fixed(props.separator_thickness.max(1.0)))
                        .into()
                } else {
                    let completed = index + 1 < resolved_step;
                    separator.unwrap_or_else(|| layered_separator(completed, props, theme))
                };

                let rail_item: Element<'a, Message> = row![left_segment, indicator, right_segment]
                    .spacing(4.0)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fixed(props.indicator_size))
                    .into();

                let labels: Element<'a, Message> = if let Some(description) = description {
                    let muted = theme.palette.muted_foreground;
                    column![
                        title,
                        layout_card(description, theme).style(move |_t| {
                            iced::widget::container::Style {
                                text_color: Some(muted),
                                ..Default::default()
                            }
                        })
                    ]
                    .spacing(props.content_spacing)
                    .align_x(Horizontal::Center)
                    .into()
                } else {
                    column![title]
                        .spacing(props.content_spacing)
                        .align_x(Horizontal::Center)
                        .into()
                };

                item_columns.push(
                    layout_card(
                        column![rail_item, labels]
                            .spacing(props.content_spacing)
                            .align_x(Horizontal::Center)
                            .width(Length::Fill),
                        theme,
                    )
                    .width(Length::FillPortion(1))
                    .into(),
                );
            }

            row(item_columns)
                .spacing(0)
                .align_y(Vertical::Top)
                .width(Length::Fill)
                .into()
        }
        StepperOrientation::Vertical => {
            let mut rendered_items: Vec<Element<'a, Message>> = Vec::with_capacity(items.len());
            let total_items = items.len();

            for (index, item) in items.into_iter().enumerate() {
                let on_press = if item.disabled {
                    None
                } else {
                    on_step_change.map(|callback| callback(index + 1))
                };
                rendered_items.push(render_item(
                    item,
                    resolved_step,
                    props,
                    theme,
                    on_press,
                    index,
                    total_items,
                ));
            }

            column(rendered_items)
                .spacing(props.item_spacing)
                .align_x(Horizontal::Left)
                .width(Length::Fill)
                .into()
        }
    };

    nav_content
}

#[must_use]
pub fn stepper<'a, Message: Clone + 'a, F>(
    items: Vec<StepperItem<'a, Message>>,
    step: usize,
    on_step_change: Option<F>,
    props: StepperProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(usize) -> Message + 'a,
{
    stepper_nav(items, step, on_step_change, props, theme)
}

#[must_use]
pub fn stepper_next<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
    current_step: usize,
    total_steps: usize,
    props: ButtonProps,
    theme: &Theme,
) -> ShadcnButton<'a, Message> {
    let is_disabled =
        props.disabled || on_press.is_none() || !can_increment_step(current_step, total_steps);
    button(
        label.into(),
        on_press.filter(|_| !is_disabled),
        props.disabled(is_disabled),
        theme,
    )
}

#[must_use]
pub fn stepper_previous<'a, Message: Clone + 'a>(
    label: impl Into<String>,
    on_press: Option<Message>,
    current_step: usize,
    _total_steps: usize,
    props: ButtonProps,
    theme: &Theme,
) -> ShadcnButton<'a, Message> {
    let is_disabled = props.disabled || on_press.is_none() || !can_decrement_step(current_step);
    button(
        label.into(),
        on_press.filter(|_| !is_disabled),
        props.disabled(is_disabled),
        theme,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_step_prefers_enabled_match() {
        let items = vec![
            StepperItemMeta { disabled: true },
            StepperItemMeta { disabled: false },
            StepperItemMeta { disabled: false },
        ];

        assert_eq!(resolve_step(&items, 1), 2);
        assert_eq!(resolve_step(&items, 2), 2);
    }

    #[test]
    fn step_state_classifies_positions() {
        assert_eq!(step_state(1, 2), StepperItemState::Completed);
        assert_eq!(step_state(2, 2), StepperItemState::Active);
        assert_eq!(step_state(3, 2), StepperItemState::Inactive);
    }

    #[test]
    fn navigation_boundaries_respected() {
        assert!(!can_increment_step(3, 3));
        assert!(can_increment_step(2, 3));
        assert!(!can_increment_step(1, 0));

        assert!(!can_decrement_step(1));
        assert!(can_decrement_step(2));
    }
}
