use crossterm::event::{MouseEvent, MouseEventKind};
use crate::app::App;

pub fn handle_mouse_event(
    mouse_event: MouseEvent,
    app: &mut App,
) -> bool {
    match mouse_event.kind {
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
    }
    false
}