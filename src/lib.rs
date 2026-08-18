use crate::bus::EventBus;
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
        DiscordClient {
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

    pub fn on_discord_event<F, G>(&self, req: DiscordEvent, f: F)
    where
        F: FnOnce(DiscordEvent) -> G + Sync + Send + Copy + 'static,
        G: Sync + Send,
    {
        let rx2 = self.bus.subscribe().unwrap();
        let req_t: T = req.into();

        thread::spawn(move || {
            while let Ok(event) = rx2.recv() {
                if req_t == event {
                    f(event.into());
                }
            }
        });
    }

    pub fn disconnect(&self) -> Result<(), ResultError> {
        self.bus.publish(DiscordEvent::Disconnected.into())?;

        Ok(())
    }
}
