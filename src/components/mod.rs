use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use anyhow::Error;
use crate::components::main_app::MainApp;

pub mod main_app;
pub mod util;
pub mod tabs;
pub mod images;
pub mod volumes;
pub mod containers;

pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &MainApp
    ) -> Result<(), Error>;
}

pub trait MutableDrawableComponent {
    ///
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &MainApp
    ) -> Result<(), Error>;
}