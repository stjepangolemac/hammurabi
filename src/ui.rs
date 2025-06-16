use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};
use tui_big_text::{BigText, PixelSize};
use crate::app::App;
use crate::game::GamePhase;

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
    content.push(Line::from(vec![
        Span::styled(
            format!("HAMMURABI: I BEG TO REPORT TO THEE, IN YEAR {} OF THY REIGN", app.game.year),
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
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    paragraph.render(area, frame.buffer_mut());
}

fn render_input_section<'a>(game: &crate::game::GameState, input_buffer: &'a str, content: &mut Vec<Line<'a>>) {
    use crate::game::GamePhase;
    
    match game.current_phase {
        GamePhase::Splash | GamePhase::Instructions => {
            // These shouldn't be reached as they are handled separately
        }
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
            content.push(Line::from("HOW MANY ACRES DOST THOU WISH TO ACQUIRE?"));
            content.push(Line::from(vec![
                Span::styled(
                    format!("(NEGATIVE TO SELL, THOU CANST BUY: {}, THY HOLDINGS: {})", max_buy, game.land),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
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
                    if max_plant == max_by_pop { "THY WORKERS" }
                    else if max_plant == max_by_grain { "THY GRAIN" }
                    else { "THY LAND" },
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(")", Style::default().fg(Color::DarkGray)),
            ]));
        }
        GamePhase::Feeding => {
            let need = game.grain_needed_for_feeding();
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::raw("HOW MANY BUSHELS SHALL FEED THY PEOPLE?"),
            ]));
            content.push(Line::from(vec![
                Span::styled(
                    format!("(THY PEOPLE REQUIRE: {}, THY STORES HOLD: {})", need, game.grain),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
        GamePhase::YearEnd => {
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    "PRESS ENTER TO CONTINUE THY REIGN...",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                ),
            ]));
        }
        GamePhase::GameOver => {
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    "PRESS ESC TO DEPART THIS MORTAL REALM",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                ),
            ]));
        }
    }

    // Input line
    if !matches!(game.current_phase, GamePhase::Splash | GamePhase::Instructions | GamePhase::YearEnd | GamePhase::GameOver) {
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

fn draw_instructions(frame: &mut Frame, area: Rect) {
    // Split area into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(16), // Instructions
            Constraint::Min(0),     // Spacer
            Constraint::Length(1),  // Press enter
        ])
        .split(area);
    
    // Title
    let title = Paragraph::new("HAMURABI INSTRUCTIONS")
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    title.render(chunks[0], frame.buffer_mut());
    
    // Instructions area - split into 3 items
    let instruction_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // First instruction
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Second instruction  
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Third instruction
            Constraint::Min(0),     // Remaining space
        ])
        .split(chunks[1]);
    
    // Render each instruction with proper alignment
    render_instruction(
        frame,
        instruction_chunks[0],
        "I.",
        vec![
            "HOW MUCH LAND TO BUY OR SELL (LAND COSTS BETWEEN",
            "17 AND 26 BUSHELS OF GRAIN).",
        ],
    );
    
    render_instruction(
        frame,
        instruction_chunks[2],
        "II.",
        vec![
            "HOW MANY BUSHELS TO FEED YOUR PEOPLE (20 PER",
            "PERSON PER YEAR REQUIRED).",
        ],
    );
    
    render_instruction(
        frame,
        instruction_chunks[4],
        "III.",
        vec![
            "HOW MANY ACRES OF LAND TO PLANT SEED IN",
            "(REQUIRES 1 BUSHEL + 1/10TH A PERSON TO TILL PER YEAR).",
        ],
    );
    
    // Press enter prompt
    let prompt = Paragraph::new("PRESS ENTER TO START...")
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center);
    prompt.render(chunks[3], frame.buffer_mut());
}

fn render_instruction(frame: &mut Frame, area: Rect, number: &str, lines: Vec<&str>) {
    // Split the area into number and text columns
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(6),   // Space for "III. "
            Constraint::Min(0),      // Rest for text
        ])
        .split(area);
    
    // Render the number
    let number_text = Paragraph::new(number)
        .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Right);
    number_text.render(chunks[0], frame.buffer_mut());
    
    // Render the instruction text
    let text = lines.join("\n");
    let instruction_text = Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: false });
    instruction_text.render(chunks[1], frame.buffer_mut());
}

fn draw_splash(frame: &mut Frame, area: Rect) {
    // Create layout for the splash screen
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),   // Top text
            Constraint::Length(2),   // Spacer
            Constraint::Length(8),   // Big title
            Constraint::Length(3),   // Subtitle
            Constraint::Min(0),      // Spacer
            Constraint::Length(3),   // Instructions
        ])
        .split(area);

    // Top descriptive text
    let top_text = "IN THIS GAME YOU CAN DIRECT THE ADMINISTRATOR OF THE ANCIENT CITY OF SUMERIA, HAMMURABI. HOW LONG CAN YOU MANAGE YOUR CITY, BECAUSE YOU ARE...";
    
    let top_paragraph = Paragraph::new(top_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    top_paragraph.render(chunks[0], frame.buffer_mut());

    // Create the big text widget
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))
        .lines(vec!["HAMMURABI".into()])
        .centered()
        .build();

    big_text.render(chunks[2], frame.buffer_mut());

    // Add subtitle
    let subtitle_lines = vec![
        Line::from(vec![
            Span::styled("KING OF", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("ANCIENT BABYLONIA", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
    ];
    
    let subtitle = Paragraph::new(subtitle_lines)
        .alignment(Alignment::Center);
    subtitle.render(chunks[3], frame.buffer_mut());
    
    // Instructions at bottom
    let instructions = vec![
        Line::from(vec![
            Span::raw("PRESS "),
            Span::styled("<ENTER>", Style::default().fg(Color::Yellow)),
            Span::raw(" TO BEGIN THE GAME"),
        ]),
        Line::from(vec![
            Span::raw("PRESS "),
            Span::styled("<ESC>", Style::default().fg(Color::Yellow)),
            Span::raw(" TO QUIT"),
        ]),
    ];
    
    let instructions_paragraph = Paragraph::new(instructions)
        .alignment(Alignment::Center);
    instructions_paragraph.render(chunks[5], frame.buffer_mut());
}