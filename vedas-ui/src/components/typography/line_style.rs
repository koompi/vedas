/// Line Style for Typography
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Underline,
    Crossline,
}

impl LineStyle {
    pub const ALL: [LineStyle; 2] = [
        LineStyle::Underline,
        LineStyle::Crossline
    ];
}

impl ToString for LineStyle {
    fn to_string(&self) -> String {
        use LineStyle::*;

        String::from(match self {
            Underline => "u",
            Crossline => "del",
        })
    }
}