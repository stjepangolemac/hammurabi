use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use crate::game::GameState;

pub fn render_history(game: &GameState, events: &[String], area: Rect, buf: &mut Buffer) {
    let mut lines = Vec::new();

    // Add recent events
    for event in events.iter().rev().take(5) {
        lines.push(Line::from(vec![
            Span::raw("• "),
            Span::raw(event),
        ]));
    }

    if !lines.is_empty() {
        lines.push(Line::from(""));
    }

    // Add year history
    for summary in game.history.iter().rev().take(3) {
        let mut year_info = format!("Year {}: Pop {}", summary.year, summary.population);
        
        if summary.starved > 0 {
            year_info.push_str(&format!(", {} starved", summary.starved));
        }
        if summary.plague_deaths > 0 {
            year_info.push_str(&format!(", {} died in plague", summary.plague_deaths));
        }
        if summary.new_citizens > 0 {
            year_info.push_str(&format!(", {} immigrants", summary.new_citizens));
        }

        lines.push(Line::from(vec![
            Span::raw("• "),
            Span::styled(year_info, Style::default().fg(Color::DarkGray)),
        ]));
    }

    let history = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" History ")
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    history.render(area, buf);
}