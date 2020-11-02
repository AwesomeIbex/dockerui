use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use anyhow::Error;

pub mod main_app;
pub mod util;
pub mod containers;

pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<(), Error>;
}

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
            Tab::Containers => {}
            Tab::Stats => {}
            Tab::Bulk => {}
            Tab::Version => {}
        };
        Ok(())
    }
}