#[macro_export]
macro_rules! col {
    () => {
        iced::Column::new();
    };
    ($w:expr) => {
        iced::Column::new().width($w)
    };
}
