use crate::components::DrawableComponent;
use tui::layout::{Rect, Alignment};
use tui::Frame;
use anyhow::Error;
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Borders};
use crate::components::main_app::MainApp;

pub struct Volumes {
    selected: usize
}
impl DrawableComponent for Volumes {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &MainApp) -> Result<(), Error> {
        f.render_widget(
            Paragraph::new("Volume list")
                .block(Block::default().borders(Borders::ALL).title("Volumes"))
                .alignment(Alignment::Left),
            rect);
        Ok(())
    }
}

impl Volumes {
    pub fn new() -> Volumes {
        Volumes {
            selected: 0
        }
    }
}