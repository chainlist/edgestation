//! The album art of the latest track of each session that was read. `media_art` returns it, and
//! the host reads the art again only when it can have changed.

use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

/// An app can publish the art of a track after its title, or keep the art of the previous track
/// for a momment. Thus the host reads the art again at each request for this long after a track
/// change.
const ART_SETTLE: Duration = Duration::from_secs(5);
/// The host forgets the art of a session that no request read for this long. The session can be
/// closed, and the art is large.
const FORGET: Duration = Duration::from_secs(30);

struct Art {
    session: String,
    /// The track of the art: title, artist and album.
    track: String,
    id: u64,
    url: Option<String>,
    since: Instant,
    seen: Instant,
}

struct Cache {
    arts: Vec<Art>,
    /// The session that the latest read found current.
    current: Option<String>,
    /// The id of the next art. Each art gets a new id, so the id of an other session is different.
    next: u64,
}

static CACHE: Mutex<Cache> = Mutex::new(Cache { arts: Vec::new(), current: None, next: 0 });

/// The art of the latest track of `session`, as a data URL. Without `session`, the art of the
/// session that the latest read found current.
pub fn get(session: Option<&str>) -> Option<String> {
    let cache = CACHE.lock().unwrap_or_else(PoisonError::into_inner);
    let session = session.or(cache.current.as_deref())?;
    let art = cache.arts.iter().find(|art| art.session == session)?;
    art.url.clone()
}

/// The id of the art of `track` in `session`, or `None` if there is no art. `read` gives the art
/// as a data URL. The host calls it after a track change, while the art settles, and while the
/// app gives none.
pub fn id(
    session: &str,
    track: String,
    current: bool,
    read: impl FnOnce() -> Option<String>,
) -> Option<u64> {
    let now = Instant::now();
    let mut cache = CACHE.lock().unwrap_or_else(PoisonError::into_inner);
    cache.arts.retain(|art| now.saturating_duration_since(art.seen) < FORGET);
    if current {
        cache.current = Some(session.to_owned());
    }
    let index = cache.arts.iter().position(|art| art.session == session);
    let previous = index.map(|index| cache.arts.swap_remove(index));
    let mut art = match previous {
        Some(art)
            if art.track == track && art.url.is_some() && art.since.elapsed() >= ART_SETTLE =>
        {
            art
        }
        previous => {
            let url = read();
            let same_track = previous.as_ref().filter(|art| art.track == track);
            let since = same_track.map_or(now, |art| art.since);
            let id = match &previous {
                Some(art) if art.url == url => art.id,
                _ => {
                    cache.next += 1;
                    cache.next
                }
            };
            Art { session: session.to_owned(), track, id, url, since, seen: now }
        }
    };
    art.seen = now;
    let id = art.url.as_ref().map(|_| art.id);
    cache.arts.push(art);
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tests share the cache, so each one uses its own sessions.
    fn art(name: &str) -> Option<String> {
        Some(format!("data:image/png;base64,{name}"))
    }

    #[test]
    fn keeps_the_art_of_each_session() {
        let a = id("keep-a", "song".into(), true, || art("a"));
        let b = id("keep-b", "film".into(), false, || art("b"));
        assert!(a.is_some() && b.is_some() && a != b);
        assert_eq!(get(Some("keep-a")), art("a"));
        assert_eq!(get(Some("keep-b")), art("b"));
    }

    #[test]
    fn same_art_keeps_its_id() {
        let first = id("same", "song".into(), false, || art("a"));
        // The art settles, so the host reads it again and finds the same art.
        let again = id("same", "song".into(), false, || art("a"));
        assert_eq!(first, again);
        let changed = id("same", "song".into(), false, || art("b"));
        assert_ne!(changed, first);
        assert_eq!(id("same", "song".into(), false, || None), None);
    }
}
