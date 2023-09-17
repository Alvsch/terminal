use std::fmt::{Display, Formatter};
use crossterm::style::Color;

#[derive(PartialOrd, PartialEq)]
pub enum Level {
    Error = 50,
    Warning = 30,
    Info = 10,
    Debug = 5,
    Trace = 0,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Level::Error => "ERROR",
            Level::Warning => "WARNING",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };
        write!(f, "{}", name)
    }
}

impl Level {
    pub fn get_color(&self) -> Color {
        match self {
            Level::Error => Color::Red,
            Level::Warning => Color::Yellow,
            Level::Info => Color::Reset,
            Level::Debug => Color::Reset,
            Level::Trace => Color::Reset,
        }
    }
}
