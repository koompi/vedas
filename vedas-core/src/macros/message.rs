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
macro_rules! as_item {
    ($i:item) => {
        #[derive(Debug, Clone)]
        $i
    };
}
