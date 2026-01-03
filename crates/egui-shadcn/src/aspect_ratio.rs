use egui::{Align, Layout, Ui, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct AspectRatioProps {
    pub ratio: f32,
}

impl Default for AspectRatioProps {
    fn default() -> Self {
        Self { ratio: 1.0 }
    }
}

impl AspectRatioProps {
    pub fn new(ratio: f32) -> Self {
        Self { ratio }
    }
}

pub fn aspect_ratio<R>(
    ui: &mut Ui,
    props: AspectRatioProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    let ratio = if props.ratio.is_finite() && props.ratio > 0.0 {
        props.ratio
    } else {
        1.0
    };

    let available = ui.available_size();
    let mut width = available.x;
    if !width.is_finite() || width <= 0.0 {
        width = ui.spacing().interact_size.x.max(1.0);
    }
    let mut height = width / ratio;

    if available.y.is_finite() && available.y > 0.0 && height > available.y {
        height = available.y;
        width = height * ratio;
    }

    let size = Vec2::new(width, height);
    ui.allocate_ui_with_layout(size, Layout::top_down(Align::Min), |content_ui| {
        content_ui.set_min_size(size);
        add_contents(content_ui)
    })
}
