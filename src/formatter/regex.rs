use lazy_static::lazy_static;
use regex::Regex;
use ratatui::style::Color;

lazy_static! {
    pub static ref DATE_REGEX: Regex = Regex::new(r"\[(.*?)\] (.*)").unwrap();
    pub static ref COLORED_USERNAMES: Vec<(Regex, Color)> = vec![
        (Regex::new(r"\u{B9AC}\u{3E70}<(.*?)> (.*)").unwrap(), Color::Green),
        (Regex::new(r"\u{2550}\u{2550}\u{2550}<(.*?)> (.*)").unwrap(), Color::LightRed),
        (Regex::new(r"\u{25B2}<(.*?)> (.*)").unwrap(), Color::Rgb(255, 255, 255)),
        (Regex::new(r"\u{253C}<(.*?)> (.*)").unwrap(), Color::LightGreen),
        (Regex::new(r"\u{00B0}\u{0298}<(.*?)> (.*)").unwrap(), Color::LightMagenta),
        (Regex::new(r"<(.*?)> (.*)").unwrap(), Color::Cyan),
    ];
    pub static ref BRACES_REGEX: Regex = Regex::new(r"\{[^}]*\}\s").unwrap();
    pub static ref ANSI_REGEX: Regex = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    pub static ref CONTROL_CHARS_REGEX: Regex = Regex::new(r"[\x00-\x1F\x7F]").unwrap();
}