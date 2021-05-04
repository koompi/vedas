/// Weight for Typography
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weight {
    Regular,
    Bold,
    Light,
}

impl Weight {
    pub const ALL: [Weight; 3] = [
        Weight::Regular,
        Weight::Bold,
        Weight::Light
    ];
}

impl Default for Weight {
    fn default() -> Self {
        Self::Regular
    }
}

impl ToString for Weight {
    fn to_string(&self) -> String {
        use Weight::*;

        String::from(match self {
            Regular => "",
            Bold => "bold",
            Light => "light",
        })
    }
}