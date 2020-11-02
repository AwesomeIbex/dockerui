use crate::components::DrawableComponent;
use tui::layout::{Rect, Alignment};
use tui::Frame;
use anyhow::Error;
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Borders};

pub struct Images {
    selected: usize
}
impl DrawableComponent for Images {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        f.render_widget(
            Paragraph::new("Image list")
                .block(Block::default().borders(Borders::ALL).title("Images"))
                .alignment(Alignment::Left),
            rect);
        Ok(())
    }
}

impl Images {
    pub fn new() -> Images {
        Images {
            selected: 0
        }
    }
}