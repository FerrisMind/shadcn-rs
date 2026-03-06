use std::rc::Rc;

use iced::alignment::Vertical;
use iced::widget::{column, container, image, mouse_area, row, text};
use iced::{Background, Border, ContentFit, Element, Length, Theme as IcedTheme, mouse};

use crate::button::{ButtonProps, ButtonVariant, button};
use crate::dialog::dialog;
use crate::slider::{SliderProps, SliderSize, slider};

use super::canvas::CropCanvas;
use super::types::{ImageCropperAction, ImageCropperContext};

pub fn image_cropper_root<'a, Message: Clone + 'a>(
    props: super::types::ImageCropperProps,
    state: &'a super::state::ImageCropperState,
    on_action: impl Fn(ImageCropperAction) -> Message + 'a,
    theme: &'a crate::theme::Theme,
    content: impl FnOnce(ImageCropperContext<'a, Message>) -> Element<'a, Message>,
) -> Element<'a, Message> {
    let ctx = ImageCropperContext {
        props,
        state,
        theme,
        on_action: Rc::new(on_action),
    };
    content(ctx)
}

pub fn image_cropper_upload_trigger<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
    child: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let child: Element<'a, Message> = child.into();

    if ctx.props.disabled {
        return child;
    }

    let action = if ctx.state.has_image() {
        ctx.message(ImageCropperAction::OpenEditor)
    } else {
        ctx.message(ImageCropperAction::PickerRequested)
    };

    mouse_area(child)
        .on_press(action)
        .interaction(mouse::Interaction::Pointer)
        .into()
}

pub fn image_cropper_preview<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
) -> Element<'a, Message> {
    let size = ctx.props.preview_size;
    let palette = ctx.theme.palette;
    let radius = match ctx.props.shape {
        super::types::ImageCropShape::Round => size / 2.0,
        super::types::ImageCropShape::Rect => ctx.theme.radius.md,
    };

    if let Some(handle) = ctx.state.preview_handle() {
        image::Image::new(handle)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .content_fit(ContentFit::Cover)
            .border_radius(radius)
            .into()
    } else {
        container(
            column![
                text("Upload image").size(16),
                text("PNG, JPG, GIF, WebP, AVIF").size(12).style(move |_| {
                    iced::widget::text::Style {
                        color: Some(palette.muted_foreground),
                    }
                }),
            ]
            .spacing(4),
        )
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme: &IcedTheme| iced::widget::container::Style {
            background: Some(Background::Color(palette.muted)),
            text_color: Some(palette.foreground),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: radius.into(),
            },
            ..Default::default()
        })
        .into()
    }
}

pub fn image_cropper_dialog<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let dialog_props = ctx.props.dialog.close_on_blur(false);

    dialog(
        base,
        ctx.state.open,
        content,
        ctx.message(ImageCropperAction::CloseEditor),
        dialog_props,
        ctx.theme,
    )
}

pub fn image_cropper_canvas<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
) -> Element<'a, Message> {
    let Some(handle) = ctx.state.editor_handle() else {
        return container(text("No image selected"))
            .height(Length::Fixed(ctx.props.cropper_height))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
    };
    let Some((width, height)) = ctx.state.editor_dimensions() else {
        return container(text("No image selected")).into();
    };
    let Some(crop_rect) = ctx.state.crop_rect else {
        return container(text("No crop rect")).into();
    };

    iced::widget::canvas::Canvas::new(CropCanvas {
        image_handle: handle,
        image_size: (width, height),
        crop_rect,
        shape: ctx.props.shape,
        theme: ctx.theme.clone(),
        zoom: ctx.state.zoom,
        on_action: Rc::clone(&ctx.on_action),
    })
    .width(Length::Fill)
    .height(Length::Fixed(ctx.props.cropper_height))
    .into()
}

pub fn image_cropper_controls<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
) -> Element<'a, Message> {
    let zoom = ctx.state.zoom;
    let zoom_slider = slider(
        1.0..=4.0,
        vec![zoom],
        Some({
            let on_action = Rc::clone(&ctx.on_action);
            move |values: Vec<f32>| on_action(ImageCropperAction::ZoomChanged(values[0]))
        }),
        SliderProps::new().size(SliderSize::Size2),
        ctx.theme,
    )
    .step(ctx.props.zoom_step)
    .width(Length::Fill);

    row![
        text(format!("Zoom {:0.0}%", zoom * 100.0)).size(14),
        zoom_slider,
    ]
    .spacing(12)
    .align_y(Vertical::Center)
    .into()
}

pub fn image_cropper_crop<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
) -> Element<'a, Message> {
    button(
        "Crop image",
        Some(ctx.message(ImageCropperAction::CropConfirmed)),
        ButtonProps::new().variant(ButtonVariant::Default),
        ctx.theme,
    )
    .into()
}

pub fn image_cropper_cancel<'a, Message: Clone + 'a>(
    ctx: &ImageCropperContext<'a, Message>,
) -> Element<'a, Message> {
    button(
        "Cancel",
        Some(ctx.message(ImageCropperAction::CropCancelled)),
        ButtonProps::new().variant(ButtonVariant::Outline),
        ctx.theme,
    )
    .into()
}
