use crate::formatter::regex::{ANSI_REGEX, CONTROL_CHARS_REGEX, AVATAR_REGEX};

pub fn sanitize_text(input: &str) -> String {
    let sanitized = ANSI_REGEX.replace_all(input, "");
    let sanitized = AVATAR_REGEX.replace_all(&sanitized, "");
    let sanitized = CONTROL_CHARS_REGEX.replace_all(&sanitized, "");
    sanitized.into_owned()
}