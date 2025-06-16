use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crate::game::{GameState, GamePhase};

pub fn render_input(game: &GameState, input_buffer: &str, area: Rect, buf: &mut Buffer) {
    let (phase_text, prompt_text) = match game.current_phase {
        GamePhase::LandTransaction => (
            "Buy/Sell Land",
            format!("Land Price: {} bushels/acre\nEnter amount (negative to sell): ", game.land_price),
        ),
        GamePhase::Planting => (
            "Plant Acres",
            format!(
                "Max plantable: {} acres\nHow many acres to plant? ",
                game.max_plantable_acres()
            ),
        ),
        GamePhase::Feeding => (
            "Feed Population",
            format!(
                "Need {} bushels to feed everyone\nHow many bushels for food? ",
                game.grain_needed_for_feeding()
            ),
        ),
        GamePhase::YearEnd => ("Year Summary", "Press Enter to continue...".to_string()),
        GamePhase::GameOver => ("Game Over", "Press Esc to exit".to_string()),
    };

    let mut lines = vec![
        Line::from(vec![
            Span::raw("Current Phase: "),
            Span::styled(
                phase_text,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];

    for line in prompt_text.lines() {
        lines.push(Line::from(line));
    }

    if !matches!(game.current_phase, GamePhase::YearEnd | GamePhase::GameOver) {
        lines.push(Line::from(vec![
            Span::raw("> "),
            Span::styled(
                input_buffer,
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                "_",
                Style::default().fg(Color::Green).add_modifier(Modifier::RAPID_BLINK),
            ),
        ]));
    }

    let input = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Input ")
                .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        )
        .style(Style::default().fg(Color::White));

    input.render(area, buf);
}