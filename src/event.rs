use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn next(&self) -> Result<Event, std::io::Error> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                CEvent::Key(key) => Ok(Event::Key(key)),
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}
