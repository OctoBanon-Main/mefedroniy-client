use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn render_chat(f: &mut Frame, area: Rect, app: &mut App) {
    let chat_inner_width = area.width.saturating_sub(2) as usize;

    let all_lines: Vec<Line> = app.messages
        .iter()
        .flat_map(|msg| msg.to_lines(chat_inner_width))
        .collect();

    let visible_area = area.height.saturating_sub(2) as usize;
    let total_lines = all_lines.len();

    let max_scroll = total_lines.saturating_sub(visible_area);

    match (total_lines <= visible_area, app.auto_scroll) {
        (true, _) => {
            app.auto_scroll = true;
            app.scroll = 0;
        }
        (false, true) => {
            app.scroll = max_scroll as u16;
        }
        (false, false) => {
            app.scroll = app.scroll.min(max_scroll as u16);
        }
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
                .title(Span::styled(" Chat History ", Style::default().fg(Color::White)))
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
    f.render_widget(chat, area);
}