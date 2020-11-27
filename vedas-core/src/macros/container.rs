#[macro_export]
macro_rules! container {
    ($item:expr) => {
        iced::Container::new($item)
    };
    ($wh:expr, $item:expr) => {
        iced::Container::new($item).width($wh).height($wh)
    };
    ($w:expr, $h:expr, $item:expr) => {
        iced::Container::new($item).width($w).height($h)
    };
}
