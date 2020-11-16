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

pub fn get_tab_variants() -> Vec<TabVariant> {
    vec![TabVariant::Docker, TabVariant::Stats, TabVariant::Version]
}

impl TabVariant {
    pub fn get_title(&self) -> &'static str {
        match self {
            TabVariant::Docker => "Containers",
            TabVariant::Stats => "Stats",
            TabVariant::Version => "Version",
        }
    }
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect, app: App) -> Result<(), Error> {
        // match self {
        //     TabVariant::Docker => {
        //         let mut tab = DockerTab::new_with_data(
        //             app.containers_widget,
        //             app.images_widget,
        //             app.volumes_widget
        //         );
        //         tab.draw(f, rect)
        //     },
        //     TabVariant::Stats => Ok(()),
        //     TabVariant::Version => Ok(())
        // };
        Ok(())
    }
}