use flume::{Receiver, Sender, bounded};
use std::borrow::Cow;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BusError {
    #[error("Failed to acquire subscriber lock: {0}")]
    LockError(Cow<'static, str>),
}

#[derive(PartialEq, Eq)]
pub enum Event {
    Connected,
    ActivityUpdated,
    Disconnected,
}

#[derive(Clone)]
pub struct EventBus<E> {
    subscribers: Arc<Mutex<Vec<Sender<E>>>>,
}

impl<E> Default for EventBus<E> {
    fn default() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<E: Clone + Send + 'static> EventBus<E> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self) -> Result<Receiver<E>, BusError> {
        let (tx, rx) = bounded(32);

        let mut subscribers = self
            .subscribers
            .lock()
            .map_err(|_| BusError::LockError(Cow::Borrowed("Mutex poisoned")))?;

        subscribers.push(tx);
        Ok(rx)
    }

    pub fn publish(&self, event: E) -> Result<usize, BusError> {
        let mut subscribers = self
            .subscribers
            .lock()
            .map_err(|_| BusError::LockError(Cow::Borrowed("Mutex poisoned")))?;

        subscribers.retain(|tx| tx.send(event.clone()).is_ok());

        Ok(subscribers.len())
    }
}
