#[macro_export]
macro_rules! message {
    ($name: ident) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {}
    };
    ($name: ident, $($variants:ident),*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($variants),*
        }
    };
    ($name: ident, $($variants:ident: $($type:ty);*),*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($variants($($type),*)),*
        }
    };
}
