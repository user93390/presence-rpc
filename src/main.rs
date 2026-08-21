use std::error::Error;

use presence_rpc::{DiscordBuilder, activity::DiscordRichPresence, app::App};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // replace with your application id
    let client = DiscordBuilder::new(1540201781378686986);
    let mut app = App::new();

    app.connect(&client)?;

    let activity = DiscordRichPresence::default()
        .state("Listening to music")
        .details("Why do programmers hate light mode? Because it attracts more bugs!")
        .start_now();

    app.set_activity(activity)?;

    Ok(())
}
