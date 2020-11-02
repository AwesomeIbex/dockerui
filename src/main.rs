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

use crate::style::{Theme, SharedTheme};
use crate::components::util::Config;
use crate::components::util::event::{Events, Event};
use crate::components::main_app::MainApp;

pub mod docker;
mod style;
mod components;

fn main() -> Result<(), Error> {
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

    let mut app = MainApp::new();

    loop {
        terminal.draw(|f| {
            &app.draw(f);
        })?;

        let should_break = app.handle_event(events.next())?;
        if should_break {
            break;
        };
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

fn get_app_config_path() -> Result<PathBuf, Error> {
    let mut path = dirs_next::config_dir()
        .ok_or_else(|| anyhow!("failed to find os config dir."))?;

    path.push("dockerui");
    fs::create_dir_all(&path)?;
    Ok(path)
}