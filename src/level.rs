use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Level {
    Novice,
    Intermediate,
    Expert,
}

impl Level {
    pub fn new(level: i32) -> Self {
        match level {
            1 => Level::Novice,
            2 => Level::Intermediate,
            3 => Level::Expert,
            _ => Level::Novice, // Varsayılan olarak Novice
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let level_str = match *self {
            Level::Novice => "Novice",
            Level::Intermediate => "Intermediate",
            Level::Expert => "Expert",
        };
        write!(f, "{}", level_str)
    }
}
