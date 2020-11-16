use std::sync::{Arc, mpsc};
use std::sync::mpsc::Sender;

use anyhow::Error;
use bollard::service::{ContainerSummaryInner, ImageSummary, Volume};
use termion::{event::Key};
use tui::{Frame, layout::{Constraint, Layout}, style::{Color, Style}, Terminal, text::{Span, Spans}, widgets::{Block, Borders, Row, Table, TableState}};
use tui::backend::Backend;
use tui::layout::{Direction, Margin, Rect, Alignment};
use tui::widgets::{Tabs, Paragraph, ListItem, List, ListState};

use crate::component::containers::Containers;
use crate::component::images::Images;
use crate::component::util::event::Event;
use crate::component::util::{TabsState, StatefulList};
use crate::component::volumes::Volumes;
use crate::components::{DrawableComponent, MutableDrawableComponent};
use crate::network;
use crate::network::IOEvent;
use crate::style::{SharedTheme, Theme};
use crate::tab::{get_tab_variants, TabVariant};
use crate::tab::docker::DockerTab;
use tui::style::Modifier;

pub struct App {
    pub(crate) should_quit: bool,
    theme: SharedTheme,
    tab_state: TabsState,
    selected_tab: usize,
    selected_pane: Pane,
    pub container_data: Vec<ContainerSummaryInner>,
    pub image_data: Vec<ImageSummary>,
    pub volume_data: Vec<Volume>,
    pub container_state: StatefulList<ContainerSummaryInner>,
    pub image_state: StatefulList<ImageSummary>,
    tx: Sender<network::IOEvent>,
}

enum Pane {
    Containers,
    Images,
    Volumes,
    Logs,
}

/// Drawing:
/// Draw main app
/// Take the current tab and then draw
/// Tab draws its components
///
/// Events
/// Handle main event
/// Take the current tab and then handle event
/// Tab matches against its panes to determine what event to handle for each componenty
/// component may update tabs pane
///
/// Lifetimes, i want the

impl App {
    pub fn new(tx: Sender<network::IOEvent>) -> App {
        App {
            selected_pane: Pane::Containers,
            should_quit: false,
            tab_state: TabsState::new(get_tab_variants()), //Build tab from dynamic list TODO
            theme: Arc::new(Theme::init()),
            selected_tab: 0,
            tx,
            container_data: vec![],
            image_data: vec![],
            volume_data: vec![],
            container_state: StatefulList::new(),
            image_state: StatefulList::new()
        }
    }

    fn update(&mut self) {
        if let Err(err) = self.tx.send(IOEvent::RefreshImages) {
            log::error!("Failed to send the message to refresh images, {}", err)
        }
        if let Err(err) = self.tx.send(IOEvent::RefreshContainers) {
            log::error!("Failed to send the message to refresh containers, {}", err)
        }
    }

    fn deselect_others(&mut self) {
        match self.selected_pane {
            Pane::Containers => {
                self.image_state.unselect();
                self.container_state.state.select(Some(0));
            }
            Pane::Images => {
                self.container_state.unselect();
                self.image_state.state.select(Some(0));
            }
            Pane::Volumes => {}
            Pane::Logs => {}
        }
    }
    pub fn handle_event(&mut self, event: Result<Event<Key>, mpsc::RecvError>) -> Result<(), Error> {
        let event = event?;
        match event {
            Event::Input(input) => match input {
                Key::Char(c) => {
                    //Basically handle exit keys but otherwise take the tab and call its event handler TODO
                    match c {
                        'q' | 'x' => { //TODO could do a multi-modifier but yolo
                            self.should_quit = true;
                        }
                        'i' => {
                            self.selected_pane = Pane::Images;
                            self.deselect_others()
                        },
                        'c' => {
                            self.selected_pane = Pane::Containers;
                            self.deselect_others()
                        },
                        _ => {
                            println!("s")
                            // get tab
                            // call handler with key
                        }
                    };
                }
                Key::Down => {
                    match self.selected_pane {
                        Pane::Containers => self.container_state.next(),
                        Pane::Images => self.image_state.next(),
                        Pane::Volumes => {}
                        Pane::Logs => {}
                    }

                }
                Key::Up => {
                    match self.selected_pane {
                        Pane::Containers => self.container_state.previous(),
                        Pane::Images => self.image_state.previous(),
                        Pane::Volumes => {}
                        Pane::Logs => {}
                    }
                }
                Key::PageDown => {
                    self.tab_state.next();
                    self.selected_tab = self.tab_state.index;
                }
                Key::PageUp => {
                    self.tab_state.previous();
                    self.selected_tab = self.tab_state.index;
                }
                Key::Backspace | Key::Esc => {
                    self.should_quit = true;
                }
                _ => println!("couldnt match")
            },
            Event::Tick => {
                self.update();
            }
        }
        Ok(())
    }

    pub fn get_default_chunks(&self, size: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
            .split(size)
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let chunks = self.get_default_chunks(size);
        let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::LightMagenta));
        f.render_widget(block, size);

        self.draw_tab_bar(f, chunks[0]);
        let tab_rect = chunks[1];

        let right_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                    .as_ref(),
            )
            .split(tab_rect);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                ]
                    .as_ref(),
            )
            .split(right_chunks[0]);

        self.draw_containers(f, left_chunks[0]);
        self.draw_images(f, left_chunks[1]);
    }

    fn draw_containers<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) {
        let items: Vec<ListItem> = self.container_state
            .items
            .iter()
            .map(|i| {
                let names = i.names.as_ref().unwrap();
                let mut lines = vec![];

                names.iter().for_each(|name| {
                    lines.push(Spans::from(Span::styled(
                        name.replace("/", ""),
                        Style::default().add_modifier(Modifier::ITALIC),
                    )));
                });
                ListItem::new(lines).style(Style::default().fg(Color::White))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Containers"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(list, rect, &mut self.container_state.state);
    }
    fn draw_images<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) {
        let mut names: Vec<String> = self.image_state.items.clone()
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
        names.dedup();

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
                ListItem::new(lines).style(Style::default().fg(Color::White))
            })
            .collect();

        //TODO items is 38 here :/
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Images"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(items, rect, &mut self.image_state.state);
    }

    fn draw_tab_bar<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tabs = self
            .tab_state
            .tabs_variants
            .iter()
            .map(|e| e.get_title())
            .map(|t| App::build_bar_spans(t))
            .collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .border_style(self.theme.block(false)),
                )
                .style(self.theme.tab(false))
                .highlight_style(self.theme.tab(true))
                // .divider(strings::tab_divider(&app.key_config))
                .select(self.selected_tab),
            r,
        );
    }

    fn build_bar_spans(t: &str) -> Spans {
        let (first, rest) = t.split_at(1);
        Spans::from(vec![
            Span::styled(first, Style::default().fg(Color::Red)),
            Span::styled(rest, Style::default().fg(Color::DarkGray)),
        ])
    }
}
