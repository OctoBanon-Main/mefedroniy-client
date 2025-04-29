use crate::formatter::regex::{ANSI_REGEX, CONTROL_CHARS_REGEX};

pub fn sanitize_text(input: &str) -> String {
    CONTROL_CHARS_REGEX
        .replace_all(&ANSI_REGEX.replace_all(input, ""), "")
        .into_owned()
}