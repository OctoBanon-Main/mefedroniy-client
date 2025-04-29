use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use ratatui::prelude::Position;

use crate::app::App;

pub fn render_input(f: &mut Frame, area: Rect, app: &mut App) {
    let input_paragraph = Paragraph::new(app.input.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Ввод (Enter отправить, Esc выйти) ", Style::default().fg(Color::White)))
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: false });
    f.render_widget(input_paragraph, area);

    let inner_width = area.width.saturating_sub(2);
    let cursor_line = (app.cursor_position as u16) / inner_width;
    let cursor_col  = (app.cursor_position as u16) % inner_width;
    f.set_cursor_position(Position {
        x: area.x + 1 + cursor_col,
        y: area.y + 1 + cursor_line,
    });
}

pub fn calculate_input_height(app: &App, available_width: u16) -> u16 {
    let available_input_width = available_width.saturating_sub(4);
    let required_input_lines: u16 = app.input
        .lines()
        .map(|line| {
            let line_len = line.chars().count() as u16;
            if line_len == 0 { 1 } else { ((line_len as f64) / (available_input_width as f64)).ceil() as u16 }
        })
        .sum();
    std::cmp::max(3, required_input_lines + 2)
}