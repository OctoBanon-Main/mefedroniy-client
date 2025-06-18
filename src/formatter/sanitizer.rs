use crate::formatter::regex::{ANSI_REGEX, CONTROL_CHARS_REGEX};

pub fn sanitize_text(input: &str) -> String {
    let without_ansi = ANSI_REGEX.replace_all(input, "");
    let sanitized = CONTROL_CHARS_REGEX.replace_all(&without_ansi, "");
    sanitized.into_owned()
}