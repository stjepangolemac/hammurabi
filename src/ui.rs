use crate::app::App;
use crate::game::GamePhase;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

const MAX_WIDTH: u16 = 80;
const MAX_HEIGHT: u16 = 24;

fn centered_rect(max_width: u16, max_height: u16, r: Rect) -> Rect {
    let width = r.width.min(max_width);
    let height = r.height.min(max_height);

    let horizontal_margin = (r.width.saturating_sub(width)) / 2;
    let vertical_margin = (r.height.saturating_sub(height)) / 2;

    let area = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height),
            Constraint::Length(vertical_margin),
        ])
        .split(r)[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Length(horizontal_margin),
            Constraint::Length(width),
            Constraint::Length(horizontal_margin),
        ])
        .split(area)[1]
}

pub fn draw(frame: &mut Frame, app: &App) {
    // Create centered area with max 80x24 dimensions
    let area = centered_rect(MAX_WIDTH, MAX_HEIGHT, frame.area());

    // Handle splash screen
    if matches!(app.game.current_phase, GamePhase::Splash) {
        draw_splash(frame, area);
        return;
    }

    // Handle instructions screen separately
    if matches!(app.game.current_phase, GamePhase::Instructions) {
        draw_instructions(frame, area);
        return;
    }

    let mut content = Vec::new();

    // Title and year
    content.push(Line::from(vec![Span::styled(
        format!(
            "HAMMURABI: I BEG TO REPORT TO THEE, IN YEAR {} OF THY REIGN",
            app.game.year
        ),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(""));

    // Status bar
    content.push(Line::from(vec![
        Span::raw("BUSHELS: "),
        Span::styled(
            format!("{}", app.game.grain),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("  ACRES: "),
        Span::styled(
            format!("{}", app.game.land),
            Style::default().fg(Color::Green),
        ),
        Span::raw("  PEOPLE: "),
        Span::styled(
            format!("{}", app.game.population),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("  YEAR: "),
        Span::styled(
            format!("{}", app.game.year),
            Style::default().fg(Color::White),
        ),
    ]));
    content.push(Line::from(""));

    // Event messages or game content
    if matches!(
        app.game.current_phase,
        GamePhase::YearEnd | GamePhase::GameOver
    ) {
        // Show events
        for msg in &app.event_messages {
            let color = if msg.contains("died") || msg.contains("starved") || msg.contains("plague")
            {
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
        content.push(Line::from(vec![Span::styled(
            format!("! {}", app.message),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]));
    }

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    paragraph.render(area, frame.buffer_mut());
}

fn render_input_section<'a>(
    game: &crate::game::GameState,
    input_buffer: &'a str,
    content: &mut Vec<Line<'a>>,
) {
    use crate::game::GamePhase;

    match game.current_phase {
        GamePhase::Splash | GamePhase::Instructions => {
            // These shouldn't be reached as they are handled separately
        }
        GamePhase::LandTransaction => {
            let max_buy = if game.land_price > 0 {
                game.grain / game.land_price
            } else {
                0
            };
            content.push(Line::from(vec![
                Span::raw("LAND IS TRADING AT "),
                Span::styled(
                    format!("{}", game.land_price),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" BUSHELS PER ACRE."),
            ]));
            content.push(Line::from(""));
            content.push(Line::from("HOW MANY ACRES DOST THOU WISH TO ACQUIRE?"));
            content.push(Line::from(vec![Span::styled(
                format!(
                    "(NEGATIVE TO SELL, THOU CANST BUY: {}, THY HOLDINGS: {})",
                    max_buy, game.land
                ),
                Style::default().fg(Color::DarkGray),
            )]));
        }
        GamePhase::Planting => {
            let max_plant = game.max_plantable_acres();
            let max_by_pop = game.population * 10;
            let max_by_grain = game.grain;
            let _max_by_land = game.land;

            content.push(Line::from(""));
            content.push(Line::from("HOW MANY ACRES WILT THOU PLANT WITH SEED?"));
            content.push(Line::from(vec![
                Span::styled(
                    format!("(THY LIMIT: {} - CONSTRAINED BY ", max_plant),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    if max_plant == max_by_pop {
                        "THY WORKERS"
                    } else if max_plant == max_by_grain {
                        "THY GRAIN"
                    } else {
                        "THY LAND"
                    },
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(")", Style::default().fg(Color::DarkGray)),
            ]));
        }
        GamePhase::Feeding => {
            let need = game.grain_needed_for_feeding();
            content.push(Line::from(""));
            content.push(Line::from(vec![Span::raw(
                "HOW MANY BUSHELS SHALL FEED THY PEOPLE?",
            )]));
            content.push(Line::from(vec![Span::styled(
                format!(
                    "(THY PEOPLE REQUIRE: {}, THY STORES HOLD: {})",
                    need, game.grain
                ),
                Style::default().fg(Color::DarkGray),
            )]));
        }
        GamePhase::YearEnd => {
            content.push(Line::from(""));
            content.push(Line::from(vec![Span::styled(
                "PRESS ENTER TO CONTINUE THY REIGN...",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )]));
        }
        GamePhase::GameOver => {
            content.push(Line::from(""));
            content.push(Line::from(vec![Span::styled(
                "PRESS ESC TO DEPART THIS MORTAL REALM",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )]));
        }
    }

    // Input line
    if !matches!(
        game.current_phase,
        GamePhase::Splash | GamePhase::Instructions | GamePhase::YearEnd | GamePhase::GameOver
    ) {
        content.push(Line::from(vec![
            Span::raw("? "),
            Span::styled(input_buffer, Style::default().fg(Color::Green)),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::RAPID_BLINK),
            ),
        ]));
    }
}

fn draw_instructions(frame: &mut Frame, area: Rect) {
    // Use responsive padding based on terminal size
    let padding = if frame.area().width >= 80 && frame.area().height >= 24 {
        1
    } else {
        0
    };
    
    // Split area into sections with responsive padding
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(padding)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(16), // Instructions
            Constraint::Min(0),     // Spacer
            Constraint::Length(1),  // Press enter
        ])
        .split(area);

    // Title
    let title = Paragraph::new("HAMURABI INSTRUCTIONS")
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());

    // Instructions text
    let instructions = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("I. ", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::styled("HOW MUCH LAND TO BUY OR SELL (LAND COSTS BETWEEN 17 AND 26 BUSHELS OF", Style::default().fg(Color::White)),
        ]),
        Line::from("   GRAIN)."),
        Line::from(""),
        Line::from(vec![
            Span::styled("II. ", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::styled("HOW MANY BUSHELS TO FEED YOUR PEOPLE (20 PER PERSON PER YEAR", Style::default().fg(Color::White)),
        ]),
        Line::from("    REQUIRED)."),
        Line::from(""),
        Line::from(vec![
            Span::styled("III. ", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::styled("HOW MANY ACRES OF LAND TO PLANT SEED IN (REQUIRES 1 BUSHEL +", Style::default().fg(Color::White)),
        ]),
        Line::from("     1/10TH A PERSON TO TILL PER YEAR)."),
    ];
    
    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });
    instructions_paragraph.render(chunks[1], frame.buffer_mut());

    // Press enter prompt
    let prompt = Paragraph::new("PRESS ENTER TO START...")
        .style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )
        .alignment(Alignment::Center);
    prompt.render(chunks[3], frame.buffer_mut());
}

fn draw_splash(frame: &mut Frame, area: Rect) {
    // Use responsive padding based on terminal size
    let padding = if frame.area().width >= 80 && frame.area().height >= 24 {
        1
    } else {
        0
    };
    
    // Determine title height based on terminal size
    let title_height = if frame.area().width < 80 || frame.area().height < 24 {
        4  // Quadrant size needs less height
    } else {
        8  // Full size needs more height
    };
    
    // Create layout for the content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(padding)
        .constraints([
            Constraint::Length(1), // Top decoration
            Constraint::Length(3), // Top text
            Constraint::Length(title_height), // Big title
            Constraint::Length(3), // Subtitle
            Constraint::Length(1), // Bottom decoration
            Constraint::Min(0),    // Spacer
            Constraint::Length(3), // Instructions
        ])
        .split(area);

    // Top decoration
    let decoration_line = "═══════════════════════════════════════════════════════════════";
    let decoration_paragraph = Paragraph::new(decoration_line)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    decoration_paragraph.render(chunks[0], frame.buffer_mut());

    // Top descriptive text
    let top_text =
        "ASSUME THE THRONE OF ANCIENT BABYLON\nRULE WITH WISDOM AND JUSTICE AS THE MIGHTY...";

    let top_paragraph = Paragraph::new(top_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    top_paragraph.render(chunks[1], frame.buffer_mut());

    // Create the big text widget - use smaller size for smaller terminals
    let pixel_size = if frame.area().width < 80 || frame.area().height < 24 {
        PixelSize::Quadrant
    } else {
        PixelSize::Full
    };
    
    let big_text = BigText::builder()
        .pixel_size(pixel_size)
        .style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )
        .lines(vec!["HAMMURABI".into()])
        .centered()
        .build();

    big_text.render(chunks[2], frame.buffer_mut());

    // Add subtitle with decorations
    let subtitle_lines = vec![
        Line::from(vec![Span::styled(
            "KING OF ANCIENT BABYLONIA",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Guide thy kingdom through ten years of tribulation",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )]),
    ];

    let subtitle = Paragraph::new(subtitle_lines).alignment(Alignment::Center);
    subtitle.render(chunks[3], frame.buffer_mut());

    // Bottom decoration
    let bottom_decoration_paragraph = Paragraph::new(decoration_line)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    bottom_decoration_paragraph.render(chunks[4], frame.buffer_mut());

    // Instructions at bottom
    let instructions = vec![
        Line::from(vec![
            Span::raw("PRESS "),
            Span::styled(
                "<ENTER>",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" TO ASCEND THE THRONE"),
        ]),
        Line::from(vec![
            Span::raw("PRESS "),
            Span::styled(
                "<ESC>",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" TO FLEE THY DESTINY"),
        ]),
    ];

    let instructions_paragraph = Paragraph::new(instructions).alignment(Alignment::Center);
    instructions_paragraph.render(chunks[6], frame.buffer_mut());
}
