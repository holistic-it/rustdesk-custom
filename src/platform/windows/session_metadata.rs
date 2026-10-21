use serde_json::{Map, Value};
use std::{
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

const MAX_AGE: Duration = Duration::from_secs(5);

#[derive(Clone)]
struct SessionAccount {
    sid: u32,
    account: String,
    locked: Option<bool>,
}

type Sample = Option<(Instant, Option<SessionAccount>)>;

fn cache() -> &'static Arc<Mutex<Sample>> {
    static CACHE: OnceLock<Arc<Mutex<Sample>>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let cache = Arc::new(Mutex::new(None));
        let destination = cache.clone();
        if let Err(error) = std::thread::Builder::new()
            .name("session-account".to_owned())
            .spawn(move || loop {
                let started = Instant::now();
                let account = super::get_current_process_session_id()
                    .filter(|sid| *sid > 0)
                    .and_then(query);
                *destination.lock().unwrap() = Some((started, account));
                std::thread::sleep(Duration::from_secs(2));
            })
        {
            hbb_common::log::error!("Cannot start session metadata collection: {error}");
        }
        cache
    })
}

fn query(sid: u32) -> Option<SessionAccount> {
    extern "C" {
        fn get_session_account_state(
            sid: u32,
            account: *mut u16,
            capacity: u32,
            locked: *mut i32,
        ) -> i32;
    }
    let mut account = [0u16; 512];
    let mut locked = -1;
    if unsafe {
        get_session_account_state(sid, account.as_mut_ptr(), account.len() as u32, &mut locked)
    } == 0
    {
        hbb_common::log::trace!("Windows session account metadata is unavailable");
        return None;
    }
    let length = account.iter().position(|value| *value == 0)?;
    let account = String::from_utf16(&account[..length]).ok()?;
    Some(SessionAccount {
        sid,
        account,
        locked: match locked {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        },
    })
}

pub fn username(sid: u32) -> String {
    query(sid).map(|value| value.account).unwrap_or_default()
}

pub fn snapshot() -> Option<(u32, String, Option<bool>)> {
    // WTS can hang indefinitely. One worker owns every query; packet and display
    // threads only read its short-lived cache and publish unknown after expiry.
    let sample = cache().lock().unwrap().clone();
    sample
        .and_then(|(at, value)| (at.elapsed() <= MAX_AGE).then_some(value).flatten())
        .map(|value| (value.sid, value.account, value.locked))
}

pub fn augment_peer_info(peer: &mut base::message_proto::PeerInfo) {
    let mut additions = if peer.platform_additions.is_empty() {
        Map::new()
    } else {
        match serde_json::from_str::<Map<String, Value>>(&peer.platform_additions) {
            Ok(value) => value,
            Err(error) => {
                hbb_common::log::trace!("Cannot augment session metadata: {error}");
                return;
            }
        }
    };
    let current = snapshot();
    let sid = current
        .as_ref()
        .map(|value| value.0)
        .or_else(|| super::get_current_process_session_id().filter(|sid| *sid > 0));
    peer.platform = "Windows".to_owned();
    if let Some((_, account, _)) = &current {
        peer.username = account.clone();
    }
    if peer.windows_sessions.is_none() {
        if let Some(current_sid) = sid {
            peer.windows_sessions = Some(base::message_proto::WindowsSessions {
                current_sid,
                ..Default::default()
            })
            .into();
        }
    }
    additions.insert(
        "is_screen_locked".to_owned(),
        current
            .and_then(|value| value.2)
            .map(Value::from)
            .unwrap_or(Value::Null),
    );
    match serde_json::to_string(&additions) {
        Ok(value) => peer.platform_additions = value,
        Err(error) => hbb_common::log::trace!("Cannot serialize session metadata: {error}"),
    }
}
