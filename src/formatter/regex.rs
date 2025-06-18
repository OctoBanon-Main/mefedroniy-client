use once_cell::sync::Lazy;

use regex::Regex;
use ratatui::style::Color;

pub static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[(.*?)\] (.*)").expect("Invalid regex for DATA_REGEX")
});

pub static COLORED_USERNAMES: Lazy<Vec<(Regex, Color)>> = Lazy::new(|| vec![
    (Regex::new(r"\u{B9AC}\u{3E70}<(.*?)> (.*)").expect("Invalid bRAC regex"), Color::Green),
    (Regex::new(r"\u{2550}\u{2550}\u{2550}<(.*?)> (.*)").expect("Invalid CRAB regex"), Color::LightRed),
    (Regex::new(r"\u{00B0}\u{0298}<(.*?)> (.*)").expect("Invalid Mefedroniy regex"), Color::LightMagenta),
    (Regex::new(r"\u{25B2}<(.*?)> (.*)").expect("Invalid Tower regex"), Color::Rgb(255, 255, 255)),
    (Regex::new(r"\u{253C}<(.*?)> (.*)").expect("Invalid WinRAC regex"), Color::LightGreen),
    (Regex::new(r"<(.*?)> (.*)").expect("Invalid clRAC regex"), Color::Cyan),
]);

pub static BRACES_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{[^}]*\}\s").expect("Invalid BRACES_REGEX")
});

pub static ANSI_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").expect("Invalid ANSI_REGEX")
});

pub static CONTROL_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\x00-\x1F\x7F]").expect("Invalid CONTROL_CHARS_REGEX")
});