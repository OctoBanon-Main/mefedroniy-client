use ratatui::style::Color;
use crate::formatter::regex::{DATE_REGEX, COLORED_USERNAMES, BRACES_REGEX};
use crate::formatter::sanitizer::sanitize_text;

pub struct FormattedMessage {
    pub date: String,
    pub sender: String,
    pub sender_color: Option<Color>,
    pub content: String,
}

pub fn format_message(raw_message: String) -> Option<FormattedMessage> {
    let cleaned = BRACES_REGEX.replace_all(raw_message.trim(), "");
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