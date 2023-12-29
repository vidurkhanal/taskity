use ratatui::{
    prelude::{Alignment, Buffer, Direction, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};

use crate::{constants, root::layout};
pub struct ProcessesTab {
    selected_row: usize,
}

impl ProcessesTab {
    pub fn new(selected_row: usize) -> Self {
        Self { selected_row }
    }
}

impl Widget for ProcessesTab {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
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
        let text = "Hello from the Processes tab!
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
}
