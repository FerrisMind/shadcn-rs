use iced::Task;

use super::types::ImageCropperSource;

pub fn image_cropper_pick_file_task<Message: Send + 'static>(
    map: impl Fn(Option<ImageCropperSource>) -> Message + Send + 'static,
) -> Task<Message> {
    Task::perform(
        async move {
            let file = rfd::FileDialog::new()
                .add_filter(
                    "Image",
                    &["png", "jpg", "jpeg", "gif", "webp", "avif", "apng"],
                )
                .pick_file();

            file.and_then(|path| {
                let bytes = std::fs::read(&path).ok()?;
                let mime = guess_mime(&path);
                Some(
                    ImageCropperSource::new(bytes)
                        .name(path.file_name()?.to_string_lossy().to_string())
                        .mime(mime),
                )
            })
        },
        map,
    )
}

fn guess_mime(path: &std::path::Path) -> String {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|value| value.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") | Some("apng") => String::from("image/png"),
        Some("jpg") | Some("jpeg") => String::from("image/jpeg"),
        Some("gif") => String::from("image/gif"),
        Some("webp") => String::from("image/webp"),
        Some("avif") => String::from("image/avif"),
        Some("svg") => String::from("image/svg+xml"),
        _ => String::from("application/octet-stream"),
    }
}
