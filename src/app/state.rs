use crate::app::message::ChatMessage;
use crate::formatter::format_message;

pub struct App {
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub scroll: u16,
    pub auto_scroll: bool,
    pub cursor_position: usize,
    pub error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            scroll: 0,
            auto_scroll: true,
            cursor_position: 0,
            error_message: None,
        }
    }

    pub fn add_messages(&mut self, new_messages: Vec<String>) {
        self.messages.extend(
            new_messages
                .into_iter()
                .filter_map(format_message)
                .map(|formatted| ChatMessage {
                    timestamp: formatted.date,
                    sender: formatted.sender,
                    sender_color: formatted.sender_color,
                    content: formatted.content,
                })
        );
    }
}