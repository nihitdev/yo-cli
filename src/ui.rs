use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

const ART: &str = r#"
██╗   ██╗ ██████╗  ██████╗
╚██╗ ██╔╝██╔═══██╗██╔═══██╗
 ╚████╔╝ ██║   ██║██║   ██║
  ╚██╔╝  ██║   ██║██║   ██║
   ██║   ╚██████╔╝╚██████╔╝
   ╚═╝    ╚═════╝  ╚═════╝
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Neon,
    Ocean,
    Mono,
}

impl Theme {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "neon" => Some(Self::Neon),
            "ocean" => Some(Self::Ocean),
            "mono" => Some(Self::Mono),
            _ => None,
        }
    }

    fn accent(self) -> &'static str {
        match self {
            Self::Neon => "95",
            Self::Ocean => "96",
            Self::Mono => "37",
        }
    }

    fn muted(self) -> &'static str {
        match self {
            Self::Neon => "92",
            Self::Ocean => "94",
            Self::Mono => "90",
        }
    }
}

pub struct Ui {
    theme: Theme,
    use_colour: bool,
    typing_delay: Duration,
}

impl Ui {
    pub fn new(theme: Theme, use_colour: bool, typing_speed_ms: u64) -> Self {
        Self {
            theme,
            use_colour,
            typing_delay: Duration::from_millis(typing_speed_ms),
        }
    }

    pub fn print_art(&self) -> io::Result<()> {
        self.write_line(&self.paint(ART.trim_end(), self.theme.accent(), true))
    }

    pub fn heading(&self, text: &str) -> io::Result<()> {
        self.write_line(&self.paint(text, self.theme.accent(), true))
    }

    pub fn divider(&self) -> io::Result<()> {
        self.write_line(&self.paint(
            "────────────────────────────────────────",
            self.theme.muted(),
            false,
        ))
    }

    pub fn info(&self, icon: &str, label: &str, value: &str) -> io::Result<()> {
        let key = self.paint(&format!("{icon} {label}"), self.theme.accent(), true);
        self.write_line(&format!("{key} {value}"))
    }

    pub fn type_line(&self, text: &str) -> io::Result<()> {
        if self.typing_delay.is_zero() {
            return self.write_line(text);
        }

        let stdout = io::stdout();
        let mut output = stdout.lock();

        for character in text.chars() {
            write!(output, "{character}")?;
            output.flush()?;
            thread::sleep(self.typing_delay);
        }

        writeln!(output)?;
        Ok(())
    }

    pub fn blank_line(&self) -> io::Result<()> {
        self.write_line("")
    }

    fn paint(&self, text: &str, colour: &str, bold: bool) -> String {
        if !self.use_colour {
            return text.to_owned();
        }

        let weight = if bold { "1;" } else { "" };
        format!("\x1b[{weight}{colour}m{text}\x1b[0m")
    }

    fn write_line(&self, text: &str) -> io::Result<()> {
        let stdout = io::stdout();
        let mut output = stdout.lock();
        writeln!(output, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_themes() {
        assert_eq!(Theme::parse("neon"), Some(Theme::Neon));
        assert_eq!(Theme::parse("OCEAN"), Some(Theme::Ocean));
        assert_eq!(Theme::parse("mono"), Some(Theme::Mono));
        assert_eq!(Theme::parse("unknown"), None);
    }
}
