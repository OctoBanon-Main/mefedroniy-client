use crossterm::event::Event;
use tokio::sync::mpsc::UnboundedSender;
use crate::app::App;
use super::{keyboard::handle_key_event, mouse::handle_mouse_event};

pub fn process_event(
    event: Event,
    app: &mut App,
    send_tx: &UnboundedSender<(String, String)>,
    username: &str,
) -> bool {
    match event {
        Event::Key(key_event) => handle_key_event(key_event, app, send_tx, username),
        Event::Mouse(mouse_event) => handle_mouse_event(mouse_event, app),
        _ => false,
    }
}