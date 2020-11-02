#![forbid(unsafe_code)]

use std::{fs, io, panic, process};
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

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

use util::event::Config;

use crate::util::event::{Event, Events};
use crate::util::TabsState;
use crate::style::{Theme, SharedTheme};

mod util;
mod style;

struct App<'a> {
    should_quit: bool,
    tabs: TabsState<'a>,
    theme: SharedTheme,
    selected_tab: usize
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let theme = Rc::new(Theme::init());

        App {
            should_quit: false,
            tabs: TabsState::new(vec!["Main", "Stats", "Bulk", "Version"]), //Build tabs from dynamic list TODO
            theme,
            selected_tab: 0
        }
    }

    fn update(&mut self) {}
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    defer! {
        // shutdown_terminal().expect("shutdown failed");
    }

    set_panic_handlers()?;

    // Setup event handlers
    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    // let mut spinner = Spinner::default();
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
                .split(size);
            let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::LightMagenta));
            f.render_widget(block, size);

            let titles = app
                .tabs
                .titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(first, Style::default().fg(Color::Yellow)),
                        Span::styled(rest, Style::default().fg(Color::Green)),
                    ])
                })
                .collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);

            // Get each tab title from the tab itself
            let inner = match app.tabs.index {
                0 => Block::default().title("Inner 0").borders(Borders::ALL),
                1 => Block::default().title("Inner 1").borders(Borders::ALL),
                2 => Block::default().title("Inner 2").borders(Borders::ALL),
                3 => Block::default().title("Inner 3").borders(Borders::ALL),
                _ => unreachable!(),
            };
            f.render_widget(inner, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Down => {}
                Key::Up => {}
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Esc => {
                    app.should_quit = true;
                }
                _ => {}
            },
            Event::Tick => {
                app.update();
            }
        }
        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn set_panic_handlers() -> Result<(), Error> {
    // regular panic handler
    panic::set_hook(Box::new(|e| {
        let backtrace = Backtrace::new();
        log::error!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
        // shutdown_terminal().expect("shutdown failed inside panic");
        eprintln!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
    }));

    // global threadpool
    rayon_core::ThreadPoolBuilder::new()
        .panic_handler(|e| {
            let backtrace = Backtrace::new();
            log::error!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
            // shutdown_terminal()
            //     .expect("shutdown failed inside panic");
            eprintln!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
            process::abort();
        })
        .num_threads(4)
        .build_global()?;

    Ok(())
}

fn draw_tabs<B: Backend>(app: &App, f: &mut Frame<B>, r: Rect) {
    let r = r.inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });

    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    //TODO: this could all be fetched from something and define if its optional
    // let tabs = [
    //     Span::raw(strings::tab_status(&self.key_config)),
    //     Span::raw(strings::tab_log(&self.key_config)),
    //     Span::raw(strings::tab_stashing(&self.key_config)),
    //     Span::raw(strings::tab_docker(&self.key_config)),
    //     // Span::raw(strings::tab_stashes(&self.key_config)), TODO add tab here
    // ]
    //     .iter()
    //     .cloned()
    //     .map(Spans::from)
    //     .collect();

    f.render_widget(
        Tabs::new(tabs)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(app.theme.block(false)),
            )
            .style(app.theme.tab(false))
            .highlight_style(app.theme.tab(true))
            // .divider(strings::tab_divider(&app.key_config))
            .select(app.selected_tab),
        r,
    );
}

fn get_app_config_path() -> Result<PathBuf, Error> {
    let mut path = dirs_next::config_dir()
        .ok_or_else(|| anyhow!("failed to find os config dir."))?;

    path.push("dockerui");
    fs::create_dir_all(&path)?;
    Ok(path)
}