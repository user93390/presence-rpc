use std::path::PathBuf;

pub fn find_active_discord_ipc() -> Option<(u8, PathBuf)> {
    #[cfg(windows)]
    {
        scan_windows()
    }

    #[cfg(unix)]
    {
        scan_unix()
    }

    #[cfg(not(any(windows, unix)))]
    {
        None
    }
}

/// see more at:
/// https://docs.discord.com/developers/topics/rpc
#[cfg(windows)]
pub(crate) fn scan_windows() -> Option<(u8, PathBuf)> {
    use std::fs::OpenOptions;

    for index in 0..=10 {
        let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", index));
        if OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .is_ok()
        {
            return Some((index, path));
        }
    }
    None
}

#[cfg(unix)]
pub(crate) fn scan_unix() -> Option<(u8, PathBuf)> {
    use std::env;
    use std::os::unix::net::UnixStream;

    let mut dirs = Vec::new();
    for var in &["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"] {
        if let Ok(val) = env::var(var) {
            if !val.is_empty() {
                dirs.push(PathBuf::from(val));
            }
        }
    }

    dirs.push(PathBuf::from("/tmp"));

    for dir in dirs {
        for index in 0..=10 {
            let socket_path = dir.join(format!("discord-ipc-{}", index));
            if UnixStream::connect(&socket_path).is_ok() {
                return Some((index, socket_path));
            }
        }
    }

    None
}
