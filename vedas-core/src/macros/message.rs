#[macro_export]
macro_rules! message {
    ($name:ident, $($body:tt)*) => {
        as_item! {
            enum $name { $($body)* }
        }
    };
    ($v:vis $name:ident, $($body:tt)*) => {
        as_item! {
            $v enum $name { $($body)* }
        }
    };
}
#[macro_export]
macro_rules! message_cpeq {
    ($name:ident, $($body:tt)*) => {
        as_item_copy_eq! {
            pub enum $name {$($body)*}
        }
    };
}
#[macro_export]
macro_rules! as_item {
    ($i:item) => {
        #[derive(Debug, Clone)]
        $i
    };
}


#[macro_export]
macro_rules! as_item_copy_eq {
    ($i:item) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $i
    };
}