use crossterm::event::{Event, KeyCode, MouseEventKind};
use tokio::sync::mpsc::UnboundedSender;
use crate::app::App;

pub fn process_event(
    event: Event,
    app: &mut App,
    send_tx: &UnboundedSender<(String, String)>,
    username: &str,
) -> bool {
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char(c) => {
                let mut chars: Vec<char> = app.input.chars().collect();
                chars.insert(app.cursor_position, c);
                app.input = chars.into_iter().collect();
                app.cursor_position += 1;
            }
            KeyCode::Backspace => match app.cursor_position {
                0 => (),
                _ => {
                    let mut chars: Vec<char> = app.input.chars().collect();
                    chars.remove(app.cursor_position - 1);
                    app.input = chars.into_iter().collect();
                    app.cursor_position -= 1;
                }
            },
            KeyCode::Enter => {
                match app.input.is_empty() {
                    false => {
                        let full_msg = app.input.clone();
                        let _ = send_tx.send((username.to_string(), full_msg));
                        app.input.clear();
                        app.cursor_position = 0;
                        app.auto_scroll = true;
                    }
                    true => (),
                }
            }
            KeyCode::Left => match app.cursor_position {
                0 => (),
                _ => app.cursor_position -= 1,
            },
            KeyCode::Right => match app.cursor_position.cmp(&app.input.chars().count()) {
                std::cmp::Ordering::Less => app.cursor_position += 1,
                _ => (),
            },
            KeyCode::Esc => return true,
            KeyCode::PageUp => {
                match app.scroll {
                    0 => (),
                    _ => app.scroll = app.scroll.saturating_sub(1),
                }
                app.auto_scroll = false;
            }
            KeyCode::PageDown => {
                app.scroll = app.scroll.saturating_add(1);
                app.auto_scroll = false;
            }
            _ => (),
        },
        Event::Mouse(mouse_event) => match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                match app.scroll {
                    0 => (),
                    _ => app.scroll = app.scroll.saturating_sub(1),
                }
                app.auto_scroll = false;
            }
            MouseEventKind::ScrollDown => {
                app.scroll = app.scroll.saturating_add(1);
                app.auto_scroll = false;
            }
            _ => (),
        },
        _ => (),
    }
    false
}