use crate::button::{Button, ButtonRadius};
use crate::theme::Theme;
use egui::{Align, CornerRadius, Layout, Response, Ui, vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct ButtonGroup<'a> {
    orientation: ButtonGroupOrientation,
    radius: u8, // Base radius value
    buttons: Vec<Button<'a>>,
}

impl<'a> ButtonGroup<'a> {
    pub fn new(buttons: Vec<Button<'a>>) -> Self {
        Self {
            orientation: ButtonGroupOrientation::Horizontal,
            radius: 8, // Default Medium
            buttons,
        }
    }

    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn radius(mut self, radius: u8) -> Self {
        self.radius = radius;
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        let count = self.buttons.len();
        let layout = match self.orientation {
            ButtonGroupOrientation::Horizontal => Layout::left_to_right(Align::Center),
            ButtonGroupOrientation::Vertical => Layout::top_down(Align::Center),
        };

        ui.with_layout(layout, |ui| {
            // Overlap borders by 1px
            let spacing = match self.orientation {
                ButtonGroupOrientation::Horizontal => vec2(-1.0, 0.0),
                ButtonGroupOrientation::Vertical => vec2(0.0, -1.0),
            };
            ui.spacing_mut().item_spacing = spacing;

            for (i, button) in self.buttons.into_iter().enumerate() {
                let r = self.radius;
                let corner_radius = if count == 1 {
                    CornerRadius::same(r)
                } else {
                    match self.orientation {
                        ButtonGroupOrientation::Horizontal => {
                            if i == 0 {
                                CornerRadius {
                                    nw: r,
                                    sw: r,
                                    ne: 0,
                                    se: 0,
                                }
                            } else if i == count - 1 {
                                CornerRadius {
                                    nw: 0,
                                    sw: 0,
                                    ne: r,
                                    se: r,
                                }
                            } else {
                                CornerRadius::same(0)
                            }
                        }
                        ButtonGroupOrientation::Vertical => {
                            if i == 0 {
                                CornerRadius {
                                    nw: r,
                                    ne: r,
                                    sw: 0,
                                    se: 0,
                                }
                            } else if i == count - 1 {
                                CornerRadius {
                                    nw: 0,
                                    ne: 0,
                                    sw: r,
                                    se: r,
                                }
                            } else {
                                CornerRadius::same(0)
                            }
                        }
                    }
                };

                button
                    .radius(ButtonRadius::Custom(corner_radius))
                    .show(ui, theme);
            }
        })
        .response
    }
}

pub fn button_group(ui: &mut Ui, theme: &Theme, buttons: Vec<Button<'_>>) -> Response {
    ButtonGroup::new(buttons).show(ui, theme)
}
