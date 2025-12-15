//! Accessible label component aligned with Radix Label API plus shadcn variants.
//!
//! # Example
//! ```rust
//! use egui_shadcn::{label, LabelVariant, Theme};
//!
//! fn ui(ui: &mut egui::Ui, theme: &Theme) {
//!     label(ui, theme, "Email", LabelVariant::Default, false, None);
//! }
//! ```

use crate::theme::Theme;
use crate::tokens::{ColorPalette, ControlSize, mix};
use egui::{Color32, FontId, Id, Response, Sense, TextStyle, Ui, WidgetText};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelVariant {
    #[default]
    Default,

    Secondary,

    Muted,

    Destructive,
}

#[derive(Clone, Debug)]
pub struct LabelStyle {
    pub text: Color32,

    pub description: Color32,

    pub required: Color32,

    pub disabled_opacity: f32,

    pub font: FontId,

    pub description_font: FontId,
}

impl LabelStyle {
    pub fn from_palette(palette: &ColorPalette, variant: LabelVariant, size: ControlSize) -> Self {
        let font = size.font();
        let text = match variant {
            LabelVariant::Default => palette.foreground,
            LabelVariant::Secondary => palette.secondary_foreground,
            LabelVariant::Muted => palette.muted_foreground,
            LabelVariant::Destructive => palette.destructive,
        };
        let description = mix(palette.muted_foreground, palette.foreground, 0.35);
        let required = palette.destructive;

        let description_font = {
            let mut desc_font = font.clone();
            desc_font.size = (font.size * 0.9).max(10.0);
            desc_font
        };

        Self {
            text,
            description,
            required,
            disabled_opacity: 0.55,
            font,
            description_font,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.text = Color32::from_rgba_unmultiplied(
            self.text.r(),
            self.text.g(),
            self.text.b(),
            (self.text.a() as f32 * self.disabled_opacity) as u8,
        );
        self.description = Color32::from_rgba_unmultiplied(
            self.description.r(),
            self.description.g(),
            self.description.b(),
            (self.description.a() as f32 * self.disabled_opacity) as u8,
        );
        self
    }
}

#[derive(Clone, Debug)]
pub struct LabelProps {
    pub text: WidgetText,

    pub html_for: Option<String>,
    pub for_id: Option<Id>,

    pub interactive: bool,

    pub size: ControlSize,

    pub variant: LabelVariant,

    pub disabled: bool,

    pub required: bool,

    pub description: Option<WidgetText>,

    pub as_child: bool,
}

impl LabelProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            html_for: None,
            for_id: None,
            interactive: true,
            size: ControlSize::Md,
            variant: LabelVariant::Default,
            disabled: false,
            required: false,
            description: None,
            as_child: false,
        }
    }

    pub fn for_id(mut self, id: Id) -> Self {
        self.for_id = Some(id);
        self
    }

    pub fn with_html_for(mut self, target: impl Into<String>) -> Self {
        self.html_for = Some(target.into());
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn variant(mut self, variant: LabelVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        label_with_props(ui, theme, self)
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    props: LabelProps,
}

impl Label {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            props: LabelProps::new(text),
        }
    }

    pub fn for_id(mut self, id: Id) -> Self {
        self.props.for_id = Some(id);
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.props.size = size;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.props.interactive = interactive;
        self
    }

    pub fn variant(mut self, variant: LabelVariant) -> Self {
        self.props.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.props.required = required;
        self
    }

    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.props.description = Some(description.into());
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        label_with_props(ui, theme, self.props)
    }
}

pub fn label_with_props(ui: &mut Ui, theme: &Theme, props: LabelProps) -> Response {
    trace!(
        "Rendering label variant={:?} size={:?} required={} disabled={} with_description={}",
        props.variant,
        props.size,
        props.required,
        props.disabled,
        props.description.is_some()
    );
    let mut style = LabelStyle::from_palette(&theme.palette, props.variant, props.size);
    if props.disabled {
        style = style.disabled();
    }

    let resolved_target = props
        .for_id
        .or_else(|| props.html_for.as_ref().map(Id::new));

    let response = ui
        .scope(|scoped_ui| {
            let mut scoped_style = scoped_ui.style().as_ref().clone();
            scoped_style
                .text_styles
                .insert(TextStyle::Body, style.font.clone());
            scoped_ui.set_style(scoped_style);

            scoped_ui
                .horizontal(|row| {
                    let label_text = props.text.clone().color(style.text);
                    let sense = if props.interactive {
                        Sense::click()
                    } else {
                        Sense::hover()
                    };
                    let label = egui::Label::new(label_text).sense(sense);
                    let label_response = row.add_enabled(!props.disabled, label);

                    if props.required {
                        row.colored_label(style.required, "*");
                    }

                    if let Some(target) = resolved_target
                        && label_response.clicked()
                    {
                        row.memory_mut(|m| m.request_focus(target));
                    }

                    let mut response = label_response;
                    if props.interactive && !props.disabled {
                        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                    response
                })
                .inner
        })
        .inner;

    if let Some(desc) = props.description {
        ui.scope(|desc_ui| {
            let mut desc_style = desc_ui.style().as_ref().clone();

            desc_style
                .text_styles
                .insert(TextStyle::Small, style.description_font.clone());
            desc_ui.set_style(desc_style);

            let desc_text = desc.color(style.description);

            desc_ui.add(egui::Label::new(desc_text).wrap().sense(Sense::hover()));
        });
    }

    response
}

pub fn label(
    ui: &mut Ui,
    theme: &Theme,
    text: impl Into<WidgetText>,
    target: Option<Id>,
    size: ControlSize,
) -> Response {
    Label::new(text)
        .size(size)
        .for_id_opt(target)
        .show(ui, theme)
}

impl Label {
    fn for_id_opt(mut self, target: Option<Id>) -> Self {
        self.props.for_id = target;
        self
    }
}
