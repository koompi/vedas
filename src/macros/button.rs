#[macro_export]
macro_rules! btn {
    ($state:expr, $name: expr) => {
        iced::Button::new($state, text!($name))
    };
}
