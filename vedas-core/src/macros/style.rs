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

#[macro_export]
macro_rules! style_btn {
    (
        $name: ident { $($items:ident: $typ:expr),* }
    ) => {
        pub struct $name;
        impl iced::widget::button::StyleSheet for $name {
            fn active(&self) -> iced::widget::button::Style {
                iced::widget::button::Style {
                    $($items: $typ),*
                }
            }
        }
    };
}
