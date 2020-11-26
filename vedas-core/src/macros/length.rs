#[macro_export]
macro_rules! fill {
    () => {
        iced::Length::Fill
    };
}
#[macro_export]
macro_rules! portion {
    ($x: expr) => {
        iced::Length::FillPortion($x as u16)
    };
}
#[macro_export]
macro_rules! shrink {
    () => {
        iced::Length::Shrink
    };
}
#[macro_export]
macro_rules! units {
    ($x: expr) => {
        iced::Length::Units($x as u16)
    };
}
