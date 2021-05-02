/// Button HTML Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    Default,
    Blank,
    Parent,
    Top,
    Iframe(&'static str)
}

impl Target {
    pub const ALL: [Target; 5] = [
        Target::Default,
        Target::Blank,
        Target::Parent,
        Target::Top,
        Target::Iframe("")
    ];
}

impl ToString for Target {
    fn to_string(&self) -> String {
        use Target::*;

        String::from(match self {
            Default => "_self",
            Blank => "_blank",
            Parent => "_parent",
            Top => "_top",
            Iframe(s) => s
        })
    }
}