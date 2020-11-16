//! Event handling.
//!
//! ```
//! # use tinybit::*;
//! # use tinybit::events::Event;
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

/// Produce events.
///
/// ```
/// # use tinybit::*;
/// # use tinybit::events::Event;
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

    // Frames
    thread::spawn(move || loop {
        let _ = tx.send(Event::Tick);
        thread::sleep(Duration::from_millis(1000 / fps));
    });

    Events { rx }
}
