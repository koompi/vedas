#[macro_export]
macro_rules! checkbox {
    ($state:expr, $description: expr, $message: expr) => {
        iced::Checkbox::new($state, $description, $message)
    };
}
