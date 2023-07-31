use serde::{Serialize, Deserialize};
use clap::ValueEnum;

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone, PartialEq)]
pub enum Color {
    NoColor,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn ansi(&self) -> &'static str {
        match self {
            Color::NoColor      => "\x1b[0m",
            Color::Black        => "\x1b[30m",
            Color::Red          => "\x1b[31m",
            Color::Green        => "\x1b[32m",
            Color::Yellow       => "\x1b[33m",
            Color::Blue         => "\x1b[34m",
            Color::Magenta      => "\x1b[35m",
            Color::Cyan         => "\x1b[36m",
            Color::White        => "\x1b[37m",
            Color::BrightBlack  => "\x1b[90m",
            Color::BrightRed    => "\x1b[91m",
            Color::BrightGreen  => "\x1b[92m",
            Color::BrightYellow => "\x1b[93m",
            Color::BrightBlue   => "\x1b[94m",
            Color::BrightMagenta=> "\x1b[95m",
            Color::BrightCyan   => "\x1b[96m",
            Color::BrightWhite  => "\x1b[97m",
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::NoColor      => "no-color",
            Color::Black        => "black",
            Color::Red          => "red",
            Color::Green        => "green",
            Color::Yellow       => "yellow",
            Color::Blue         => "blue",
            Color::Magenta      => "magenta",
            Color::Cyan         => "cyan",
            Color::White        => "white",
            Color::BrightBlack  => "bright-black",
            Color::BrightRed    => "bright-red",
            Color::BrightGreen  => "bright-green",
            Color::BrightYellow => "bright-yellow",
            Color::BrightBlue   => "bright-blue",
            Color::BrightMagenta=> "bright-magenta",
            Color::BrightCyan   => "bright-cyan",
            Color::BrightWhite  => "bright-white",
        }
    }
}
