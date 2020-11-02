use crate::components::DrawableComponent;
use tui::backend::Backend;
use tui::layout::Rect;
use anyhow::Error;
use tui::Frame;

mod containers;

pub enum Tab {
    Containers,
    Stats,
    Bulk,
    Version
}

pub fn get_tabs() -> Vec<Tab> {
    vec![Tab::Containers, Tab::Stats, Tab::Bulk, Tab::Version]
}

impl Tab {
    pub fn get_title(&self) -> &'static str {
        match self {
            Tab::Containers => "Containers",
            Tab::Stats => "Stats",
            Tab::Bulk => "Bulk",
            Tab::Version => "Version",
        }
    }
}

impl DrawableComponent for Tab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        match self {
            Tab::Containers => containers::Containers{}.draw(f, rect).unwrap(),
            Tab::Stats => {}
            Tab::Bulk => {}
            Tab::Version => {}
        };
        Ok(())
    }
}