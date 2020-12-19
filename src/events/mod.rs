//! Event handling.
//!
//! ```
//! # use tinybit::*;
//! # use tinybit::events::{Event, EventModel};
//! for event in events::events(EventModel::Fps(20)) {
//!     match event {
//!         Event::Tick => {
//! #          break
//!         }
//!         _ => {}
//!     }
//! }
//! ```
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use crossterm::event::{read, Event as CrossTermEvent};

pub use crossterm::event::{KeyCode, KeyEvent};

type Rx = Receiver<Event>;

/// Event. Either a tick event or a key press event
/// TODO: add resize event
#[derive(Debug, Clone, Copy)] 
pub enum Event {
    /// Generated for every frame
    Tick,

    /// A key press
    Key(KeyEvent),


    /// Terminal resize event
    Resize(u16, u16),
}

/// Events producer
pub struct Events {
    rx: Rx,
}

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}

/// The type of events to listen for.
pub enum EventModel {
    /// Generate a tick every N milliseconds
    Fps(u64),
    /// Block until an event is raised
    Blocking
}

/// Produce events.
///
/// ```
/// # use tinybit::*;
/// # use tinybit::events::{Event, EventModel};
/// let model = EventModel::Fps(20);
/// for event in events::events(model) {
///     match event {
///         Event::Tick => {
/// #          break
///         }
///         _ => {}
///     }
/// }
/// ```
pub fn events(event_model: EventModel) -> Events {
    let (tx, rx) = mpsc::channel();

    // Input events
    let tx_clone = tx.clone();
    thread::spawn(move || loop {
        if let Ok(ev) = read() {
            match ev {
                CrossTermEvent::Key(k) => {
                    let _ = tx_clone.send(Event::Key(k));
                }
                CrossTermEvent::Resize(w, h) => {
                    let _ = tx_clone.send(Event::Resize(w, h));
                }
                _ => {}
            }
        }
    });

    if let EventModel::Fps(fps) = event_model {
        // Frames
        thread::spawn(move || loop {
            let _ = tx.send(Event::Tick);
            thread::sleep(Duration::from_millis(1000 / fps));
        });
    }

    Events { rx }
}
