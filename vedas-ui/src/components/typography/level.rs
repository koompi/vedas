/// Title Level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    H1,
    H2,
    H3,
    H4,
    H5
}

impl Level {
    pub const ALL: [Level; 5] = [
        Level::H1,
        Level::H2,
        Level::H3,
        Level::H4,
        Level::H5,
    ];
}

impl Default for Level {
    fn default() -> Self {
        Self::H1
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        use Level::*;

        String::from(match self {
            H1 => "h1",
            H2 => "h2",
            H3 => "h3",
            H4 => "h4",
            H5 => "h5",
        })
    }
}