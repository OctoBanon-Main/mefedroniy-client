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
    f.render_widget(chat, area);
}