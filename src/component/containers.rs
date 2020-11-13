use tui::layout::{Rect, Alignment};
use tui::Frame;
use anyhow::Error;
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Borders, ListItem, List};
use crate::component::util::StatefulList;
use bollard::service::ContainerSummaryInner;
use tui::style::{Style, Modifier, Color};
use tui::text::{Span, Spans};
use crate::app::App;
use crate::components::MutableDrawableComponent;

pub struct Containers {
    selected: usize,
    items: StatefulList<ContainerSummaryInner>
}
impl MutableDrawableComponent for Containers {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        let items: Vec<ListItem> = self.items
            .items
            .iter()
            .map(|i| {
                let names = i.names.as_ref().unwrap();
                let mut lines = vec![];

                names.iter().for_each(|name| {
                    lines.push(Spans::from(Span::styled(
                        name,
                        Style::default().add_modifier(Modifier::ITALIC),
                    )));
                });
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Containers"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list, rect, &mut self.items.state); //TODO i think this is wrong

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
    pub fn new_with_items(data: Vec<ContainerSummaryInner>) -> Containers {
        Containers {
            selected: 0,
            items: StatefulList::with_items(data)
        }
    }
}