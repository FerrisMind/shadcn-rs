use crate::theme::Theme;
use crate::tokens::mix;
use egui::{Color32, Frame, Margin, RichText, Stroke, Ui, Vec2};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KbdSize {
    Size1,
    #[default]
    Size2,
    Size3,
    Size4,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl KbdSize {
    fn font_size(self) -> f32 {
        match self {
            KbdSize::Size1 => 10.0,
            KbdSize::Size2 => 11.0,
            KbdSize::Size3 => 12.0,
            KbdSize::Size4 => 13.0,
            KbdSize::Five => 14.0,
            KbdSize::Six => 16.0,
            KbdSize::Seven => 18.0,
            KbdSize::Eight => 20.0,
            KbdSize::Nine => 24.0,
        }
    }

    fn padding(self) -> Vec2 {
        match self {
            KbdSize::Size1 => Vec2::new(4.0, 1.0),
            KbdSize::Size2 => Vec2::new(6.0, 2.0),
            KbdSize::Size3 => Vec2::new(8.0, 3.0),
            KbdSize::Size4 => Vec2::new(10.0, 4.0),
            KbdSize::Five => Vec2::new(12.0, 5.0),
            KbdSize::Six => Vec2::new(14.0, 6.0),
            KbdSize::Seven => Vec2::new(16.0, 7.0),
            KbdSize::Eight => Vec2::new(18.0, 8.0),
            KbdSize::Nine => Vec2::new(20.0, 9.0),
        }
    }

    fn radius(self, theme: &Theme) -> f32 {
        match self {
            KbdSize::Size1 | KbdSize::Size2 | KbdSize::Size3 => theme.radius.r2 * 0.7,
            KbdSize::Size4 | KbdSize::Five => theme.radius.r2,
            KbdSize::Six | KbdSize::Seven => theme.radius.r3,
            _ => theme.radius.r4,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KbdProps {
    pub size: KbdSize,
    pub background: Option<Color32>,
    pub color: Option<Color32>,
    pub border_color: Option<Color32>,
    pub shadow: bool,
}

impl Default for KbdProps {
    fn default() -> Self {
        Self {
            size: KbdSize::default(),
            background: None,
            color: None,
            border_color: None,
            shadow: true,
        }
    }
}

impl KbdProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: KbdSize) -> Self {
        self.size = size;
        self
    }

    pub fn background(mut self, bg: Color32) -> Self {
        self.background = Some(bg);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }
}

pub fn kbd(ui: &mut Ui, theme: &Theme, label: &str, props: KbdProps) {
    let background = props
        .background
        .unwrap_or_else(|| mix(theme.palette.muted, theme.palette.background, 0.6));
    let text_color = props.color.unwrap_or(theme.palette.muted_foreground);
    let border_color = props.border_color.unwrap_or(theme.palette.border);
    let radius = props.size.radius(theme);
    let padding = props.size.padding();

    let frame = Frame::NONE
        .fill(background)
        .stroke(Stroke::new(1.0_f32, border_color))
        .corner_radius(radius)
        .inner_margin(Margin::symmetric(padding.x as i8, padding.y as i8));

    frame.show(ui, |ui| {
        ui.add(
            egui::Label::new(
                RichText::new(label)
                    .size(props.size.font_size())
                    .color(text_color)
                    .monospace(),
            )
            .selectable(false),
        );
    });
}

pub fn kbd_group(
    ui: &mut Ui,
    theme: &Theme,
    labels: &[&str],
    props: KbdProps,
    separator: Option<&str>,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for (i, label) in labels.iter().enumerate() {
            if i > 0
                && let Some(sep) = separator
            {
                ui.add(
                    egui::Label::new(
                        RichText::new(sep)
                            .size(12.0)
                            .color(theme.palette.muted_foreground),
                    )
                    .selectable(false),
                );
            }
            kbd(ui, theme, label, props);
        }
    });
}

pub fn kbd_shortcut(ui: &mut Ui, theme: &Theme, labels: &[&str], props: KbdProps) {
    kbd_group(ui, theme, labels, props, Some("+"));
}
