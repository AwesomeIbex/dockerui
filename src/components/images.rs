use anyhow::Error;
use bollard::models::ImageSummary;
use bollard::service::ContainerSummaryInner;
use itertools::Itertools;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::components::{DrawableComponent, MutableDrawableComponent};
use crate::components::main_app::MainApp;
use crate::components::util::StatefulList;

pub struct Images {
    selected: usize,
    items: StatefulList<ImageSummary>,
}

impl MutableDrawableComponent for Images {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect, app: &MainApp) -> Result<(), Error> {
        let items: Vec<ListItem> = app.images.clone()
            .iter()
            .map(|i| {
                let summary = i.clone();
                let images = summary.labels.get_key_value("name");

                match images {
                    None => vec![],
                    Some(value) => {
                        let name = value.1.clone();
                        let mut lines = vec![];

                        lines.push(Spans::from(Span::styled(
                            name,
                            Style::default().add_modifier(Modifier::ITALIC),
                        )));
                        lines
                    }
                }
            }).dedup()
            .map(|lines| ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White)))
            .collect();

        //TODO items is 38 here :/
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Images"))
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

impl Images {
    pub fn new() -> Images {
        Images {
            selected: 0,
            items: StatefulList::new(),
        }
    }
}