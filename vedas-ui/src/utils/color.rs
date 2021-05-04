/// Button Styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Default,
    Primary,
    Secondary,
    Info,
    Success,
    Warning,
    Danger,
}

impl Color {
    pub const ALL: [Color; 7] = [
        Color::Default,
        Color::Primary,
        Color::Secondary,
        Color::Info,
        Color::Success,
        Color::Warning,
        Color::Danger
    ];
}

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        use Color::*;

        String::from(match self {
            Default => "",
            Primary => "primary",
            Secondary => "secondary",
            Info => "info",
            Success => "success",
            Warning => "warning",
            Danger => "dangerous",
        })
    }
}