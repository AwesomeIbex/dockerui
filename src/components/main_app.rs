use std::rc::Rc;
use std::sync::mpsc;

use anyhow::anyhow;
use anyhow::Error;
use backtrace::Backtrace;
use scopeguard::defer;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tokio::time::Duration;
use tui::{backend::TermionBackend, Frame, layout::{Constraint, Layout}, style::{Color, Modifier, Style}, Terminal, text::{Span, Spans}, widgets::{Block, Borders, Row, Table, TableState}};
use tui::backend::Backend;
use tui::layout::{Direction, Margin, Rect};
use tui::widgets::canvas::{Canvas, Map, MapResolution, Rectangle};
use tui::widgets::Tabs;
use crate::components::util::event::Event;
use crate::components::util::TabsState;
use crate::style::{SharedTheme, Theme};
use crate::components::{DrawableComponent, Tab, get_tabs};
use crate::components::containers::Containers;

pub struct MainApp {
    should_quit: bool,
    tab_state: TabsState,
    theme: SharedTheme,
    selected_tab: usize,
}

impl MainApp {
    pub fn new() -> MainApp {
        let theme = Rc::new(Theme::init());

        let tabs = get_tabs();
        MainApp {
            should_quit: false,
            tab_state: TabsState::new(tabs), //Build tabs from dynamic list TODO
            theme,
            selected_tab: 0,
        }
    }

    fn update(&mut self) {}
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' | 'x' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn handle_event(&mut self, event: Result<Event<Key>, mpsc::RecvError>) -> Result<bool, Error> {
        match event? {
            Event::Input(input) => match input {
                Key::Char(c) => {
                    self.on_key(c);
                }
                Key::Down => {}
                Key::Up => {}
                Key::Right => {
                    self.tab_state.next();
                    self.selected_tab = self.tab_state.index;
                },
                Key::Left => {
                    self.tab_state.previous();
                    self.selected_tab = self.tab_state.index;
                },
                Key::Esc => {
                    self.should_quit = true;
                }
                _ => {}
            },
            Event::Tick => {
                self.update();
            }
        }
        return if self.should_quit {
            Ok(true)
        } else {
            Ok(false)
        };
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


        // TODO Get each tab title from the tab itself
        let tab = self.tab_state.get_current_tab();
        tab.draw(f, chunks[1]);
        // let inner = Block::default().title(tab.get_title()).borders(Borders::ALL);
        // f.render_widget(inner, chunks[1]);
    }

    fn draw_tab_bar<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tabs = self
            .tab_state.tabs
            .iter()
            .map(|e| e.get_title())
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Yellow)),
                    Span::styled(rest, Style::default().fg(Color::Green)),
                ])
            })
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
}