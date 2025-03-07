use ratatui::crossterm::event::{self, Event as CrosstermEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Application events.
///
/// You can extend this enum with your own custom events.
#[derive(Clone, Debug)]
pub enum AppEvent {
    /// Custom increment event.
    Increment,
    /// Custom decrement event.
    Decrement,
    /// Quit the application.
    Quit,
}

/// Representation of all possible events.
#[derive(Clone, Debug)]
pub enum Event {
    /// An event that is emitted on a regular schedule.
    ///
    /// Use this event to run any code which has to run outside of being a direct response to
    /// a user event. e.g. polling exernal systems, updating animations, or rendering the UI
    /// based on a fixed frame rate.
    Tick,
    /// Crossterm events.
    ///
    /// These events are emitted by the terminal.
    Crossterm(CrosstermEvent),
    /// Application events.
    ///
    /// Use this event to emit custom events that are specific to your application.
    App(AppEvent),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new() -> Self {
        let tick_rate = Duration::from_secs_f32(1.0 / 30.0);
        let (sender, receiver) = mpsc::channel();
        let sender_cloned = sender.clone();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("failed to poll new events") {
                    let event = event::read().expect("unable to read event");
                    let _ = sender.send(Event::Crossterm(event));
                }

                if last_tick.elapsed() >= tick_rate {
                    let _ = sender.send(Event::Tick);
                    last_tick = Instant::now();
                }
            }
        });
        Self {
            sender: sender_cloned,
            receiver,
        }
    }

    /// Receives an event from the sender.
    pub fn receive(&self) -> color_eyre::Result<Event> {
        Ok(self.receiver.recv()?)
    }

    /// Sends an event to the receiver.
    pub fn send(&self, event: Event) {
        // The result is ignored because the receiver may have been dropped when the app is
        // shutting down.
        //
        // This is expected behavior and should not panic.
        let _ = self.sender.send(event);
    }
}
