use miniserde::ser::{Fragment, Map};
use miniserde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct DiscordRichPresence {
    pub(crate) r#type: Option<u8>,
    pub(crate) state: Option<String>,
    details: Option<String>,

    timestamps: Option<Timestamps>,
    assets: Option<Assets>,
    party: Option<Party>,
    secrets: Option<Secrets>,

    instance: Option<bool>,
}

#[derive(Deserialize)]
pub struct Timestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Deserialize)]
pub struct Assets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Deserialize)]
pub struct Party {
    id: Option<String>,
    size: Option<[u16; 2]>,
}

#[derive(Deserialize)]
pub struct Secrets {
    r#match: Option<String>,
    join: Option<String>,
    spectate: Option<String>,
}

impl Serialize for DiscordRichPresence {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(DiscordRichPresenceMap {
            presence: self,
            state: 0,
        }))
    }
}

struct DiscordRichPresenceMap<'a> {
    presence: &'a DiscordRichPresence,
    state: usize,
}

impl<'a> Map for DiscordRichPresenceMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        loop {
            let state = self.state;
            self.state += 1;

            match state {
                0 if self.presence.r#type.is_some() => {
                    return Some((
                        Cow::Borrowed("type"),
                        self.presence.r#type.as_ref().unwrap(),
                    ));
                }

                1 if self.presence.state.is_some() => {
                    return Some((
                        Cow::Borrowed("state"),
                        self.presence.state.as_ref().unwrap(),
                    ));
                }

                2 if self.presence.details.is_some() => {
                    return Some((
                        Cow::Borrowed("details"),
                        self.presence.details.as_ref().unwrap(),
                    ));
                }

                3 if self.presence.timestamps.is_some() => {
                    return Some((
                        Cow::Borrowed("timestamps"),
                        self.presence.timestamps.as_ref().unwrap(),
                    ));
                }

                4 if self.presence.assets.is_some() => {
                    return Some((
                        Cow::Borrowed("assets"),
                        self.presence.assets.as_ref().unwrap(),
                    ));
                }

                5 if self.presence.party.is_some() => {
                    return Some((
                        Cow::Borrowed("party"),
                        self.presence.party.as_ref().unwrap(),
                    ));
                }

                6 if self.presence.secrets.is_some() => {
                    return Some((
                        Cow::Borrowed("secrets"),
                        self.presence.secrets.as_ref().unwrap(),
                    ));
                }

                7 if self.presence.instance.is_some() => {
                    return Some((
                        Cow::Borrowed("instance"),
                        self.presence.instance.as_ref().unwrap(),
                    ));
                }

                _ => return None,
            }
        }
    }
}

impl Serialize for Timestamps {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(TimestampsMap {
            timestamps: self,
            state: 0,
        }))
    }
}

struct TimestampsMap<'a> {
    timestamps: &'a Timestamps,
    state: usize,
}

impl<'a> Map for TimestampsMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        loop {
            let state = self.state;
            self.state += 1;

            match state {
                0 if self.timestamps.start.is_some() => {
                    return Some((
                        Cow::Borrowed("start"),
                        self.timestamps.start.as_ref().unwrap(),
                    ));
                }

                1 if self.timestamps.end.is_some() => {
                    return Some((Cow::Borrowed("end"), self.timestamps.end.as_ref().unwrap()));
                }

                _ => return None,
            }
        }
    }
}

impl Serialize for Assets {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(AssetsMap {
            assets: self,
            state: 0,
        }))
    }
}

struct AssetsMap<'a> {
    assets: &'a Assets,
    state: usize,
}

impl<'a> Map for AssetsMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        loop {
            let state = self.state;
            self.state += 1;

            match state {
                0 if self.assets.large_image.is_some() => {
                    return Some((
                        Cow::Borrowed("large_image"),
                        self.assets.large_image.as_ref().unwrap(),
                    ));
                }

                1 if self.assets.large_text.is_some() => {
                    return Some((
                        Cow::Borrowed("large_text"),
                        self.assets.large_text.as_ref().unwrap(),
                    ));
                }

                2 if self.assets.small_image.is_some() => {
                    return Some((
                        Cow::Borrowed("small_image"),
                        self.assets.small_image.as_ref().unwrap(),
                    ));
                }

                3 if self.assets.small_text.is_some() => {
                    return Some((
                        Cow::Borrowed("small_text"),
                        self.assets.small_text.as_ref().unwrap(),
                    ));
                }

                _ => return None,
            }
        }
    }
}

impl Serialize for Party {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(PartyMap {
            party: self,
            state: 0,
        }))
    }
}

struct PartyMap<'a> {
    party: &'a Party,
    state: usize,
}

impl<'a> Map for PartyMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        loop {
            let state = self.state;
            self.state += 1;

            match state {
                0 if self.party.id.is_some() => {
                    return Some((Cow::Borrowed("id"), self.party.id.as_ref().unwrap()));
                }

                1 if self.party.size.is_some() => {
                    return Some((Cow::Borrowed("size"), self.party.size.as_ref().unwrap()));
                }

                _ => return None,
            }
        }
    }
}

impl Serialize for Secrets {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(SecretsMap {
            secrets: self,
            state: 0,
        }))
    }
}

struct SecretsMap<'a> {
    secrets: &'a Secrets,
    state: usize,
}

impl<'a> Map for SecretsMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        loop {
            let state = self.state;
            self.state += 1;

            match state {
                0 if self.secrets.r#match.is_some() => {
                    return Some((
                        Cow::Borrowed("match"),
                        self.secrets.r#match.as_ref().unwrap(),
                    ));
                }

                1 if self.secrets.join.is_some() => {
                    return Some((Cow::Borrowed("join"), self.secrets.join.as_ref().unwrap()));
                }

                2 if self.secrets.spectate.is_some() => {
                    return Some((
                        Cow::Borrowed("spectate"),
                        self.secrets.spectate.as_ref().unwrap(),
                    ));
                }

                _ => return None,
            }
        }
    }
}

impl DiscordRichPresence {
    pub fn new() -> Self {
        Self {
            r#type: Some(0),
            state: None,
            details: None,
            timestamps: None,
            assets: None,
            party: None,
            secrets: None,
            instance: None,
        }
    }

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
                r#match: None,
                join: None,
                spectate: None,
            })
            .r#match = Some(value.into());

        self
    }

    pub fn join_secret(mut self, value: impl Into<String>) -> Self {
        self.secrets
            .get_or_insert(Secrets {
                r#match: None,
                join: None,
                spectate: None,
            })
            .join = Some(value.into());

        self
    }

    pub fn spectate_secret(mut self, value: impl Into<String>) -> Self {
        self.secrets
            .get_or_insert(Secrets {
                r#match: None,
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

pub struct SetActivity<'a> {
    pub pid: u32,
    pub activity: &'a DiscordRichPresence,
}

impl<'a> Serialize for SetActivity<'a> {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(SetActivityMap {
            args: SetActivityArgs {
                pid: self.pid,
                activity: self.activity,
            },
            state: 0,
        }))
    }
}

struct SetActivityMap<'a> {
    args: SetActivityArgs<'a>,
    state: usize,
}

impl<'a> Map for SetActivityMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        let result = match self.state {
            0 => Some((Cow::Borrowed("cmd"), &"SET_ACTIVITY" as &dyn Serialize)),

            1 => Some((Cow::Borrowed("args"), &self.args as &dyn Serialize)),

            _ => None,
        };

        self.state += 1;
        result
    }
}

struct SetActivityArgs<'a> {
    pid: u32,
    activity: &'a DiscordRichPresence,
}

impl<'a> Serialize for SetActivityArgs<'a> {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Map(Box::new(SetActivityArgsMap {
            args: self,
            state: 0,
        }))
    }
}

struct SetActivityArgsMap<'a> {
    args: &'a SetActivityArgs<'a>,
    state: usize,
}

impl<'a> Map for SetActivityArgsMap<'a> {
    fn next(&mut self) -> Option<(Cow<'static, str>, &dyn Serialize)> {
        let result = match self.state {
            0 => Some((Cow::Borrowed("pid"), &self.args.pid as &dyn Serialize)),

            1 => Some((
                Cow::Borrowed("activity"),
                self.args.activity as &dyn Serialize,
            )),

            _ => None,
        };

        self.state += 1;
        result
    }
}
