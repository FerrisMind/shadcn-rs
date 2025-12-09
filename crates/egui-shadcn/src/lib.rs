//! Библиотека компонентов для egui по мотивам shadcn/ui.
//! В первой версии включены базовые элементы форм.

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

