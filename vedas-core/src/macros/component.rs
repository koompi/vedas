#[macro_export]
macro_rules! component {
    // ($($element: ident: $ty: ty),*)
    // loop over arguments seperated by comma
    ($name: ident: $arg:expr => $($element: ident: $ty: ty),* ) =>
        {
            #[derive(Default, Debug)]
            pub struct $name { $($element: $ty),* }

            // impl $name {
            //     f!(new, Self);
            //     af_ref_self!(title, String);
            //     af_ref_mut_self!(update,Self, user_name: String, email: String);
            //     af_ref_mut_self!(view,Self, user_name: String, email: String);
            // }
        }
    }

#[macro_export]
macro_rules! f {
    ($fn_name:ident, $body: block ) => {
        fn $fn_name() $body;
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        fn $fn_name() -> $return_type $body
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name($($fn_args: $ty),*) -> $return_type $body;
    };
}
#[macro_export]
macro_rules! f_self {
    ($fn_name:ident, $body: block) => {
        fn $fn_name(self) {}
    };
    ($fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(self, $($fn_args: $ty),*) $body;
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        fn $fn_name(self) -> $return_type $body;
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(self, $($fn_args: $ty),*) -> $return_type $body;
    }
}

#[macro_export]
macro_rules! f_mut_self {
    ($fn_name:ident, $body: block) => {
        fn $fn_name(mut self) {}
    };
    ($fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(mut self, $($fn_args: $ty),*) $body;
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        fn $fn_name(mut self) -> $return_type $body;
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(mut self, $($fn_args: $ty),*) -> $return_type $body;
    }
}

#[macro_export]
macro_rules! f_ref_self {
    ($self:ident, $fn_name:ident) => {
        fn $fn_name(&$self) {}
    };
    ($self:ident, $fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(&$self, $($fn_args: $ty),*) $body
    };
    ($self:ident, $fn_name:ident, $return_type:ty, $body: block) => {
        fn $fn_name(&$self) -> $return_type { $body }
    };
    ($self:ident, $fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(&$self, $($fn_args: $ty),*) -> $return_type $body
    }
}

#[macro_export]
macro_rules! f_ref_mut_self {
    ($self:ident, $fn_name:ident, $body: block) => {
        fn $fn_name($self) $body
    };
    ($self:ident, $fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(&mut $self, $($fn_args: $ty),*) $body
    };
    ($self:ident, $fn_name:ident, $return_type:ty, $body: block) => {
        fn $fn_name(&mut $self) -> $return_type $body
    };
    ($self:ident, $fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name($self, $($fn_args: $ty),*) -> $return_type $body;
    }
}

#[macro_export]
macro_rules! af {
    ($fn_name:ident, $body: block) => {
        async fn $fn_name() {}
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        async fn $fn_name() -> $return_type $body;
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        async fn $fn_name($($fn_args: $ty),*) -> $return_type $body;
    };
}
#[macro_export]
macro_rules! af_self {
    ($fn_name:ident, $body: block) => {
        async fn $fn_name(self) {}
    };
    ($fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        async fn $fn_name(self, $($fn_args: $ty),*) $body;
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        async fn $fn_name(self) -> $return_type $body;
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        async fn $fn_name(self, $($fn_args: $ty),*) -> $return_type $body;
    }
}

#[macro_export]
macro_rules! af_mut_self {
    ($fn_name:ident, $body: block) => {
        async fn $fn_name(mut self) {}
    };
    ($fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        fn $fn_name(mut self, $($fn_args: $ty),*) $body;
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        async fn $fn_name(mut self) -> $return_type $body;
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        async fn $fn_name(mut self, $($fn_args: $ty),*) -> $return_type $body;
    }
}

#[macro_export]
macro_rules! af_ref_self {
    ($fn_name:ident, $body: block) => {
        async fn $fn_name(&self) {}
    };
    ($fn_name:ident, $($fn_args: ident: $ty: ty),*, $body: block) => {
        async fn $fn_name(&self, $($fn_args: $ty),*) $body;
    };
    ($fn_name:ident, $return_type:ty, $body: block) => {
        async fn $fn_name(&self) -> $return_type $body;
    };
    ($fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),*, $body: block) => {
        async fn $fn_name(&self, $($fn_args: $ty),*) -> $return_type $body;
    }
}

#[macro_export]
macro_rules! af_ref_mut_self {
    ($self:expr, $fn_name:ident, {$body: block}) => {
        async fn $fn_name($self) $body
    };
    ($self:expr, $fn_name:ident, $($fn_args: ident: $ty: ty),*, {$body: block}) => {
        async fn $fn_name($self, $($fn_args: $ty),*) $body;
    };
    ($self:expr, $fn_name:ident, $return_type:ty) => {
        async fn $fn_name($self) -> $return_type $body;
    };
    ($self:expr, $fn_name:ident, $return_type:ty, $($fn_args: ident: $ty: ty),* {$body: block}) => {
        async fn $fn_name($self, $($fn_args: $ty),*) -> $return_type $body;
    }
}
