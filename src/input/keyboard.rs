use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use tokio::sync::mpsc::UnboundedSender;
use crate::app::App;

pub fn handle_key_event(
    key_event: KeyEvent,
    app: &mut App,
    send_tx: &UnboundedSender<(String, String)>,
    username: &str,
) -> bool {
    if key_event.kind != KeyEventKind::Press {
        return false;
    }
    
    match key_event.code {
        KeyCode::Char(c) => {
            let mut chars: Vec<char> = app.input.chars().collect();
            chars.insert(app.cursor_position, c);
            app.input = chars.into_iter().collect();
            app.cursor_position += 1;
            false
        }
        KeyCode::Backspace => {
            match app.cursor_position {
                0 => (),
                _ => {
                    let mut chars: Vec<char> = app.input.chars().collect();
                    chars.remove(app.cursor_position - 1);
                    app.input = chars.into_iter().collect();
                    app.cursor_position -= 1;
                }
            }
            false
        }
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
            false
        }
        KeyCode::Left => {
            match app.cursor_position {
                0 => (),
                _ => app.cursor_position -= 1,
            }
            false
        }
        KeyCode::Right => {
            match app.cursor_position.cmp(&app.input.chars().count()) {
                std::cmp::Ordering::Less => app.cursor_position += 1,
                _ => (),
            }
            false
        }
        KeyCode::Esc => true,
        KeyCode::PageUp => {
            match app.scroll {
                0 => (),
                _ => app.scroll = app.scroll.saturating_sub(1),
            }
            app.auto_scroll = false;
            false
        }
        KeyCode::PageDown => {
            app.scroll = app.scroll.saturating_add(1);
            app.auto_scroll = false;
            false
        }
        _ => false,
    }
}