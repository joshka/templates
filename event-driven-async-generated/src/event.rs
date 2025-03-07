use color_eyre::eyre::OptionExt;
use futures::{FutureExt, StreamExt};
use ratatui::crossterm::event::Event as CrosstermEvent;
use std::time::Duration;
use tokio::sync::mpsc;

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
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new() -> Self {
        let tick_rate = Duration::from_secs_f32(1.0 / 30.0);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  _ = _sender.closed() => {
                    break;
                  }
                  _ = tick_delay => {
                    let _= _sender.send(Event::Tick);
                  }
                  Some(Ok(evt)) = crossterm_event => {
                    let _ = _sender.send(Event::Crossterm(evt));
                  }
                };
            }
        });
        Self { sender, receiver }
    }

    /// Receives an event from the sender.
    pub async fn receive(&mut self) -> color_eyre::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
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
