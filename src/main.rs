mod app;
mod client;
mod config;
mod formatter;
mod ui;
mod input;

use std::{io, time::Duration};
use crossterm::{
    event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::unbounded_channel;

use app::App;
use client::{receive_messages, send_messages};
use config::{Config, read_input, resolve_address};
use ui::draw_ui;
use input::process_event;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==========================================");
    println!("            Mefedroniy Client             ");
    println!("==========================================\n");

    let mut config = Config::load();

    println!("Сохранённые серверы:");
    for (i, server) in config.servers.iter().enumerate() {
        println!("{}: {}", i + 1, server);
    }
    println!("0: Ввести новый сервер");

    let server_choice: usize = read_input("Выберите сервер: ")?
        .parse()
        .unwrap_or(0);

    let server_addr = match server_choice {
        n if n > 0 && n <= config.servers.len() => config.servers[n - 1].clone(),
        _ => {
            let server_host = read_input("Введите IP/домен сервера: ")?;
            let server_port = read_input("Введите порт сервера: ")?;
            let addr = resolve_address(&server_host, &server_port)
                .await
                .unwrap_or_else(|| format!("{}:{}", server_host, server_port));
            config.add_server(addr.clone());
            addr
        }
    };

    config.username = match config.username.as_str() {
        "" => read_input("Введите имя пользователя: ")?,
        username => username.to_string(),
    };

    config.update_interval = match config.update_interval {
        5 => read_input("Введите интервал обновления (секунд): ")?
            .parse()
            .unwrap_or(5),
        interval => interval,
    };
    config.save();

    let (chat_tx, mut chat_rx) = unbounded_channel::<String>();
    let (send_tx, send_rx) = unbounded_channel::<(String, String)>();

    tokio::spawn(receive_messages(
        server_addr.clone(),
        config.update_interval,
        chat_tx,
    ));
    tokio::spawn(send_messages(server_addr, send_rx));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        while let Ok(chat) = chat_rx.try_recv() {
            let new_messages = chat.lines().map(String::from).collect();
            app.add_messages(new_messages);
        }

        draw_ui(&mut terminal, &mut app)?;

        match event::poll(Duration::from_millis(50))? {
            true => match event::read()? {
                evt => match process_event(evt, &mut app, &send_tx, &config.username) {
                    true => break,
                    false => continue,
                }
            },
            false => continue,
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
