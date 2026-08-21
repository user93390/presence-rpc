<div align="center">

![Handmade](https://badges.ws/handmade)
![Size](https://badges.ws/crates/size/presence-rpc)

# Presence-Rpc

</div>

Presence-rpc is an incredibly small library that makes creating rich presences easy for your application
This library comes with an easy-to-use api that allows you to have lots of control. Here is a code example

```rust
use std::error::Error;

use presence_rpc::{DiscordBuilder, activity::DiscordRichPresence, app::App};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // replace with your application id
    let client = DiscordBuilder::new(12345678);
    let mut app = App::new();

    app.connect(&client)?;

    let activity = DiscordRichPresence::default()
        .state("Listening to music")
        .details("Why do programmers hate light mode? Because it attracts more bugs!")
        .start_now();

    app.set_activity(&activity)?;

    Ok(())
}
```

## Minimizing Dependencies

We do in fact have support for custom errors via [thiserror](https://crates.io/crates/thiserror).
Presence-rpc only uses 3 dependencies; one of them being [flume](https://crates.io/crates/flume).
Honestly the dependencies I've chosen are lightweight.
There are plenty of other discord rpc, feel free to use them instead!

## For Asynchronous Applications

It is not a garentee that Presence-rpc will work in an async context.
We will not be adding asynchronous support in any time.


## Supported  Operating Systems

| Operating System  | Supported   | Developer's message                       |
|-------------------|-----------  |-------------------------------------------|
| Windows           | ✅          | Supported. Shouldn't be using it though.  |
| Unix (Linux, Mac) | ✅          | Developed on and well tested.             |
| BSD/FreeBSD       | 🚫          | 😉                                        |
