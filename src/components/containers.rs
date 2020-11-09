use crate::components::{DrawableComponent, MutableDrawableComponent};
use tui::layout::{Rect, Alignment};
use tui::Frame;
use anyhow::Error;
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Borders, ListItem, List};
use crate::components::util::StatefulList;
use bollard::service::ContainerSummaryInner;
use tui::style::{Style, Modifier, Color};
use tui::text::{Span, Spans};
use crate::components::main_app::MainApp;

pub struct Containers {
    selected: usize,
    items: StatefulList<ContainerSummaryInner>
}
impl MutableDrawableComponent for Containers {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect, app: &MainApp) -> Result<(), Error> {
        let items: Vec<ListItem> = app.containers
            .iter()
            .map(|i| {
                let id = i.names.as_ref().unwrap();
                let mut lines = vec![]; //TODO remove this
                for _ in 0..id.len() {
                    lines.push(Spans::from(Span::styled(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                        Style::default().add_modifier(Modifier::ITALIC),
                    )));
                }
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(items, rect, &mut self.items.state);

        // f.render_widget(
        //     Paragraph::new("Container list")
        //         .block(Block::default().borders(Borders::ALL).title("Containers"))
        //         .alignment(Alignment::Left),
        //     rect);
        Ok(())
    }
}

impl Containers {
    pub fn new() -> Containers {
        Containers {
            selected: 0,
            items: StatefulList::new()
        }
    }
}