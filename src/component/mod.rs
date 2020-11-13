use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use anyhow::Error;
use crate::app::App;

pub mod util;
pub mod tab;
pub mod images;
pub mod volumes;
pub mod containers;

pub trait DrawableComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App
    ) -> Result<(), Error>;
}

pub trait MutableDrawableComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App
    ) -> Result<(), Error>;
}