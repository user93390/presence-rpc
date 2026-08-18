use miniserde::json;
use std::{
    error::Error,
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    sync::{Arc, Mutex},
};

use crate::{
    DiscordClient, DiscordEvent,
    activity::{DiscordRichPresence, SetActivity},
    bus::EventBus,
    sys,
};

pub struct App {
    stream: Option<UnixStream>,
    bus: EventBus<DiscordEvent>,
}

#[derive(miniserde::MiniSerialize)]
struct Connect {
    v: u8,
    client_id: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            stream: None,
            bus: EventBus::new(),
        }
    }
    pub fn connect(
        &mut self,
        discord: &DiscordClient<DiscordEvent>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        match sys::find_active_discord_ipc() {
            Some(value) => {
                let runtime_dir = value.1;

                self.stream = Some(UnixStream::connect(runtime_dir.clone()).unwrap());
                let stream = UnixStream::connect(runtime_dir)?;

                let shared_stream = Arc::new(Mutex::new(Some(stream)));
                let static_ref: &'static Arc<Mutex<Option<UnixStream>>> =
                    Box::leak(Box::new(shared_stream));

                let id = discord
                    .app_id
                    .expect("Error: you are required to provide an application ID")
                    .to_string();

                let con = Connect {
                    v: 1,
                    client_id: id,
                };

                discord.on_discord_event(DiscordEvent::Disconnected, |_| {
                    let guard = static_ref.lock().expect("Error: lock is poisoned");

                    if let Some(ref s) = *guard {
                        s.shutdown(Shutdown::Both).unwrap();
                    }
                });

                let json = json::to_string(&con);
                if let Some(shared_stream) = self.stream.as_mut() {
                    Self::send_frame(shared_stream, 0, &json)?;
                    Self::read_frame(shared_stream)?;
                }
            }

            None => {
                println!("Failed to find ipc path");
            }
        }

        Ok(())
    }

    pub fn set_activity(
        &mut self,
        activity: &DiscordRichPresence,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {

        let command = SetActivity {
            pid: std::process::id(),
            activity: &activity,
        };

        let json = json::to_string(&command);

        let stream = self.stream.take();

        if let Some(mut stream) = stream {
            Self::send_frame(&mut stream, 1, &json)?;
            Self::read_frame(&mut stream)?;
        }

        println!("{}", &json);

        let event = DiscordEvent::ActivityUpdated(json);

        self.bus.publish(event)?;

        Ok(())
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
