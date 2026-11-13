//! The album art of the latest track that was read. `media_art` returns it, and the host reads
//! the art again only when it can have changed.

use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

/// An app can publish the art of a track after its title, or keep the art of the previous track
/// for a momment. Thus the host reads the art again at each request for this long after a track
/// change.
const ART_SETTLE: Duration = Duration::from_secs(5);

struct Art {
    /// The track of the art: app, title, artist and album.
    track: String,
    id: u64,
    url: Option<String>,
    since: Instant,
}

static ART: Mutex<Option<Art>> = Mutex::new(None);

/// The art of the latest track, as a data URL.
pub fn current() -> Option<String> {
    let cache = ART.lock().unwrap_or_else(PoisonError::into_inner);
    cache.as_ref().and_then(|art| art.url.clone())
}

/// The id of the art of `track`, or `None` if there is no art. `read` gives the art as a data
/// URL. The host calls it after a track change, while the art settles, and while the app gives
/// none.
pub fn id(track: String, read: impl FnOnce() -> Option<String>) -> Option<u64> {
    let mut cache = ART.lock().unwrap_or_else(PoisonError::into_inner);
    let art = match cache.take() {
        Some(art)
            if art.track == track && art.url.is_some() && art.since.elapsed() >= ART_SETTLE =>
        {
            art
        }
        previous => {
            let url = read();
            let same_track = previous.as_ref().filter(|art| art.track == track);
            let since = same_track.map_or_else(Instant::now, |art| art.since);
            let id = match &previous {
                Some(art) if art.url == url => art.id,
                Some(art) => art.id + 1,
                None => 0,
            };
            Art { track, id, url, since }
        }
    };
    let id = art.url.as_ref().map(|_| art.id);
    *cache = Some(art);
    id
}
