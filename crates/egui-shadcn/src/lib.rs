//! Библиотека компонентов для egui по мотивам shadcn/ui.
//! В первой версии включены базовые элементы форм.

pub mod theme;
pub mod tokens;

pub use theme::{ControlVisuals, InputVisuals, Theme};
pub use tokens::{
    input_tokens, mix, variant_tokens, ColorPalette, ControlSize, ControlVariant, InputTokens,
    StateColors, VariantTokens,
};

/// Общее состояние библиотеки.
#[cfg(test)]
mod tests {
    use env_logger;
    use log;

    /// Проверяет, что crate инициализируется и логгер конфигурируется в тестах.
    #[test]
    fn crate_inits() {
        let _ = env_logger::builder().is_test(true).try_init();
        log::info!("egui-shadcn crate initialized for testing");
        assert!(true);
    }
}

