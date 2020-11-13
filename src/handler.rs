use std::sync::mpsc;
use std::sync::mpsc::RecvError;

use anyhow::Error;
use termion::event::Key;

use crate::component::util::event::Event;
use crate::tab::docker::DockerTab;

pub trait ComponentEventHandler {
    fn handle(&self, event: Result<Event<Key>, mpsc::RecvError>) -> Result<(), Error>;
}

impl ComponentEventHandler for DockerTab {
    fn handle(&self, event: Result<Event<Key>, RecvError>) -> Result<(), Error> {
        let event = event?;
        match event {
            Event::Input(input) => match input {
                Key::Char(c) => {
                    // TODO self.on_key(c);
                }
                Key::Down => {
                }
                Key::Up => {}
                _ => {}
            },
            _ => {}
        };
        Ok(())
    }
}