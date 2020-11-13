#![forbid(unsafe_code)]

use std::{fs, io, panic, process};
use std::io::Write;
use std::panic::PanicInfo;
use std::path::PathBuf;
use std::sync::{Arc};
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Error;
use backtrace::Backtrace;
use termion::{input::MouseTerminal, raw::IntoRawMode};
use tui::{backend::TermionBackend, Terminal};

use app::App;
use crate::component::util::Config;
use crate::component::util::event::{Event, Events};
use tokio::sync::Mutex;

mod page;
mod components;
mod handler;
mod app;
pub mod docker;
mod style;
mod component;
pub mod tab;

fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");
        println!("{}", stacktrace);
        //TODO shut down the terminal
    //     execute!(
    //   io::stdout(),
    //   LeaveAlternateScreen,
    //   Print(format!(
    //     "thread '<unnamed>' panicked at '{}', {}\n\r{}",
    //     msg, location, stacktrace
    //   )),
    //   DisableMouseCapture
    // )
    //         .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    pretty_env_logger::init();

    let (tx, rx) = std::sync::mpsc::channel();

    let app = Arc::new(Mutex::new(App::new(tx)));
    let cloned_app = Arc::clone(&app);

    std::thread::spawn(move || {
        docker::start_tokio(&app, rx);
    });

    start_ui(&cloned_app).await?;
    Ok(())
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<(), Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    // let stdout = AlternateScreen::from(stdout); //TODO to enable the tui but with logs
    let backend = TermionBackend::new(stdout);

    // set_panic_handlers()?;

    let config = Config {
        tick_rate: Duration::from_millis(250),
        ..Default::default()
    };
    let events = Events::with_config(config);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    loop {
        let mut app = app.lock().await;
        if app.should_quit {
            break;
        }
        terminal.draw(|f| {
            &app.draw(f);
        })?;

        app.handle_event(events.next())?;
    }

    Ok(())
}

fn set_panic_handlers() -> Result<(), Error> {
    // regular panic handler
    panic::set_hook(Box::new(|e| {
        let backtrace = Backtrace::new();
        println!("{:?}", backtrace);

        log::error!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
        // shutdown_terminal().expect("shutdown failed inside panic");
        eprintln!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
    }));

    // global threadpool
    rayon_core::ThreadPoolBuilder::new()
        .panic_handler(|e| {
            let backtrace = Backtrace::new();
            println!("{:?}", backtrace);
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
