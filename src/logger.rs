use crossterm::{self, Color, Colored};

#[repr(i16)]
pub enum Verbosity {
    Shout,
    Error,
    Warn,
    Info,
    Trace,
    Custom(CustomVerbosity),
}

pub struct CustomVerbosity {
    level: i16,
    sign: String,
    display_color: Option<Color>,
}

impl Verbosity {
    pub fn level(&self) -> i16 {
        match self {
            Verbosity::Custom(c) => c.level,
            Verbosity::Shout => 80,
            Verbosity::Error => 70,
            Verbosity::Warn => 50,
            Verbosity::Info => 20,
            Verbosity::Trace => 10,
        }
    }

    pub fn sign(&self) -> &str {
        match self {
            Verbosity::Custom(c) => &c.sign,
            Verbosity::Shout => "Shout",
            Verbosity::Error => "Error",
            Verbosity::Warn => "Warn",
            Verbosity::Info => "Info",
            Verbosity::Trace => "Trace",
        }
    }

    pub fn display_color(&self) -> Color {
        match self {
            Verbosity::Custom(CustomVerbosity { display_color, .. }) => match display_color {
                Some(s) => *s,
                None => Color::Blue,
            },
            Verbosity::Shout => Color::Magenta,
            Verbosity::Error => Color::Red,
            Verbosity::Warn => Color::Yellow,
            Verbosity::Info => Color::Cyan,
            Verbosity::Trace => Color::Grey,
        }
    }
}

const LOG_LEFT_WIDTH: usize = 8;

pub fn log<'a>(verbosity: &Verbosity, message: &'a str) {
    println!(
        "{3}{1:<0$}{4} {2}",
        LOG_LEFT_WIDTH,
        verbosity.sign(),
        message,
        Colored::Fg(verbosity.display_color()),
        Colored::Fg(Color::Reset)
    );
}
