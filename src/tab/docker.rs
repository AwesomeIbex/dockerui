use std::sync::mpsc::RecvError;

use anyhow::Error;
use bollard::models::{ContainerSummaryInner, Volume, ImageSummary};
use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::component::containers::Containers;
use crate::component::images::Images;
use crate::component::util::event::Event;
use crate::component::volumes::Volumes;
use crate::components::{DrawableComponent, MutableDrawableComponent};
use crate::handler::ComponentEventHandler;
use crate::tab::TabVariant;

pub struct DockerTab {
    pub containers: Option<Containers>, //TODO make these self contained too
    pub images: Option<Images>,
    pub volumes: Option<Volumes>,
}

impl DockerTab {
    pub fn new(&self) -> DockerTab {
        DockerTab {
            containers: Some(Containers::new()), //todo DONT COPY
            images: Some(Images::new()),
            volumes: Some(Volumes::new()),
        }
    }
    pub fn new_with_data(container_data: Vec<ContainerSummaryInner>, image_data: Vec<ImageSummary>, volume_data: Vec<Volume>) -> DockerTab {
        DockerTab {
            containers: Some(Containers::new_with_items(container_data)),
            images: Some(Images::new_with_items(image_data)),
            volumes: Some(Volumes::new_with_items(volume_data)),
        }
    }
    pub fn get_title(&self) -> String {
        String::from("Docker")
    }
    pub fn get_variant(&self) -> TabVariant {
        TabVariant::Docker
    }
}

impl MutableDrawableComponent for DockerTab {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
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

        //TODO these are nasty hacks
        if self.containers.is_some() {
            let containers = &mut self.containers;
            let s: &mut Containers = containers.as_mut().unwrap();
            s.draw(f, left_chunks[0]);
            // &containers.draw(f, left_chunks[0])?;
        }

        if self.images.is_some() {
            let images = &mut self.images;
            let s: &mut Images = images.as_mut().unwrap();
            s.draw(f, left_chunks[1]);
        }

        if self.volumes.is_some() {
            let volumes = &mut self.volumes;
            let s: &mut Volumes = volumes.as_mut().unwrap();
            s.draw(f, left_chunks[2]);
        }
        Ok(())
    }
}

