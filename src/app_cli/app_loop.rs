use std::time::Duration;
use crossterm::event;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::app::App;
use crate::ui::draw_ui;
use crate::input::process_event;
use crate::app_cli::setup::terminal::MefTerminal;

pub async fn run_app_loop(
    terminal: &mut MefTerminal,
    mut chat_rx: UnboundedReceiver<String>,
    send_tx: UnboundedSender<(String, String)>,
    username: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    loop {
        while let Ok(chat) = chat_rx.try_recv() {
            let new_messages = chat.lines().map(String::from).collect();
            app.add_messages(new_messages);
        }

        draw_ui(terminal, &mut app)?;

        match event::poll(Duration::from_millis(50))? {
            true => match event::read()? {
                evt => match process_event(evt, &mut app, &send_tx, &username) {
                    true => break,
                    false => continue,
                }
            },
            false => continue,
        }
    }

    Ok(())
}