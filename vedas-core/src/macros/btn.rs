#[macro_export]
macro_rules! btn {
    ($state:expr, $name: expr) => {
        iced::Button::new($state, text!($name))
    };
}
#[macro_export]
macro_rules! btn_svg {
    ($state:expr, $name: expr) => {
        iced::Button::new($state, $name)
    };
}
