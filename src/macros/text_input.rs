#[macro_export]
macro_rules! txt_input {
    ($state: expr, $placeholder: expr, $value:expr, $message:expr) => {
        iced::TextInput::new($state, $placeholder, $value, $message)
    };
}
