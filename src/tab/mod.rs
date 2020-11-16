use crate::components::{DrawableComponent, MutableDrawableComponent};
use tui::backend::Backend;
use tui::layout::Rect;
use anyhow::Error;
use tui::Frame;
use crate::app::App;
use crate::tab::docker::DockerTab;

pub mod docker;

pub enum TabVariant {
    Docker,
    Stats,
    Version
}

impl TabVariant {
    pub fn get_title(&self) -> &'static str {
        match self {
            TabVariant::Docker => "Containers",
            TabVariant::Stats => "Stats",
            TabVariant::Version => "Version",
        }
    }
    pub(crate) fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<(), Error> {
        match self {
            TabVariant::Docker => app.docker_tab.unwrap().draw(f, rect),
            TabVariant::Stats => Ok(()),
            TabVariant::Version => Ok(())
        };
        Ok(())
    }
}