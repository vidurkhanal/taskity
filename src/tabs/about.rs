use ratatui::{
    prelude::{Alignment, Buffer, Direction, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};

use crate::{constants, root::layout};

pub struct AboutTab {
    selected_row: usize,
}

const TASKITY_LOGO: [&str; 6] = [
    r" _____ ____ ____ _  ___ ________  _",
    r"/__ __/  _ / ___/ |/ / /__ __\  \//",
    r"  / \ | / \|    |   /| | / \  \  / ",
    r"  | | | |-|\___ |   \| | | |  / /  ",
    r"  \_/ \_/ \\____\_|\_\_/ \_/ /_/   ",
    r"                                   ",
];

impl AboutTab {
    pub fn new(selected_row: usize) -> Self {
        Self { selected_row }
    }
}

impl Widget for AboutTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Vertical, vec![8, 50]);
        render_description(area[1], buf);
        render_logo(area[0], buf);
    }
}

fn render_description(area: Rect, buf: &mut Buffer) {
    let area = area.inner(
        &(Margin {
            vertical: 2,
            horizontal: 2,
        }),
    );
    Clear.render(area, buf);
    Block::new()
        .style(
            Style::new()
                .bg(constants::DARK_BLUE)
                .fg(constants::LIGHT_GRAY),
        )
        .render(area, buf);
    let area = area.inner(
        &(Margin {
            vertical: 1,
            horizontal: 10,
        }),
    );
    let text = "- Visualize and Control your processes and ports -

    Takity is a modern version of top/htop and lsof written in Rust. It is a terminal-based system monitoring dashboard for Linux and macOS. It provides a way to visualize and control your processes and ports.

    Developed by @vidurkhanal [Github, Linkedin], @vidurkl [Twitter/X]
";

    Paragraph::new(text)
        .style(
            Style::new()
                .fg(constants::LIGHT_GRAY)
                .bg(constants::DARK_BLUE)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::new())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .scroll((0, 0))
        .render(area, buf);
}

fn render_logo(area: Rect, buf: &mut Buffer) {
    Clear.render(area, buf);
    Block::new()
        .style(
            Style::new()
                .bg(constants::DARK_BLUE)
                .fg(constants::LIGHT_GRAY),
        )
        .render(area, buf);
    let mut y = area.y + 1;

    for line in TASKITY_LOGO.iter() {
        Paragraph::new(*line)
            .style(
                Style::new()
                    .bg(constants::DARK_BLUE)
                    .fg(constants::LIGHT_GRAY),
            )
            .alignment(Alignment::Center)
            .render(Rect::new(area.x, y, area.width, 1), buf);
        y += 1;
    }
}
