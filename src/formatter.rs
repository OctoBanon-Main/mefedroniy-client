use lazy_static::lazy_static;
use ratatui::style::Color;
use regex::Regex;

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"\[(.*?)\] (.*)").unwrap();
    static ref COLORED_USERNAMES: Vec<(Regex, Color)> = vec![
        (Regex::new(r"\u{B9AC}\u{3E70}<(.*?)> (.*)").unwrap(), Color::Green),
        (Regex::new(r"\u{2550}\u{2550}\u{2550}<(.*?)> (.*)").unwrap(), Color::LightRed),
        (Regex::new(r"\u{25B2}<(.*?)> (.*)").unwrap(), Color::Rgb(255, 255, 255)),
        (Regex::new(r"\u{0217}<(.*?)> (.*)").unwrap(), Color::LightGreen),
        (Regex::new(r"\u{00B0}\u{0298}<(.*?)> (.*)").unwrap(), Color::LightMagenta),
        (Regex::new(r"<(.*?)> (.*)").unwrap(), Color::Cyan),
    ];
}

pub struct FormattedMessage {
    pub date: String,
    pub sender: String,
    pub sender_color: Option<Color>,
    pub content: String,
}

pub fn format_message(raw_message: String) -> Option<FormattedMessage> {
    let re_braces = Regex::new(r"\{[^}]*\}\s").unwrap();
    let cleaned = re_braces.replace_all(raw_message.trim(), "");
    let message = sanitize_text(&cleaned);

    let caps = DATE_REGEX.captures(&message)?;
    let (date, rest) = (
        caps.get(1)?.as_str().to_string(),
        caps.get(2)?.as_str()
    );

    let (color, sender, content) = get_username_color(rest);

    Some(FormattedMessage {
        date,
        sender,
        sender_color: Some(color),
        content,
    })
}

pub fn sanitize_text(input: &str) -> String {
    let ansi_regex = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    let control_chars_regex = Regex::new(r"[\x00-\x1F\x7F]").unwrap();
    
    control_chars_regex
        .replace_all(&ansi_regex.replace_all(input, ""), "")
        .into_owned()
}

pub fn get_username_color(message: &str) -> (Color, String, String) {
    COLORED_USERNAMES
        .iter()
        .find_map(|(regex, color)| {
            regex.captures(message).map(|caps| {
                let username = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let text = caps.get(2).map_or("", |m| m.as_str()).to_string();
                (*color, username, text)
            })
        })
        .unwrap_or((Color::White, String::new(), message.to_string()))
}
