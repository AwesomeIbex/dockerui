use crate::components::DrawableComponent;
use tui::layout::Rect;
use tui::Frame;
use anyhow::Error;
use tui::backend::Backend;

pub struct Containers {
}

impl DrawableComponent for Containers {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        Ok(())
    }
}