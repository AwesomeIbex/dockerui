use anyhow::Error;
use bollard::models::{ImageSummary, Volume};
use bollard::service::ContainerSummaryInner;
use itertools::Itertools;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};

use crate::app::App;
use crate::component::util::StatefulList;
use crate::components::MutableDrawableComponent;

pub struct Volumes {
    selected: usize,
    items: StatefulList<Volume>,
}
impl MutableDrawableComponent for Volumes {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<(), Error> {
        let mut names = self.filter_names();
        names.dedup();

        let items = Volumes::map_to_list_items(&names);

        //TODO items is 38 here :/
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Volumes"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(items, rect, &mut self.items.state);
        Ok(())
    }
}

impl Volumes {
    pub fn new() -> Volumes {
        Volumes {
            selected: 0,
            items: StatefulList::new()
        }
    }
    pub fn new_with_items(data: Vec<Volume>) -> Volumes {
        Volumes {
            selected: 0,
            items: StatefulList::with_items(data)
        }
    }

    fn filter_names(&self) -> Vec<String> {
        let names: Vec<String> = self.items.items.clone()
            .iter()
            .map(|i| {
                let summary = i.clone();
                let images = summary.labels.get_key_value("name");

                match images {
                    None => String::new(),
                    Some(value) => {
                        value.1.clone()
                    }
                }
            })
            .filter(|name| !name.is_empty())
            .collect();
        names
    }

    fn map_to_list_items(names: &Vec<String>) -> Vec<ListItem> {
        let items: Vec<ListItem> = names
            .iter()
            .map(|name| {
                vec![(Spans::from(Span::styled(
                    name,
                    Style::default().add_modifier(Modifier::ITALIC),
                )))]
            })
            .map(|mut lines| {
                lines.dedup();
                ListItem::new(lines).style(Style::default().fg(Color::Red))
            })
            .collect();
        items
    }
}