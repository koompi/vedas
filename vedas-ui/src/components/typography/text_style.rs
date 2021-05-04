/// Text Style for Typography
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Highlight,
    Code,
    Keyboard,
}

impl Style {
    pub const ALL: [Style; 3] = [
        Style::Highlight,
        Style::Code,
        Style::Keyboard
    ];
}

impl ToString for Style {
    fn to_string(&self) -> String {
        use Style::*;

        String::from(match self {
            Highlight => "mark",
            Code => "code",
            Keyboard => "kbd",
        })
    }
}