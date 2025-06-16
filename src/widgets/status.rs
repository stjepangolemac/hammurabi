use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::game::GameState;

pub fn render_status(game: &GameState, area: Rect, buf: &mut Buffer) {
    let status_text = vec![
        Line::from(vec![
            Span::raw("Population: "),
            Span::styled(
                format!("{}", game.population),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::raw("    Land: "),
            Span::styled(
                format!("{} acres", game.land),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("Grain: "),
            Span::styled(
                format!("{} bushels", game.grain),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let status = Paragraph::new(status_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" HAMMURABI - Year {} ", game.year))
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White));

    status.render(area, buf);
}