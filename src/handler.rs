use std::sync::mpsc;

use termion::event::Key;

use crate::component::util::event::Event;
use anyhow::Error;

pub trait ComponentEventHandler {
    fn handle(&self, event: Result<Event<Key>, mpsc::RecvError>) -> Result<(), Error>;
}