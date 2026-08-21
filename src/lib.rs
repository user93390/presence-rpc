use crate::app::AppError;
use crate::bus::{Event, EventBus};
use std::fmt::{self};

use std::{
    error::Error,
    fmt::{Debug, Display},
    thread,
};
pub mod activity;
pub mod app;
mod bus;
mod sys;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscordEvent {
    Connected { pid: usize },
    ActivityUpdated(String),
    Disconnected,
}

impl DiscordEvent {
    pub fn kind(&self) -> Event {
        match self {
            DiscordEvent::ActivityUpdated(_) => Event::ActivityUpdated,
            DiscordEvent::Disconnected => Event::Disconnected,
            DiscordEvent::Connected { pid } => Event::Connected,
        }
    }
}

impl Display for DiscordEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscordEvent::Connected { .. } => write!(f, "Connected"),
            DiscordEvent::ActivityUpdated(_) => write!(f, "ActivityUpdated"),
            DiscordEvent::Disconnected => write!(f, "Disconnected"),
        }
    }
}

pub struct DiscordClient<E: Debug + Display> {
    app_id: Option<u64>,
    bus: EventBus<E>,
}

pub struct DiscordBuilder {
    id: u64,
}

impl DiscordBuilder {
    pub fn new(id: u64) -> DiscordClient<DiscordEvent> {
        DiscordClient {
            app_id: Some(id),
            bus: EventBus::new(),
        }
    }
}

type ResultError = Box<dyn Error + Sync + Send>;

impl<T: PartialEq + From<DiscordEvent>> DiscordClient<T>
where
    T: Send + Clone + Sync + Debug + Into<DiscordEvent> + Display + 'static,
{
    pub fn new(id: u64) -> Self {
        Self {
            app_id: Some(id),
            bus: EventBus::new(),
        }
    }

    pub fn set_activity<M>(&self, content: M) -> Result<(), ResultError>
    where
        M: Display + ToString + Into<String>,
    {
        self.bus
            .publish(DiscordEvent::ActivityUpdated(content.into()).into())?;

        Ok(())
    }

    pub fn on_discord_event<F>(&self, req: Event, f: F)
    where
        F: Fn(DiscordEvent) -> Result<(), AppError> + Send + Sync + 'static,
    {
        let rx = self.bus.subscribe().unwrap();

        thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                let event: DiscordEvent = event.into();

                if event.kind() == req {
                    if let Err(err) = f(event) {
                        eprintln!("Discord event error: {err}");
                    }
                }
            }
        });
    }

    pub fn disconnect(&self) -> Result<(), ResultError> {
        self.bus.publish(DiscordEvent::Disconnected.into())?;

        Ok(())
    }
}
