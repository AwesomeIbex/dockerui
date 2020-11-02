use crate::components::DrawableComponent;
use tui::layout::{Rect, Alignment};
use tui::Frame;
use anyhow::Error;
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Borders};

pub struct Containers {
    selected: usize
}
impl DrawableComponent for Containers {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        f.render_widget(
            Paragraph::new("Container list")
                .block(Block::default().borders(Borders::ALL).title("Containers"))
                .alignment(Alignment::Left),
            rect);
        Ok(())
    }
}

impl Containers {
    pub fn new() -> Containers {
        Containers {
            selected: 0
        }
    }
}