#[macro_export]
macro_rules! select {
   ($state:expr, $options:expr, $selected:expr, $on_selected:expr) => {
      iced::PickList::new($state, $options, $selected, $on_selected)
   };
}

#[macro_export]
macro_rules! select_all {
    ($name:ident,$field:ident,$count:expr,$($body:tt)*) => {
        impl $name {
            const $field:[$name;$count]  = [
               $($body)*
            ];
        }

    }
}
#[macro_export]
macro_rules! select_default {
    ($name:ident, $default:expr) => {
        impl Default for $name {
            fn default() -> $name {
                $default
            }
        }
    };
}

#[macro_export]
macro_rules! select_display {
    ($name:ident, $($key:path => $value:expr),+ ) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                       $($key => $value),+
                })
            }
        }
    };
}