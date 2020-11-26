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
    // ($name:expr, $($x:tt => $func: block)*) => {
    //     match $name {
    //         $($x)* => $func,
    //         x @ Some(_) => panic!("Expected {:?}, got {:?}", $($x)*, x),
    //         None => panic!("Expected {:?}, got None", $($x)*)
    //     }
    // }
    // ($message: expr, $type:ty, $(<$enum:ty>::$variant:ident( $($args:ident),* ) => $tree:block ),* ) => {{
    //     type Enum = $type;

    //     match $message {
    //         $(Enum::$variant($($args),*) => { $tree } ),*
    //     }
    // }}
}

// macro_rules! message {
//     ($name:ident, $($body:tt)*) => {
//         as_item! {
//             pub enum $name { $($body)* }
//         }
//     };
// }

// #[macro_export]
// macro_rules! as_arm {
//     ($i:item) => {
//         $i
//     };
// }
