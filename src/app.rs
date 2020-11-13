use std::sync::{Arc, mpsc};
use std::sync::mpsc::Sender;

use anyhow::Error;
use bollard::service::{ContainerSummaryInner, ImageSummary, Volume};
use termion::{event::Key};
use tui::{Frame, layout::{Constraint, Layout}, style::{Color, Style}, Terminal, text::{Span, Spans}, widgets::{Block, Borders, Row, Table, TableState}};
use tui::backend::Backend;
use tui::layout::{Direction, Margin, Rect};
use tui::widgets::Tabs;

use crate::components::DrawableComponent;
use crate::component::containers::Containers;
use crate::component::images::Images;
use crate::tab::get_tabs;
use crate::component::util::event::Event;
use crate::component::util::TabsState;
use crate::component::volumes::Volumes;
use crate::docker;
use crate::docker::IOEvent;
use crate::style::{SharedTheme, Theme};

pub struct App {
    pub(crate) should_quit: bool,
    theme: SharedTheme,
    tab_state: TabsState,
    selected_tab: usize,
    selected_pane: Pane,
    pub container_data: Vec<ContainerSummaryInner>,
    pub image_data: Vec<ImageSummary>,
    pub volume_data: Vec<Volume>,
    containers_widget: Option<Containers>,
    images_widget: Option<Images>,
    volumes_widget: Option<Volumes>,
    tx: Sender<docker::IOEvent>,
}

enum Pane {
    Containers,
    Images,
    Volumes,
    Logs,
}

impl App {
    pub fn new(tx: Sender<docker::IOEvent>) -> App {
        let theme = Arc::new(Theme::init());

        let tabs = get_tabs();

        App {
            selected_pane: Pane::Containers,
            should_quit: false,
            tab_state: TabsState::new(tabs), //Build tab from dynamic list TODO
            theme,
            selected_tab: 0,
            container_data: vec![],
            image_data: vec![],
            volume_data: vec![],
            containers_widget: Option::None,
            volumes_widget: Option::None,
            images_widget: Option::None,
            tx,
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
                        _ => {
                            // get tab
                            // call handler with key
                        }
                    };
                }
                Key::Down => {
                    self.tab_state.get_current_tab();
                }
                Key::Up => {}
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
                _ => {}
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

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();
        let chunks = self.get_default_chunks(size);
        let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::LightMagenta));
        f.render_widget(block, size);
        self.draw_tab_bar(f, chunks[0]);

        //TODO this will change with architecture and just take the current tab, draw it
        let tab = self.tab_state.get_current_tab();
        if let Err(error) = tab.draw(f, chunks[1], self) {
            log::error!("There was an error {:?}", error)
        }
    }

    fn draw_tab_bar<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tabs = self
            .tab_state
            .tabs
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
