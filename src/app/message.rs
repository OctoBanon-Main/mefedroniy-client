use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Line, Span};
use textwrap::{wrap, Options};
use unicode_width::UnicodeWidthStr;

pub struct ChatMessage {
    pub timestamp: String,
    pub sender: String,
    pub sender_color: Option<Color>,
    pub content: String,
}

impl ChatMessage {
    pub fn to_lines(&self, max_width: usize) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let indent = "    ";

        let prefix = match &self.sender {
            s if s.is_empty() => format!("{} ", self.timestamp),
            _ => format!("{} {}: ", self.timestamp, self.sender),
        };
        let prefix_width = UnicodeWidthStr::width(prefix.as_str());
        let available_first = max_width.saturating_sub(prefix_width);

        let content_lines: Vec<&str> = self.content.split('\n').collect();
        
        let first_content = match content_lines.first() {
            Some(content) => {
                let mut spans = vec![
                    Span::styled(self.timestamp.clone(), Style::default().fg(Color::DarkGray)),
                    Span::raw(" "),
                ];

                match self.sender.is_empty() {
                    false => spans.extend_from_slice(&[
                        Span::styled(
                            self.sender.clone(),
                            Style::default()
                                .fg(self.sender_color.unwrap_or(Color::White))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(": "),
                    ]),
                    true => (),
                }

                let opts_first = Options::new(available_first).break_words(true);
                let wrapped_first = wrap(content, opts_first);

                match wrapped_first.first() {
                    Some(first_line) => {
                        spans.push(Span::raw(first_line.to_string()));
                        lines.push(Line::from(spans));

                        lines.extend(
                            wrapped_first
                                .iter()
                                .skip(1)
                                .map(|part| Line::from(Span::raw(format!("{}{}", indent, part))))
                        );
                    }
                    None => lines.push(Line::from(spans)),
                }
                true
            }
            None => false,
        };

        match first_content {
            true => {
                let opts = Options::new(max_width).break_words(true);
                lines.extend(
                    content_lines
                        .iter()
                        .skip(1)
                        .flat_map(|content| {
                            let wrapped = wrap(content, opts.clone());
                            match wrapped.is_empty() {
                                true => vec![Line::from(Span::raw(indent.to_string()))],
                                false => wrapped
                                    .into_iter()
                                    .map(|part| Line::from(Span::raw(format!("{}{}", indent, part))))
                                    .collect(),
                            }
                        })
                );
            }
            false => (),
        }

        lines
    }
}