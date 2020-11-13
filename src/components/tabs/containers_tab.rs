use anyhow::Error;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Paragraph};

use crate::components::{DrawableComponent, MutableDrawableComponent};
use crate::components::images::Images;
use crate::components::containers::Containers;
use crate::components::volumes::Volumes;
use crate::components::main_app::MainApp;

pub struct ContainersTab {}

impl DrawableComponent for ContainersTab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &MainApp) -> Result<(), Error> {
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
            Paragraph::new("logs value with some stuff")
                .block(Block::default().borders(Borders::ALL).title("Logs"))
                .alignment(Alignment::Left),
            right_chunks[1]);

        let mut containers = Containers::new();
        containers.draw(f, left_chunks[0], app)?;

        let mut images = Images::new();
        images.draw(f, left_chunks[1], app)?;

        let mut volumes = Volumes::new();
        volumes.draw(f, left_chunks[2], app)?;

        Ok(())
    }
}