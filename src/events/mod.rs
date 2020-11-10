//! Event handling.
//!
//! ```
//! # use florp_engine::*;
//! # use florp_engine::events::Event;
//! let fps = 20;
//! for event in events::events(fps) {
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
pub enum Event {
    /// Generated for every frame
    Tick,

    /// A key press
    Key(KeyEvent),
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

/// Produce events.
///
/// ```
/// # use florp_engine::*;
/// # use florp_engine::events::Event;
/// let fps = 20;
/// for event in events::events(fps) {
///     match event {
///         Event::Tick => {
/// #          break
///         }
///         _ => {}
///     }
/// }
/// ```
pub fn events(fps: u64) -> Events {
    let (tx, rx) = mpsc::channel();

    // Input events
    let tx_clone = tx.clone();
    thread::spawn(move || loop {
        if let Ok(ev) = read() {
            if let CrossTermEvent::Key(k) = ev {
                let _ = tx_clone.send(Event::Key(k));
            }
        }
    });

    // Frames
    thread::spawn(move || loop {
        let _ = tx.send(Event::Tick);
        thread::sleep(Duration::from_millis(1000 / fps));
    });

    Events { rx }
}
