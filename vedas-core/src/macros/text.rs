#[macro_export]
macro_rules! text {
    ($arg:expr) => {
        iced::Text::new($arg.to_string())
    };
}
