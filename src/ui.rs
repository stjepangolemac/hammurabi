use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};
use crate::app::App;
use crate::game::GamePhase;

pub fn draw(frame: &mut Frame, app: &App) {
    // Single column layout with spacing
    let area = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(0)])
        .split(frame.area())[0];

    let mut content = Vec::new();

    // Title and year
    content.push(Line::from(vec![
        Span::styled(
            format!("HAMURABI: I BEG TO REPORT TO YOU, IN YEAR {}", app.game.year),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ]));
    content.push(Line::from(""));

    // Status bar
    content.push(Line::from(vec![
        Span::raw("BUSHELS:"),
        Span::styled(format!("{}", app.game.grain), Style::default().fg(Color::Yellow)),
        Span::raw(" ACRES:"),
        Span::styled(format!("{}", app.game.land), Style::default().fg(Color::Green)),
        Span::raw(" PEOPLE:"),
        Span::styled(format!("{}", app.game.population), Style::default().fg(Color::Cyan)),
        Span::raw(format!("                    YEAR:{}", app.game.year)),
    ]));
    content.push(Line::from(""));

    // Event messages or game content
    if matches!(app.game.current_phase, GamePhase::YearEnd | GamePhase::GameOver) {
        // Show events
        for msg in &app.event_messages {
            let color = if msg.contains("died") || msg.contains("starved") || msg.contains("plague") {
                Color::Red
            } else if msg.contains("came to") || msg.contains("Harvest") {
                Color::Green
            } else if msg.contains("Rats") {
                Color::Magenta
            } else {
                Color::White
            };

            content.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(msg.clone(), Style::default().fg(color)),
            ]));
        }
        content.push(Line::from(""));
    }

    // Input section
    render_input_section(&app.game, &app.input_buffer, &mut content);

    // Error message
    if !app.message.is_empty() {
        content.push(Line::from(""));
        content.push(Line::from(vec![
            Span::styled(
                format!("! {}", app.message),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    paragraph.render(area, frame.buffer_mut());
}

fn render_input_section<'a>(game: &crate::game::GameState, input_buffer: &'a str, content: &mut Vec<Line<'a>>) {
    use crate::game::GamePhase;
    
    match game.current_phase {
        GamePhase::LandTransaction => {
            let max_buy = if game.land_price > 0 { game.grain / game.land_price } else { 0 };
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::raw("LAND IS TRADING AT "),
                Span::styled(
                    format!("{}", game.land_price),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" BUSHELS PER ACRE."),
            ]));
            content.push(Line::from(""));
            content.push(Line::from("HOW MANY ACRES DO YOU WISH TO BUY?"));
            content.push(Line::from(vec![
                Span::styled(
                    format!("(NEGATIVE TO SELL, MAX BUY: {}, YOU OWN: {})", max_buy, game.land),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
        GamePhase::Planting => {
            let max_plant = game.max_plantable_acres();
            let max_by_pop = game.population * 10;
            let max_by_grain = game.grain;
            let max_by_land = game.land;
            
            content.push(Line::from(""));
            content.push(Line::from("HOW MANY ACRES DO YOU WISH TO PLANT WITH SEED?"));
            content.push(Line::from(vec![
                Span::styled(
                    format!("(MAX: {} - LIMITED BY ", max_plant),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    if max_plant == max_by_pop { "WORKERS" }
                    else if max_plant == max_by_grain { "GRAIN" }
                    else { "LAND" },
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(")", Style::default().fg(Color::DarkGray)),
            ]));
        }
        GamePhase::Feeding => {
            let need = game.grain_needed_for_feeding();
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::raw("HOW MANY BUSHELS DO YOU WISH TO FEED YOUR PEOPLE?"),
            ]));
            content.push(Line::from(vec![
                Span::styled(
                    format!("(NEED: {} FOR ALL, HAVE: {})", need, game.grain),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
        GamePhase::YearEnd => {
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    "PRESS ENTER TO CONTINUE...",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                ),
            ]));
        }
        GamePhase::GameOver => {
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    "PRESS ESC TO EXIT",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                ),
            ]));
        }
    }

    // Input line
    if !matches!(game.current_phase, GamePhase::YearEnd | GamePhase::GameOver) {
        content.push(Line::from(vec![
            Span::raw("? "),
            Span::styled(input_buffer, Style::default().fg(Color::Green)),
            Span::styled(
                "_",
                Style::default().fg(Color::Green).add_modifier(Modifier::RAPID_BLINK),
            ),
        ]));
    }
}