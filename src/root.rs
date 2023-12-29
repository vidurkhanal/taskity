use crate::{
    constants,
    tabs::{AboutTab, PortsTab, ProcessesTab},
};
use std::rc::Rc;

use crate::app::AppContext;
use ratatui::{prelude::*, widgets::*};

pub struct Root<'a> {
    context: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self { context }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .style(Style::new().bg(constants::DARK_BLUE))
            .render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_title_bar(area[0], buf);
        self.render_selected_tab(area[1], buf);
        self.render_bottom_bar(area[2], buf);
    }
}

impl Root<'_> {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 45]);

        Paragraph::new(Span::styled(
            "Taskity",
            Style::new()
                .fg(constants::WHITE)
                .bg(constants::DARK_BLUE)
                .add_modifier(Modifier::BOLD),
        ))
        .render(area[0], buf);
        let titles = vec![" Processes ", " Ports ", " About "];
        Tabs::new(titles)
            .style(
                Style::new()
                    .fg(constants::MID_GRAY)
                    .bg(constants::DARK_BLUE),
            )
            .highlight_style(
                Style::new()
                    .fg(constants::WHITE)
                    .bg(constants::DARK_BLUE)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .select(self.context.tab_index)
            .divider("")
            .render(area[1], buf);
    }

    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        let row_index = self.context.row_index;
        match self.context.tab_index {
            0 => ProcessesTab::new(row_index).render(area, buf),
            1 => PortsTab::new(row_index).render(area, buf),
            2 => AboutTab::new(row_index).render(area, buf),
            // 1 => RecipeTab::new(row_index).render(area, buf),
            // 2 => EmailTab::new(row_index).render(area, buf),
            // 3 => TracerouteTab::new(row_index).render(area, buf),
            // 4 => WeatherTab::new(row_index).render(area, buf),
            _ => unreachable!(),
        };
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let keys = [
            ("Q/Esc", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
            ("ctrl+r", "Refresh"),
        ];
        let spans: Vec<Span> = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(
                    format!(" {} ", key),
                    Style::new().fg(constants::BLACK).bg(constants::DARK_GRAY),
                );
                let desc = Span::styled(
                    format!(" {} ", desc),
                    Style::new().fg(constants::DARK_GRAY).bg(constants::BLACK),
                );
                [key, desc]
            })
            .collect();
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}

pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}
