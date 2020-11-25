#[macro_export]
macro_rules! mtch {
    ($expr_name:expr, $type_name: ident,  $($variant:ident $tree:tt $func:block),*) => {
        match $expr_name {
            $($type_name::$variant => {$func} ),*
        }
    };
    // with tuple
    ($message: expr, $type:ty, $(<$enum:ty>::$variant:ident($($args:ident),*) $tree:tt $func:block),* ) => {{
        type Enum = $type;
        match $message {
            $(Enum::$variant($($args),*) $tree $func ),*
        }
    }}
}
