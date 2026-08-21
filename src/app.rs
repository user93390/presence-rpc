use crate::activity::EasyActivity;
use crate::activity::EasyActivityArgs;
use nanoserde::SerJson;
use std::{
    error::Error,
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    sync::{Arc, Mutex},
};
use thiserror::Error;

use crate::{
    DiscordClient, DiscordEvent,
    activity::DiscordRichPresence,
    app::AppError::{CouldntExit, FailedReadFrame, FailedToConnect, NoIpcPathsFound, PoisonedLock},
    bus::{Event, EventBus},
    sys,
};

pub struct App {
    stream: Option<UnixStream>,
    bus: EventBus<DiscordEvent>,
    pub activity: Option<DiscordRichPresence>,
}

#[derive(SerJson)]
struct Connect {
    v: u8,
    client_id: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Cannot find application ID")]
    NoApplicationID,

    #[error("Failed to connect to IPC: {0}")]
    FailedToConnect(String),

    #[error("Poisoned lock")]
    PoisonedLock,

    #[error("Failed to exit properly. {0}")]
    CouldntExit(String),

    #[error("Failed to read frame properly. {0}")]
    FailedReadFrame(String),

    #[error("Failed to write frame properly. {0}")]
    FailedWriteFrame(String),

    #[error("Found not IPC paths. Make sure a discord client is open.")]
    NoIpcPathsFound,
}

impl App {
    pub fn new() -> Self {
        Self {
            stream: None,
            bus: EventBus::new(),
            activity: None,
        }
    }

    pub fn connect(&mut self, discord: &DiscordClient<DiscordEvent>) -> Result<(), AppError> {
        match sys::find_active_discord_ipc() {
            Some(value) => {
                let runtime_dir = value.1;

                let stream = UnixStream::connect(&runtime_dir)
                    .map_err(|err| FailedToConnect(err.to_string()))?;

                self.stream = Some(stream.try_clone().unwrap());

                let shared_stream = Arc::new(Mutex::new(Some(stream)));

                let static_ref: &'static Arc<Mutex<Option<UnixStream>>> =
                    Box::leak(Box::new(shared_stream));

                let id = discord
                    .app_id
                    .ok_or(0)
                    .map_err(|_| AppError::NoApplicationID)?
                    .to_string();

                let con = Connect {
                    v: 1,
                    client_id: id,
                };
                discord.on_discord_event(Event::Disconnected, |_| -> Result<(), AppError> {
                    let guard = static_ref.lock().map_err(|_| PoisonedLock)?;

                    if let Some(ref s) = *guard {
                        s.shutdown(Shutdown::Both)
                            .map_err(|err| CouldntExit(err.to_string()))?;
                    }
                    Ok(())
                });

                let json = &con.serialize_json();
                if let Some(shared_stream) = self.stream.as_mut() {
                    Self::send_frame(shared_stream, 0, &json)
                        .map_err(|err| FailedReadFrame(err.to_string()))?;

                    Self::read_frame(shared_stream)
                        .map_err(|err| FailedReadFrame(err.to_string()))?;
                }
            }

            None => return Err(NoIpcPathsFound),
        }

        Ok(())
    }

    pub fn set_activity(
        &mut self,
        activity: DiscordRichPresence,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let command = EasyActivity {
            cmd: "SET_ACTIVITY".to_string(),
            args: EasyActivityArgs {
                pid: std::process::id(),
                activity,
            },
        };

        let json = &command.serialize_json();

        let stream = self.stream.take();

        if let Some(mut stream) = stream {
            Self::send_frame(&mut stream, 1, &json)?;
            Self::read_frame(&mut stream)?;
        }

        println!("{}", &json);

        let event = DiscordEvent::ActivityUpdated(json.to_string());

        self.bus.publish(event)?;

        Ok(())
    }

    pub fn get_activity(self) -> Option<DiscordRichPresence> {
        self.activity
    }

    fn send_frame(stream: &mut UnixStream, opcode: u32, payload: &str) -> std::io::Result<()> {
        let mut packet = Vec::with_capacity(8 + payload.len());

        packet.extend_from_slice(&opcode.to_le_bytes());
        packet.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        packet.extend_from_slice(payload.as_bytes());

        stream.write_all(&packet)
    }

    fn read_frame(stream: &mut UnixStream) -> std::io::Result<(u32, Vec<u8>)> {
        let mut header = [0u8; 8];

        stream.read_exact(&mut header)?;

        let opcode = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(header[4..8].try_into().unwrap());

        let mut payload = vec![0u8; length as usize];

        stream.read_exact(&mut payload)?;

        Ok((opcode, payload))
    }
}
