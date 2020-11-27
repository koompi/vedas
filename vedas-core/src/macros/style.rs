#[macro_export]
macro_rules! style_container {
    (
        $name: ident { $($items:ident: $typ:expr),* }
    ) => {
        pub struct $name;
        impl iced::container::StyleSheet for $name {
            fn style(&self) -> iced::widget::container::Style {
                iced::widget::container::Style {
                    $($items: $typ),*
                }
            }
        }
    };
}
