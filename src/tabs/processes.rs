use std::time::Duration;

use constants::*;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Margin},
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, Clear, List, ListItem, ListState, Paragraph, Row, Scrollbar, ScrollbarState,
        StatefulWidget, Table, TableState, Tabs, Widget, Wrap,
    },
};
use sysinfo::Process;

use crate::{app::AppContext, constants, root::layout};
pub struct ProcessesTab<'a> {
    app_context: &'a AppContext,
    processes: &'a Vec<&'a Process>,
}

impl<'a> ProcessesTab<'a> {
    pub fn new(app_context: &'a AppContext, processes: &'a Vec<&'a Process>) -> Self {
        Self {
            app_context,
            processes,
        }
    }
}

impl Widget for ProcessesTab<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        });
        // Clear.render(area, buf);
        let area = layout(area, Direction::Vertical, vec![20, 0]);

        let process_list_area = area[0];
        let process_display_area = area[1];

        let highlight_symbol = "|>";
        let header = Row::new::<Vec<String>>(vec![
            "PID".into(),
            "Name".into(),
            "Running for".into(),
            "UserID".into(),
            "Disk Read".into(),
            "Disk Written".into(),
            "cwd".into(),
        ])
        .style(Style::new().bold())
        .bottom_margin(1);

        let items: Vec<_> = self
            .processes
            .iter()
            .map(|p| {
                let name = format!("{}", p.name());
                Row::new::<Vec<String>>(vec![
                    format!("{}", p.pid()).into(),
                    name.into(),
                    format!("{}", format_duration(Duration::from_secs(p.run_time()))).into(),
                    format!(
                        "{}",
                        match p.user_id() {
                            Some(u) => u.to_string(),
                            None => "System".to_string(),
                        }
                    )
                    .into(),
                    format!("{}", format_bytes(p.disk_usage().total_read_bytes)).into(),
                    format!("{}", format_bytes(p.disk_usage().total_written_bytes)).into(),
                    format!(
                        "{}",
                        match p.cwd() {
                            Some(p) => p.to_string_lossy().to_string(),
                            None => "Unknown".to_string(),
                        }
                    )
                    .into(),
                ])
            })
            .collect();

        let mut state = TableState::default().with_selected(Some(self.app_context.row_index));
        StatefulWidget::render(
            Table::new(
                items,
                [
                    Constraint::Length(7),
                    Constraint::Length(30),
                    Constraint::Length(15),
                    Constraint::Length(10),
                    Constraint::Length(13),
                    Constraint::Length(13),
                    Constraint::Length(30),
                ],
            )
            .header(header)
            .style(Style::new().bg(DARK_BLUE).fg(LIGHT_GRAY))
            .highlight_style(Style::new().fg(LIGHT_YELLOW))
            .highlight_symbol(highlight_symbol),
            process_list_area,
            buf,
            &mut state,
        );
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(self.processes.len())
            .position(self.app_context.row_index);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("▐")
            .render(process_list_area, buf, &mut scrollbar_state);
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let seconds = duration.as_secs();

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    let mut result = String::new();

    if hours > 0 {
        result.push_str(&format!("{}h", hours));
    }

    if minutes > 0 {
        result.push_str(&format!("{}m", minutes));
    }

    if seconds > 0 || result.is_empty() {
        result.push_str(&format!("{}s", seconds));
    }

    result
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes < KB {
        return format!("{}B", bytes);
    } else if bytes < MB {
        return format!("{:.2}KB", bytes as f64 / KB as f64);
    } else if bytes < GB {
        return format!("{:.2}MB", bytes as f64 / MB as f64);
    } else if bytes < TB {
        return format!("{:.2}GB", bytes as f64 / GB as f64);
    } else {
        return format!("{:.2}TB", bytes as f64 / TB as f64);
    }
}
