#[macro_export]
macro_rules! col {
    () => {
        iced::Column::new();
    };
    ($($args:expr),*) => {{
        iced::Column::new()
    }};
}
