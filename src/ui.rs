use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use crate::app::App;
use crate::widgets::{render_status, render_input, render_history};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),     // Status
            Constraint::Length(8),     // Input
            Constraint::Min(5),        // History
            Constraint::Length(3),     // Help
        ])
        .split(frame.area());

    // Render status
    render_status(&app.game, chunks[0], frame.buffer_mut());

    // Render input area
    render_input(&app.game, &app.input_buffer, chunks[1], frame.buffer_mut());

    // Render history
    render_history(&app.game, &app.event_messages, chunks[2], frame.buffer_mut());

    // Render help
    render_help(chunks[3], frame.buffer_mut());

    // Render message if any
    if !app.message.is_empty() {
        render_message(&app.message, frame.area(), frame.buffer_mut());
    }
}

fn render_help(area: Rect, buf: &mut ratatui::buffer::Buffer) {
    let help_text = vec![
        Line::from(vec![
            Span::raw("Enter: "),
            Span::styled("Confirm", Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::raw("Backspace: "),
            Span::styled("Delete", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::raw("Esc: "),
            Span::styled("Quit", Style::default().fg(Color::Red)),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .style(Style::default().fg(Color::DarkGray)),
        );

    help.render(area, buf);
}

fn render_message(message: &str, area: Rect, buf: &mut ratatui::buffer::Buffer) {
    let popup_area = centered_rect(50, 20, area);
    
    let message_widget = Paragraph::new(message)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Error ")
                .style(Style::default().fg(Color::Red)),
        )
        .style(Style::default().fg(Color::White));

    message_widget.render(popup_area, buf);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}