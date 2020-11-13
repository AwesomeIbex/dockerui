use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Paragraph};

use crate::component::containers::Containers;
use crate::component::images::Images;
use crate::component::volumes::Volumes;
use crate::components::{DrawableComponent, MutableDrawableComponent};
use crate::app::App;
use anyhow::Error;
use crate::handler::ComponentEventHandler;
use crate::component::util::event::Event;
use std::sync::mpsc::RecvError;
use termion::event::Key;

struct DockerTab {
    containers: Option<Containers>, //TODO make these self contained too
    images: Option<Images>,
    volumes: Option<Volumes>,
}

impl DockerTab {
    pub fn new(&self) -> DockerTab {
        DockerTab {
            containers: Some(Containers::new()),
            images: Some(Images::new()),
            volumes: Some(Volumes::new()),
        }
    }
    pub fn get_title(&self) -> String {
        String::from("Docker")
    }
}

impl DrawableComponent for DockerTab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<(), Error> {
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

        if let Some(mut containers) = &self.containers {
            containers.draw(f, left_chunks[0], app)?;
        }

        if let Some(mut images) = &self.images {
            images.draw(f, left_chunks[1], app)?;
        }

        if let Some(mut volumes) = &self.volumes {
            volumes.draw(f, left_chunks[2], app)?;
        }
        Ok(())
    }
}

impl ComponentEventHandler for DockerTab {
    fn handle(&self, event: Result<Event<Key>, RecvError>) -> Result<(), Error> {
        let event = event?;
        match event {
            Event::Input(input) => match input {
                Key::Char(c) => {
                    // TODO self.on_key(c);
                }
                Key::Down => {
                }
                Key::Up => {}
                _ => {}
            },
            _ => {}
        };
        Ok(())
    }
}