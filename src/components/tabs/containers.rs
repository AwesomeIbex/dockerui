use anyhow::Error;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Paragraph};

use crate::components::DrawableComponent;

pub struct Containers {}

impl DrawableComponent for Containers {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        let right_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                    .as_ref(),
            )
            .split(rect);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
            )
            .split(right_chunks[0]);

        f.render_widget(
            Paragraph::new("Logs3232")
                .block(Block::default().borders(Borders::ALL).title("Logs22"))
                .alignment(Alignment::Left),
            right_chunks[0]);
        f.render_widget(
            Paragraph::new("logs value with some stuff")
                .block(Block::default().borders(Borders::ALL).title("Logs"))
                .alignment(Alignment::Left),
            right_chunks[1]);
        f.render_widget(
            Paragraph::new("some list of containers")
                .block(Block::default().borders(Borders::ALL).title("Containers"))
                .alignment(Alignment::Left),
            left_chunks[0]);
        f.render_widget(
            Paragraph::new("Image list")
                .block(Block::default().borders(Borders::ALL).title("Images"))
                .alignment(Alignment::Left),
            left_chunks[1]);
        f.render_widget(
            Paragraph::new("Volume list")
                .block(Block::default().borders(Borders::ALL).title("Volumes"))
                .alignment(Alignment::Left),
            left_chunks[2]);
        Ok(())
    }
}