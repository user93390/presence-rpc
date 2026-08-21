use std::time::{SystemTime, UNIX_EPOCH};

use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug)]
pub struct DiscordRichPresence {
    #[nserde(rename = "type")]
    pub(crate) rtype: Option<u8>,
    pub(crate) state: Option<String>,
    details: Option<String>,

    timestamps: Option<Timestamps>,
    assets: Option<Assets>,
    party: Option<Party>,
    secrets: Option<Secrets>,

    instance: Option<bool>,
}

#[derive(DeJson, SerJson, Debug)]
pub struct Timestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(DeJson, SerJson, Debug)]
pub struct Assets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(DeJson, SerJson, Debug)]
pub struct Party {
    id: Option<String>,
    size: Option<[u16; 2]>,
}
#[derive(SerJson)]
pub struct EasyActivity {
    pub(crate) cmd: String,
    pub(crate) args: EasyActivityArgs,
}

#[derive(SerJson)]
pub struct EasyActivityArgs {
    pub(crate) pid: u32,
    pub(crate) activity: DiscordRichPresence,
}

#[derive(DeJson, SerJson, Debug)]
pub struct Secrets {
    #[nserde(rename = "match")]
    rmatch: Option<String>,
    join: Option<String>,
    spectate: Option<String>,
}

impl Default for DiscordRichPresence {
    fn default() -> Self {
        Self {
            rtype: Some(0),
            state: None,
            details: None,
            timestamps: None,
            assets: None,
            party: None,
            secrets: None,
            instance: None,
        }
    }
}

struct DiscordRichPresenceMap<'a> {
    presence: &'a DiscordRichPresence,
    state: usize,
}

struct TimestampsMap<'a> {
    timestamps: &'a Timestamps,
    state: usize,
}

struct AssetsMap<'a> {
    assets: &'a Assets,
    state: usize,
}

struct PartyMap<'a> {
    party: &'a Party,
    state: usize,
}

struct SecretsMap<'a> {
    secrets: &'a Secrets,
    state: usize,
}

impl DiscordRichPresence {
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn details(mut self, value: impl Into<String>) -> Self {
        self.details = Some(value.into());
        self
    }

    pub fn start_time(mut self, value: u64) -> Self {
        self.timestamps
            .get_or_insert(Timestamps {
                start: None,
                end: None,
            })
            .start = Some(value);

        self
    }

    pub fn start_now(mut self) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before Unix epoch")
            .as_secs();

        self.timestamps
            .get_or_insert(Timestamps {
                start: None,
                end: None,
            })
            .start = Some(timestamp);

        self
    }

    pub fn end_time(mut self, value: u64) -> Self {
        self.timestamps
            .get_or_insert(Timestamps {
                start: None,
                end: None,
            })
            .end = Some(value);

        self
    }

    pub fn large_image_key(mut self, value: impl Into<String>) -> Self {
        self.assets
            .get_or_insert(Assets {
                large_image: None,
                large_text: None,
                small_image: None,
                small_text: None,
            })
            .large_image = Some(value.into());

        self
    }

    pub fn large_image_text(mut self, value: impl Into<String>) -> Self {
        self.assets
            .get_or_insert(Assets {
                large_image: None,
                large_text: None,
                small_image: None,
                small_text: None,
            })
            .large_text = Some(value.into());

        self
    }

    pub fn small_image_key(mut self, value: impl Into<String>) -> Self {
        self.assets
            .get_or_insert(Assets {
                large_image: None,
                large_text: None,
                small_image: None,
                small_text: None,
            })
            .small_image = Some(value.into());

        self
    }

    pub fn small_image_text(mut self, value: impl Into<String>) -> Self {
        self.assets
            .get_or_insert(Assets {
                large_image: None,
                large_text: None,
                small_image: None,
                small_text: None,
            })
            .small_text = Some(value.into());

        self
    }

    pub fn party_id(mut self, value: impl Into<String>) -> Self {
        self.party
            .get_or_insert(Party {
                id: None,
                size: None,
            })
            .id = Some(value.into());

        self
    }

    pub fn party_size(mut self, current: u16) -> Self {
        let party = self.party.get_or_insert(Party {
            id: None,
            size: None,
        });

        let max = party.size.map(|size| size[1]).unwrap_or(0);
        party.size = Some([current, max]);

        self
    }

    pub fn party_max_size(mut self, max: u16) -> Self {
        let party = self.party.get_or_insert(Party {
            id: None,
            size: None,
        });

        let current = party.size.map(|size| size[0]).unwrap_or(0);
        party.size = Some([current, max]);

        self
    }

    pub fn match_secret(mut self, value: impl Into<String>) -> Self {
        self.secrets
            .get_or_insert(Secrets {
                rmatch: None,
                join: None,
                spectate: None,
            })
            .rmatch = Some(value.into());

        self
    }

    pub fn join_secret(mut self, value: impl Into<String>) -> Self {
        self.secrets
            .get_or_insert(Secrets {
                rmatch: None,
                join: None,
                spectate: None,
            })
            .join = Some(value.into());

        self
    }

    pub fn spectate_secret(mut self, value: impl Into<String>) -> Self {
        self.secrets
            .get_or_insert(Secrets {
                rmatch: None,
                join: None,
                spectate: None,
            })
            .spectate = Some(value.into());

        self
    }

    pub fn instance(mut self, value: bool) -> Self {
        self.instance = Some(value);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
