use std::sync::mpsc;

use anyhow::Error;
use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;

use crate::app::App;

pub trait DrawableComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect
    ) -> Result<(), Error>;
}

pub trait MutableDrawableComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect
    ) -> Result<(), Error>;
}
