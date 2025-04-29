use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Block,
    Terminal,
};

use crate::app::App;
use crate::ui::chat::render_chat;
use crate::ui::input::{calculate_input_height, render_input};

mod chat;
mod input;

pub fn draw_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.area();
        
        let input_height = calculate_input_height(app, size.width);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(input_height),
            ].as_ref())
            .split(size);

        let background = Block::default().style(Style::default().bg(Color::Black));
        f.render_widget(background, size);

        render_chat(f, chunks[0], app);
        render_input(f, chunks[1], app);
    })?;
    Ok(())
}