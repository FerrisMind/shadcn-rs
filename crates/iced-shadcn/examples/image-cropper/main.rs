use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Task};

#[cfg(feature = "rfd")]
use iced_shadcn::image_cropper_pick_file_task;
use iced_shadcn::{
    CardProps, CardSize, CardVariant, ImageCropResult, ImageCropShape, ImageCropperAction,
    ImageCropperProps, ImageCropperSource, ImageCropperState, Theme, card, image_cropper_cancel,
    image_cropper_canvas, image_cropper_controls, image_cropper_crop, image_cropper_dialog,
    image_cropper_preview, image_cropper_root, image_cropper_upload_trigger,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    round: ImageCropperState,
    rect: ImageCropperState,
    empty: ImageCropperState,
    custom: ImageCropperState,
}

#[derive(Clone, Debug)]
enum Message {
    Round(ImageCropperAction),
    Rect(ImageCropperAction),
    Empty(ImageCropperAction),
    Custom(ImageCropperAction),
    #[cfg(feature = "rfd")]
    EmptyPicked(Option<ImageCropperSource>),
}

impl Default for Example {
    fn default() -> Self {
        let mut round = ImageCropperState::new();
        round.apply(
            ImageCropperAction::SetExternalSource(seed_source()),
            ImageCropShape::Round,
        );

        let mut rect = ImageCropperState::new();
        rect.apply(
            ImageCropperAction::SetExternalSource(seed_source()),
            ImageCropShape::Rect,
        );

        let mut custom = ImageCropperState::new();
        custom.apply(
            ImageCropperAction::SetExternalSource(seed_source()),
            ImageCropShape::Round,
        );

        Self {
            theme: Theme::dark(),
            round,
            rect,
            empty: ImageCropperState::new(),
            custom,
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Round(action) => apply_action(&mut self.round, action, ImageCropShape::Round),
            Message::Rect(action) => apply_action(&mut self.rect, action, ImageCropShape::Rect),
            Message::Custom(action) => {
                apply_action(&mut self.custom, action, ImageCropShape::Round)
            }
            Message::Empty(action) => {
                #[cfg(feature = "rfd")]
                if matches!(action, ImageCropperAction::PickerRequested) {
                    return image_cropper_pick_file_task(Message::EmptyPicked);
                }

                #[cfg(not(feature = "rfd"))]
                if matches!(action, ImageCropperAction::PickerRequested) {
                    self.empty.apply(
                        ImageCropperAction::FileAccepted(seed_source()),
                        ImageCropShape::Round,
                    );
                    return Task::none();
                }

                apply_action(&mut self.empty, action, ImageCropShape::Round)
            }
            #[cfg(feature = "rfd")]
            Message::EmptyPicked(source) => {
                if let Some(source) = source {
                    self.empty.apply(
                        ImageCropperAction::FileAccepted(source),
                        ImageCropShape::Round,
                    );
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            header(theme),
            demo_card(
                theme,
                "Default",
                self.cropper_panel(&self.round, ImageCropShape::Round, Message::Round)
            ),
            demo_card(
                theme,
                "Square Preview",
                self.cropper_panel(&self.rect, ImageCropShape::Rect, Message::Rect)
            ),
            demo_card(
                theme,
                "No Default Image",
                self.cropper_panel(&self.empty, ImageCropShape::Round, Message::Empty)
            ),
            demo_card(theme, "Custom Preview", self.custom_panel()),
        ]
        .spacing(20)
        .width(Length::Fill);

        container(scrollable(content))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.background)),
                ..Default::default()
            })
            .into()
    }

    fn cropper_panel<'a>(
        &'a self,
        state: &'a ImageCropperState,
        shape: ImageCropShape,
        map: fn(ImageCropperAction) -> Message,
    ) -> Element<'a, Message> {
        let props = ImageCropperProps::new().shape(shape);
        image_cropper_root(props, state, map, &self.theme, |ctx| {
            let trigger = image_cropper_upload_trigger(&ctx, image_cropper_preview(&ctx));
            image_cropper_dialog(
                &ctx,
                trigger,
                column![
                    image_cropper_canvas(&ctx),
                    image_cropper_controls(&ctx),
                    row![image_cropper_cancel(&ctx), image_cropper_crop(&ctx)]
                        .spacing(12)
                        .align_y(Alignment::Center),
                    result_line(state.latest_result.as_ref()),
                ]
                .spacing(16),
            )
        })
    }

    fn custom_panel<'a>(&'a self) -> Element<'a, Message> {
        let props = ImageCropperProps::new().shape(ImageCropShape::Round);
        image_cropper_root(props, &self.custom, Message::Custom, &self.theme, |ctx| {
            let trigger = image_cropper_upload_trigger(
                &ctx,
                container(
                    column![image_cropper_preview(&ctx), text("Change avatar").size(14)]
                        .spacing(10)
                        .align_x(Alignment::Center),
                )
                .padding(12),
            );

            image_cropper_dialog(
                &ctx,
                trigger,
                column![
                    text("Custom trigger + preview").size(18),
                    image_cropper_canvas(&ctx),
                    image_cropper_controls(&ctx),
                    row![image_cropper_cancel(&ctx), image_cropper_crop(&ctx)].spacing(12),
                    result_line(self.custom.latest_result.as_ref()),
                ]
                .spacing(16),
            )
        })
    }
}

fn apply_action(
    state: &mut ImageCropperState,
    action: ImageCropperAction,
    shape: ImageCropShape,
) -> Task<Message> {
    let _ = state.apply(action, shape);
    Task::none()
}

fn header<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let content = column![
        text("Image Cropper demo").size(28),
        text("Covers default, square preview, no default image, and custom preview flows.")
            .size(14)
            .style(move |_| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(8);

    #[cfg(not(feature = "rfd"))]
    let content = content.push(
        text("Picker feature is off; the empty-state demo injects a sample image instead.")
            .size(12)
            .style(move |_| iced::widget::text::Style {
                color: Some(theme.palette.primary),
            }),
    );

    content.into()
}

fn demo_card<'a>(
    theme: &'a Theme,
    title: &'a str,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    card(
        column![text(title).size(18), content].spacing(16),
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size3),
        theme,
    )
    .into()
}

fn result_line<'a>(result: Option<&'a ImageCropResult>) -> Element<'a, Message> {
    match result {
        Some(result) => text(format!(
            "PNG {}x{}, {} bytes",
            result.output_width,
            result.output_height,
            result.png_bytes.len()
        ))
        .size(12)
        .into(),
        None => text("No cropped output yet").size(12).into(),
    }
}

fn seed_source() -> ImageCropperSource {
    ImageCropperSource::new(SEED_PNG.to_vec())
        .name("seed.png")
        .mime("image/png")
}

const SEED_PNG: &[u8] = include_bytes!("../../assets/icons/shadcn-iced/256x256.png");
