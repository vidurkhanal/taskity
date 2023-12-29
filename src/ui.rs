use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{app::App, constants::PRIMARY_COLOR};

pub fn render(app: &mut App, f: &mut Frame) {
    // f.render_widget(
    //     Paragraph::new(format!(
    //         "
    //     Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //     Press `j` and `k` to increment and decrement the counter respectively.\n\
    //     Counter: {}
    //   ",
    //         app.counter
    //     ))
    //     .block(
    //         Block::default()
    //             .title("Taskity")
    //             .title_alignment(Alignment::Center)
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Magenta))
    //     .alignment(Alignment::Center),
    //     f.size(),

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(PRIMARY_COLOR));

    let title = Paragraph::new(Text::styled("Taskity", Style::default().fg(PRIMARY_COLOR)))
        .block(title_block)
        .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);
}
