use crate::components::DrawableComponent;
use tui::backend::Backend;
use tui::layout::Rect;
use anyhow::Error;
use tui::Frame;
use crate::components::main_app::MainApp;
use containers_tab::ContainersTab;

mod containers_tab;

pub enum Tab {
    Containers,
    Stats,
    Version
}

pub fn get_tabs() -> Vec<Tab> {
    vec![Tab::Containers, Tab::Stats, Tab::Version]
}

impl Tab {
    pub fn get_title(&self) -> &'static str {
        match self {
            Tab::Containers => "Containers",
            Tab::Stats => "Stats",
            Tab::Version => "Version",
        }
    }
}

impl DrawableComponent for Tab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &MainApp) -> Result<(), Error> {
        match self {
            Tab::Containers => ContainersTab().draw(f, rect, app).unwrap(),
            Tab::Stats => {}
            Tab::Version => {}
        };
        Ok(())
    }
}