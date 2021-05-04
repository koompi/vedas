/// Button Styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Default,
    Fill,
    Dashed,
    Link,
    Text
}

impl Style {
    pub const ALL: [Style; 5] = [
        Style::Default,
        Style::Fill,
        Style::Dashed,
        Style::Link,
        Style::Text
    ];

    pub fn is_unbordered(&self) -> bool {
        use Style::*;

        match self {
            Link | Text => true,
            _ => false
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Default
    }
}

impl ToString for Style {
    fn to_string(&self) -> String {
        use Style::*;

        String::from(match self {
            Default => "",
            Fill => "primary",
            Dashed => "dashed",
            Link => "link",
            Text => "text"
        })
    }
}