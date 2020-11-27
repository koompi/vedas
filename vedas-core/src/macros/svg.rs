#[macro_export]
macro_rules! svg {
    ($path:expr) => {
        Svg::from_path(std::path::PathBuf::from($path))
    };
    ($path:expr, $size:expr) => {
        Svg::from_path(std::path::PathBuf::from($path))
            .width(units!($size))
            .height(units!($size))
    };
}
