use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use ratatui::prelude::Position;

use crate::app::App;

pub fn draw_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.area();

        let available_input_width = size.width.saturating_sub(4);
        let required_input_lines: u16 = app.input
            .lines()
            .map(|line| {
                let line_len = line.chars().count() as u16;
                if line_len == 0 { 1 } else { ((line_len as f64) / (available_input_width as f64)).ceil() as u16 }
            })
            .sum();
        let input_height = std::cmp::max(3, required_input_lines + 2);

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

        let chat_chunk = chunks[0];
        let chat_inner_width = chat_chunk.width.saturating_sub(2) as usize;

        let all_lines: Vec<Line> = app.messages
            .iter()
            .flat_map(|msg| msg.to_lines(chat_inner_width))
            .collect();

        let visible_area = chat_chunk.height.saturating_sub(2) as usize;
        let total_lines = all_lines.len();

        if total_lines <= visible_area {
            app.auto_scroll = true;
            app.scroll = 0;
        } else if app.auto_scroll {
            app.scroll = (total_lines - visible_area) as u16;
        } else {
            let max_scroll = total_lines - visible_area;
            app.scroll = app.scroll.min(max_scroll as u16);
        }

        let visible_lines: Vec<Line> = all_lines
            .iter()
            .skip(app.scroll as usize)
            .take(visible_area)
            .cloned()
            .collect();

        let chat = Paragraph::new(Text::from(visible_lines))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(" Чат ", Style::default().fg(Color::White)))
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });
        f.render_widget(chat, chat_chunk);

        let input_paragraph = Paragraph::new(app.input.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(" Ввод (Enter отправить, Esc выйти) ", Style::default().fg(Color::White)))
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .wrap(Wrap { trim: false });
        f.render_widget(input_paragraph, chunks[1]);

        let inner_width = chunks[1].width.saturating_sub(2);
        let cursor_line = (app.cursor_position as u16) / inner_width;
        let cursor_col  = (app.cursor_position as u16) % inner_width;
        f.set_cursor_position(Position {
            x: chunks[1].x + 1 + cursor_col,
            y: chunks[1].y + 1 + cursor_line,
        });
    })?;
    Ok(())
}