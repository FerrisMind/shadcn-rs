use crate::theme::Theme;
use crate::toggle::toggle;
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
}

pub fn toggle_group(
    ui: &mut Ui,
    props: ToggleGroupProps,
    content: impl FnOnce(&mut Ui, &ToggleGroupContext),
) -> Response {
    let context = ToggleGroupContext {
        variant: props.variant,
        size: props.size,
    };

    ui.horizontal(|ui| {
        // Shadcn uses a small gap for toggle groups
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
    toggle(
        ui,
        theme,
        on,
        label,
        context.variant,
        context.size,
        ui.is_enabled(),
    )
}
