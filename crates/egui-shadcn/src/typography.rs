use crate::theme::Theme;
use crate::tokens::mix;
use egui::{
    Align, Color32, CornerRadius, FontFamily, FontId, Frame, Response, RichText, Sense, Stroke, Ui,
    WidgetText, pos2, vec2,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAs {
    Span,
    Div,
    Label,
    P,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWeight {
    Light,
    Regular,
    Medium,
    Bold,
    ExtraBold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    fn to_egui(self) -> Align {
        match self {
            TextAlign::Left => Align::Min,
            TextAlign::Center => Align::Center,
            TextAlign::Right => Align::Max,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextTrim {
    Normal,
    Start,
    End,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWrap {
    Wrap,
    NoWrap,
    Pretty,
    Balance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypographyColor {
    Default,
    Muted,
    Primary,
}

impl TypographyColor {
    fn resolve(self, theme: &Theme, high_contrast: bool) -> Color32 {
        match self {
            TypographyColor::Default => theme.palette.foreground,
            TypographyColor::Muted => {
                if high_contrast {
                    mix(
                        theme.palette.muted_foreground,
                        theme.palette.foreground,
                        0.25,
                    )
                } else {
                    theme.palette.muted_foreground
                }
            }
            TypographyColor::Primary => theme.palette.primary,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingAs {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkUnderline {
    Auto,
    Always,
    Hover,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeVariant {
    Solid,
    Soft,
    Outline,
    Ghost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadcnTypographyVariant {
    H1,
    H2,
    H3,
    H4,
    P,
    Lead,
    Large,
    Small,
    Muted,
    InlineCode,
    Blockquote,
}

#[derive(Clone, Debug)]
pub struct TextProps {
    pub text: WidgetText,
    pub as_tag: TextAs,
    pub size: Option<f32>,
    pub weight: Option<TextWeight>,
    pub align: Option<TextAlign>,
    pub trim: Option<TextTrim>,
    pub truncate: bool,
    pub wrap: Option<TextWrap>,
    pub color: Option<TypographyColor>,
    pub high_contrast: bool,
    pub italic: bool,
    pub monospace: bool,
    pub underline: bool,
}

impl TextProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            as_tag: TextAs::Span,
            size: None,
            weight: None,
            align: None,
            trim: None,
            truncate: false,
            wrap: None,
            color: None,
            high_contrast: false,
            italic: false,
            monospace: false,
            underline: false,
        }
    }

    pub fn as_tag(mut self, as_tag: TextAs) -> Self {
        self.as_tag = as_tag;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn trim(mut self, trim: TextTrim) -> Self {
        self.trim = Some(trim);
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn color(mut self, color: TypographyColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn monospace(mut self, monospace: bool) -> Self {
        self.monospace = monospace;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }
}

#[derive(Clone, Debug)]
pub struct HeadingProps {
    pub text: WidgetText,
    pub as_tag: HeadingAs,
    pub size: Option<f32>,
    pub weight: Option<TextWeight>,
    pub align: Option<TextAlign>,
    pub trim: Option<TextTrim>,
    pub truncate: bool,
    pub wrap: Option<TextWrap>,
    pub color: Option<TypographyColor>,
    pub high_contrast: bool,
}

impl HeadingProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            as_tag: HeadingAs::H1,
            size: Some(30.0),
            weight: None,
            align: None,
            trim: None,
            truncate: false,
            wrap: None,
            color: None,
            high_contrast: false,
        }
    }

    pub fn as_tag(mut self, as_tag: HeadingAs) -> Self {
        self.as_tag = as_tag;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn trim(mut self, trim: TextTrim) -> Self {
        self.trim = Some(trim);
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn color(mut self, color: TypographyColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

#[derive(Clone, Debug)]
pub struct LinkProps {
    pub text: WidgetText,
    pub underline: LinkUnderline,
    pub size: Option<f32>,
    pub weight: Option<TextWeight>,
    pub trim: Option<TextTrim>,
    pub truncate: bool,
    pub wrap: Option<TextWrap>,
    pub color: Option<TypographyColor>,
    pub high_contrast: bool,
}

impl LinkProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            underline: LinkUnderline::Auto,
            size: None,
            weight: None,
            trim: None,
            truncate: false,
            wrap: None,
            color: Some(TypographyColor::Primary),
            high_contrast: false,
        }
    }

    pub fn underline(mut self, underline: LinkUnderline) -> Self {
        self.underline = underline;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn trim(mut self, trim: TextTrim) -> Self {
        self.trim = Some(trim);
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn color(mut self, color: TypographyColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

#[derive(Clone, Debug)]
pub struct CodeProps {
    pub text: WidgetText,
    pub variant: CodeVariant,
    pub size: Option<f32>,
    pub weight: Option<TextWeight>,
    pub color: Option<TypographyColor>,
    pub high_contrast: bool,
    pub truncate: bool,
    pub wrap: Option<TextWrap>,
}

impl CodeProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            variant: CodeVariant::Soft,
            size: None,
            weight: None,
            color: None,
            high_contrast: false,
            truncate: false,
            wrap: None,
        }
    }

    pub fn variant(mut self, variant: CodeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn color(mut self, color: TypographyColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }
}

#[derive(Clone, Debug)]
pub struct KbdProps {
    pub text: WidgetText,
    pub size: Option<f32>,
}

impl KbdProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            size: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

#[derive(Clone, Debug)]
pub struct BlockquoteProps {
    pub text: WidgetText,
    pub size: Option<f32>,
    pub high_contrast: bool,
    pub truncate: bool,
    pub wrap: Option<TextWrap>,
}

impl BlockquoteProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            size: None,
            high_contrast: false,
            truncate: false,
            wrap: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.wrap = Some(wrap);
        self
    }
}

#[derive(Clone, Debug)]
pub struct TypographyProps {
    pub text: WidgetText,
    pub variant: ShadcnTypographyVariant,
    pub align: Option<TextAlign>,
}

impl TypographyProps {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            variant: ShadcnTypographyVariant::P,
            align: None,
        }
    }

    pub fn variant(mut self, variant: ShadcnTypographyVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedTextStyle {
    pub size: f32,
    pub monospace: bool,
    pub strong: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: TypographyColor,
}

pub fn resolve_shadcn_style(variant: ShadcnTypographyVariant) -> ResolvedTextStyle {
    match variant {
        ShadcnTypographyVariant::H1 => ResolvedTextStyle {
            size: 36.0,
            monospace: false,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::H2 => ResolvedTextStyle {
            size: 30.0,
            monospace: false,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::H3 => ResolvedTextStyle {
            size: 24.0,
            monospace: false,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::H4 => ResolvedTextStyle {
            size: 20.0,
            monospace: false,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::Lead => ResolvedTextStyle {
            size: 20.0,
            monospace: false,
            strong: false,
            italic: false,
            underline: false,
            color: TypographyColor::Muted,
        },
        ShadcnTypographyVariant::Large => ResolvedTextStyle {
            size: 18.0,
            monospace: false,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::Small => ResolvedTextStyle {
            size: 14.0,
            monospace: false,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::Muted => ResolvedTextStyle {
            size: 14.0,
            monospace: false,
            strong: false,
            italic: false,
            underline: false,
            color: TypographyColor::Muted,
        },
        ShadcnTypographyVariant::InlineCode => ResolvedTextStyle {
            size: 14.0,
            monospace: true,
            strong: true,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::Blockquote => ResolvedTextStyle {
            size: 14.0,
            monospace: false,
            strong: false,
            italic: true,
            underline: false,
            color: TypographyColor::Default,
        },
        ShadcnTypographyVariant::P => ResolvedTextStyle {
            size: 14.0,
            monospace: false,
            strong: false,
            italic: false,
            underline: false,
            color: TypographyColor::Default,
        },
    }
}

fn widget_text_to_plain(text: WidgetText) -> String {
    match text {
        WidgetText::Text(t) => t,
        WidgetText::RichText(rt) => rt.text().to_string(),
        WidgetText::Galley(g) => g.text().to_string(),
        WidgetText::LayoutJob(job) => job.text.to_string(),
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct TextStyleOptions {
    size: Option<f32>,
    weight: Option<TextWeight>,
    italic: bool,
    monospace: bool,
    underline: bool,
    color: Option<TypographyColor>,
    high_contrast: bool,
}

fn resolve_text_style(theme: &Theme, opts: TextStyleOptions) -> ResolvedTextStyle {
    let size = opts.size.unwrap_or(14.0);
    let strong = matches!(
        opts.weight.unwrap_or(TextWeight::Regular),
        TextWeight::Medium | TextWeight::Bold | TextWeight::ExtraBold
    );
    let mut resolved = ResolvedTextStyle {
        size,
        monospace: opts.monospace,
        strong,
        italic: opts.italic,
        underline: opts.underline,
        color: opts.color.unwrap_or(TypographyColor::Default),
    };
    if opts.high_contrast && resolved.color == TypographyColor::Muted {
        resolved.color = TypographyColor::Default;
    }
    let _ = theme;
    resolved
}

fn label_for_text(
    ui: &mut Ui,
    rich: RichText,
    align: Option<TextAlign>,
    wrap: Option<TextWrap>,
) -> Response {
    let wrap = wrap.unwrap_or(TextWrap::Wrap);
    let mut label = egui::Label::new(rich);
    if !matches!(wrap, TextWrap::NoWrap) {
        label = label.wrap();
    }
    if let Some(align) = align {
        ui.with_layout(egui::Layout::top_down(align.to_egui()), |ui| ui.add(label))
            .inner
    } else {
        ui.add(label)
    }
}

pub fn text(ui: &mut Ui, theme: &Theme, props: TextProps) -> Response {
    let resolved = resolve_text_style(
        theme,
        TextStyleOptions {
            size: props.size,
            weight: props.weight,
            italic: props.italic,
            monospace: props.monospace,
            underline: props.underline,
            color: props.color,
            high_contrast: props.high_contrast,
        },
    );

    let color = resolved.color.resolve(theme, props.high_contrast);
    let mut rich = RichText::new(widget_text_to_plain(props.text));
    rich = rich.color(color);
    rich = rich.font(FontId::new(
        resolved.size,
        if resolved.monospace {
            FontFamily::Monospace
        } else {
            FontFamily::Proportional
        },
    ));
    if resolved.strong {
        rich = rich.strong();
    }
    if resolved.italic {
        rich = rich.italics();
    }
    if resolved.underline {
        rich = rich.underline();
    }

    let _ = props.as_tag;
    let _ = props.trim;
    let _ = props.truncate;

    label_for_text(ui, rich, props.align, props.wrap)
}

pub fn heading(ui: &mut Ui, theme: &Theme, props: HeadingProps) -> Response {
    let mut resolved = resolve_text_style(
        theme,
        TextStyleOptions {
            size: props.size,
            weight: props.weight,
            color: props.color,
            high_contrast: props.high_contrast,
            ..Default::default()
        },
    );
    if !resolved.strong {
        resolved.strong = true;
    }

    let color = resolved.color.resolve(theme, props.high_contrast);
    let rich = RichText::new(widget_text_to_plain(props.text))
        .color(color)
        .font(FontId::proportional(resolved.size))
        .strong();

    let _ = props.as_tag;
    let _ = props.trim;
    let _ = props.truncate;

    label_for_text(ui, rich, props.align, props.wrap)
}

pub fn link(ui: &mut Ui, theme: &Theme, props: LinkProps) -> Response {
    let resolved = resolve_text_style(
        theme,
        TextStyleOptions {
            size: props.size,
            weight: props.weight,
            color: props.color,
            high_contrast: props.high_contrast,
            ..Default::default()
        },
    );
    let base_color = resolved.color.resolve(theme, props.high_contrast);
    let mut rich = RichText::new(widget_text_to_plain(props.text))
        .color(base_color)
        .font(FontId::proportional(resolved.size));
    if resolved.strong {
        rich = rich.strong();
    }

    let base = ui.add(egui::Label::new(rich).sense(Sense::click()).wrap());
    let underline = match props.underline {
        LinkUnderline::None => false,
        LinkUnderline::Always => true,
        LinkUnderline::Hover => base.hovered(),
        LinkUnderline::Auto => base.hovered(),
    };
    if underline {
        let painter = ui.painter();
        let y = base.rect.bottom() - 1.0;
        painter.line_segment(
            [pos2(base.rect.left(), y), pos2(base.rect.right(), y)],
            Stroke::new(1.0, base_color),
        );
    }

    let _ = props.trim;
    let _ = props.truncate;
    let _ = props.wrap;

    base
}

pub fn code(ui: &mut Ui, theme: &Theme, props: CodeProps) -> Response {
    let resolved = resolve_text_style(
        theme,
        TextStyleOptions {
            size: props.size.or(Some(14.0)),
            weight: props.weight.or(Some(TextWeight::Bold)),
            monospace: true,
            color: props.color,
            high_contrast: props.high_contrast,
            ..Default::default()
        },
    );
    let fg = resolved.color.resolve(theme, props.high_contrast);
    let rounding = CornerRadius::same(4);

    let (fill, stroke) = match props.variant {
        CodeVariant::Soft => (theme.palette.muted, Stroke::NONE),
        CodeVariant::Solid => (theme.palette.primary, Stroke::NONE),
        CodeVariant::Outline => (Color32::TRANSPARENT, Stroke::new(1.0, theme.palette.border)),
        CodeVariant::Ghost => (Color32::TRANSPARENT, Stroke::NONE),
    };

    let rich = RichText::new(widget_text_to_plain(props.text))
        .font(FontId::new(resolved.size, FontFamily::Monospace))
        .color(match props.variant {
            CodeVariant::Solid => theme.palette.primary_foreground,
            _ => fg,
        })
        .strong();

    let inner_margin = vec2(6.0, 4.0);
    let frame = Frame::NONE
        .fill(fill)
        .stroke(stroke)
        .corner_radius(rounding)
        .inner_margin(inner_margin);

    let response = frame
        .show(ui, |ui| ui.add(egui::Label::new(rich).wrap()))
        .inner;

    let _ = props.truncate;
    let _ = props.wrap;

    response
}

pub fn kbd(ui: &mut Ui, theme: &Theme, props: KbdProps) -> Response {
    let size = props.size.unwrap_or(13.0);
    let rounding = CornerRadius::same(4);
    let fill = mix(theme.palette.muted, theme.palette.background, 0.6);
    let stroke = Stroke::new(1.0, theme.palette.border);
    let rich = RichText::new(widget_text_to_plain(props.text))
        .font(FontId::new(size, FontFamily::Monospace))
        .color(theme.palette.foreground)
        .strong();

    Frame::NONE
        .fill(fill)
        .stroke(stroke)
        .corner_radius(rounding)
        .inner_margin(vec2(6.0, 4.0))
        .show(ui, |ui| ui.add(egui::Label::new(rich)))
        .inner
}

pub fn blockquote(ui: &mut Ui, theme: &Theme, props: BlockquoteProps) -> Response {
    let size = props.size.unwrap_or(14.0);
    let fg = theme.palette.foreground;
    let rich = RichText::new(widget_text_to_plain(props.text))
        .font(FontId::proportional(size))
        .color(fg)
        .italics();

    let indent = 24.0;
    let border_width = 2.0;
    let response = ui
        .horizontal(|ui| {
            ui.add_space(indent);
            ui.add(egui::Label::new(rich).wrap())
        })
        .inner;
    let border_x = response.rect.left() - indent + border_width * 0.5;
    ui.painter().line_segment(
        [
            pos2(border_x, response.rect.top()),
            pos2(border_x, response.rect.bottom()),
        ],
        Stroke::new(border_width, theme.palette.border),
    );

    let _ = props.truncate;
    let _ = props.wrap;

    response
}

pub fn typography(ui: &mut Ui, theme: &Theme, props: TypographyProps) -> Response {
    match props.variant {
        ShadcnTypographyVariant::InlineCode => code(
            ui,
            theme,
            CodeProps::new(props.text).variant(CodeVariant::Soft),
        ),
        ShadcnTypographyVariant::Blockquote => {
            blockquote(ui, theme, BlockquoteProps::new(props.text))
        }
        variant => {
            let resolved = resolve_shadcn_style(variant);
            let color = resolved.color.resolve(theme, false);
            let mut rich = RichText::new(widget_text_to_plain(props.text))
                .font(FontId::new(
                    resolved.size,
                    if resolved.monospace {
                        FontFamily::Monospace
                    } else {
                        FontFamily::Proportional
                    },
                ))
                .color(color);
            if resolved.strong {
                rich = rich.strong();
            }
            if resolved.italic {
                rich = rich.italics();
            }
            if resolved.underline {
                rich = rich.underline();
            }

            let response = label_for_text(ui, rich, props.align, Some(TextWrap::Wrap));
            if variant == ShadcnTypographyVariant::H2 {
                let padding_bottom = 8.0;
                ui.add_space(padding_bottom);
                let y = response.rect.bottom() + padding_bottom;
                ui.painter().line_segment(
                    [
                        pos2(response.rect.left(), y),
                        pos2(response.rect.right(), y),
                    ],
                    Stroke::new(1.0, theme.palette.border),
                );
            }
            response
        }
    }
}
