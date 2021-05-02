/// Button Sizes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Middle,
    Small,
    Large,
}

impl Size {
    pub const ALL: [Size; 3] = [
        Size::Small,
        Size::Middle,
        Size::Large
    ];
}

impl ToString for Size {
    fn to_string(&self) -> String {
        use Size::*;

        String::from(match self {
            Middle => "",
            Small => "sm",
            Large => "lg",
        })
    }
}