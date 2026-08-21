<div align="center">

![Handmade](https://badges.ws/handmade)
![Size](https://badges.ws/crates/size/presence-rpc)

# Presence-Rpc

</div>

<p>Presence-rpc is an incredibly small library that makes creating rich presences easy for your application.</p>
<p>This library comes with an easy-to-use api that allows you to have lots of control. Here is a code example</p>

```rust
use std::error::Error;

use presence_rpc::{DiscordBuilder, activity::DiscordRichPresence, app::App};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // replace with your application id
    let client = DiscordBuilder::new(12345678);
    let mut app = App::new();

    app.connect(&client)?;

    let activity = DiscordRichPresence::new()
        .state("Listening to music")
        .details("Why do programmers hate light mode? Because it attracts more bugs!")
        .start_now();

    app.set_activity(&activity)?;

    Ok(())
}
```

<h2>Minimizing Dependencies</h2>
Presence-rpc only uses 3 dependencies; one of them being [flume](https://crates.io/crates/flume).
Honestly most of the dependencies I've chosen are very lightweight and don't require many dependencies
themselfs. We do in fact have support for custom errors via [thiserror](https://crates.io/crates/thiserror), including JSON parsing via [miniserde](https://crates.io/crates/miniserde), a superset to serde to reduce general library size.



<h2>For Asynchronous Applications</h2>

<p>It is not a garentee that Presence-rpc will work in an async context. We will not be adding asynchronous support in any time.</p>


<h2>Supported  Operating Systems</h2>

| Operating System  	| Supported   | Developer's message                       |
|-------------------	|-----------  |-------------------------------------------|
| Windows           	| ✅          | Supported. Shouldn't be using it though.  |
| Unix (Linux, Mac) 	| ✅          | Developed on and well tested.             |
| BSD/FreeBSD           | 🚫          | 😉                                        |
