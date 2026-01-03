use crate::theme::Theme;
use crate::tokens::{ControlSize, ToggleVariant};
use egui::{Response, Ui, WidgetText};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleGroupProps {
    pub variant: ToggleVariant,
    pub size: ControlSize,
}

impl Default for ToggleGroupProps {
    fn default() -> Self {
        Self {
            variant: ToggleVariant::Default,
            size: ControlSize::Md,
        }
    }
}

pub struct ToggleGroupContext {
    pub variant: ToggleVariant,
    pub size: ControlSize,
    item_count: std::cell::Cell<usize>,
}

pub fn toggle_group(
    ui: &mut Ui,
    props: ToggleGroupProps,
    content: impl FnOnce(&mut Ui, &ToggleGroupContext),
) -> Response {
    let context = ToggleGroupContext {
        variant: props.variant,
        size: props.size,
        item_count: std::cell::Cell::new(0),
    };

    ui.horizontal(|ui| {
        // Используем отрицательный spacing для слияния границ (как в shadcn/ui)
        ui.spacing_mut().item_spacing = egui::vec2(1.0, 0.0);
        content(ui, &context);
    })
    .response
}

pub fn toggle_group_item(
    ui: &mut Ui,
    theme: &Theme,
    context: &ToggleGroupContext,
    on: &mut bool,
    label: impl Into<WidgetText>,
) -> Response {
    toggle_group_item_with_position(
        ui,
        theme,
        context,
        on,
        label,
        ToggleGroupItemPosition::Middle,
    )
}

pub fn toggle_group_item_last(
    ui: &mut Ui,
    theme: &Theme,
    context: &ToggleGroupContext,
    on: &mut bool,
    label: impl Into<WidgetText>,
) -> Response {
    toggle_group_item_with_position(ui, theme, context, on, label, ToggleGroupItemPosition::Last)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToggleGroupItemPosition {
    Middle,
    Last,
}

fn toggle_group_item_with_position(
    ui: &mut Ui,
    theme: &Theme,
    context: &ToggleGroupContext,
    on: &mut bool,
    label: impl Into<WidgetText>,
    position: ToggleGroupItemPosition,
) -> Response {
    let index = context.item_count.get();
    context.item_count.set(index + 1);

    let base_radius = context.size.rounding();
    let radius_value = base_radius.nw; // Получаем значение радиуса из одного угла

    let custom_radius = match (index, position) {
        (0, ToggleGroupItemPosition::Last) => {
            // Единственный элемент - используем стандартное скругление
            base_radius
        }
        (0, _) => {
            // Первый элемент - скругляем только левые углы
            egui::epaint::CornerRadius {
                nw: radius_value,
                sw: radius_value,
                ne: 0,
                se: 0,
            }
        }
        (_, ToggleGroupItemPosition::Last) => {
            // Последний элемент - скругляем только правые углы
            egui::epaint::CornerRadius {
                nw: 0,
                sw: 0,
                ne: radius_value,
                se: radius_value,
            }
        }
        _ => {
            // Средние элементы - без скруглений
            egui::epaint::CornerRadius::ZERO
        }
    };

    crate::toggle::toggle_with_radius(
        ui,
        theme,
        on,
        label,
        context.variant,
        context.size,
        ui.is_enabled(),
        Some(custom_radius),
    )
}
