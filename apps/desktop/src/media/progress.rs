//! Continues the position of a track when its app no longer reports it. Zen and Firefox publish
//! the timeline on play, pause and seek. Then, about once a minute, they replace it with an
//! empty one.

use std::time::{Duration, Instant};

/// Continue only from a request that is this recent. In a longer gap, the user can pause or
/// seek. The app reports this at that time, but can then replace it with an empty timeline.
const CARRY: Duration = Duration::from_secs(5);

/// The position of the track of a session at the latest request that knew it.
struct Seen {
    session: String,
    track: String,
    /// In seconds.
    position: f64,
    duration: f64,
    playing: bool,
    rate: f64,
    at: Instant,
}

pub struct Progress {
    seen: Vec<Seen>,
}

impl Progress {
    pub const fn new() -> Self {
        Self { seen: Vec::new() }
    }

    /// Returns the position and the lenght in seconds, as the app of `session` reports them. If
    /// the app reports none, uses the last values for this track plus the time played since then.
    pub fn update(
        &mut self,
        at: Instant,
        session: &str,
        track: &str,
        reported: Option<(f64, f64)>,
        playing: bool,
        rate: f64,
    ) -> Option<(f64, f64)> {
        // A session that no request saw for `CARRY` cannot carry on. This also forgets closed sessions.
        self.seen.retain(|seen| at.saturating_duration_since(seen.at) < CARRY);
        let index = self.seen.iter().position(|seen| seen.session == session);
        let carried = || {
            let seen = index.map(|index| &self.seen[index]).filter(|seen| seen.track == track)?;
            let since = at.checked_duration_since(seen.at).filter(|&since| since < CARRY)?;
            let played = if seen.playing { since.as_secs_f64() * seen.rate } else { 0.0 };
            Some(((seen.position + played).min(seen.duration), seen.duration))
        };
        let progress = reported.or_else(carried);
        if let Some(index) = index {
            self.seen.swap_remove(index);
        }
        if let Some((position, duration)) = progress {
            self.seen.push(Seen {
                session: session.to_owned(),
                track: track.to_owned(),
                position,
                duration,
                playing,
                rate,
                at,
            });
        }
        progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECOND: Duration = Duration::from_secs(1);

    /// A `Progress` that saw `track` of session `a` at 60 s of 300 s, at time `at`.
    fn seen(at: Instant, playing: bool) -> Progress {
        let mut progress = Progress::new();
        progress.update(at, "a", "track", Some((60.0, 300.0)), playing, 1.0);
        progress
    }

    #[test]
    fn reported_timeline_wins() {
        let start = Instant::now();
        let mut progress = seen(start, true);
        let shown = progress.update(start + SECOND, "a", "track", Some((10.0, 300.0)), true, 1.0);
        assert_eq!(shown, Some((10.0, 300.0)));
    }

    #[test]
    fn empty_timeline_carries_on_while_playing() {
        let start = Instant::now();
        let mut progress = seen(start, true);
        assert_eq!(
            progress.update(start + SECOND, "a", "track", None, true, 1.0),
            Some((61.0, 300.0))
        );
        // Each request continues from the previous one, so the postion moves on.
        assert_eq!(
            progress.update(start + 3 * SECOND, "a", "track", None, true, 1.0),
            Some((63.0, 300.0))
        );
        assert_eq!(
            progress.update(start + 7 * SECOND, "a", "track", None, true, 1.0),
            Some((67.0, 300.0))
        );
    }

    #[test]
    fn empty_timeline_holds_while_paused() {
        let start = Instant::now();
        let mut progress = seen(start, false);
        assert_eq!(
            progress.update(start + 2 * SECOND, "a", "track", None, false, 1.0),
            Some((60.0, 300.0))
        );
    }

    #[test]
    fn playback_rate_counts() {
        let start = Instant::now();
        let mut progress = Progress::new();
        progress.update(start, "a", "track", Some((60.0, 300.0)), true, 2.0);
        assert_eq!(
            progress.update(start + SECOND, "a", "track", None, true, 2.0),
            Some((62.0, 300.0))
        );
    }

    #[test]
    fn stops_at_the_end() {
        let start = Instant::now();
        let mut progress = Progress::new();
        progress.update(start, "a", "track", Some((299.0, 300.0)), true, 1.0);
        assert_eq!(
            progress.update(start + 3 * SECOND, "a", "track", None, true, 1.0),
            Some((300.0, 300.0))
        );
    }

    #[test]
    fn other_track_is_not_carried() {
        let start = Instant::now();
        let mut progress = seen(start, true);
        assert_eq!(progress.update(start + SECOND, "a", "next track", None, true, 1.0), None);
        // The session forgot the first track, so it does not come back.
        assert_eq!(progress.update(start + 2 * SECOND, "a", "track", None, true, 1.0), None);
    }

    #[test]
    fn long_gap_is_not_carried() {
        let start = Instant::now();
        let mut progress = seen(start, true);
        assert_eq!(progress.update(start + CARRY, "a", "track", None, true, 1.0), None);
        // And not later, because the host forgot the track.
        assert_eq!(progress.update(start + CARRY + SECOND, "a", "track", None, true, 1.0), None);
    }

    #[test]
    fn sessions_carry_on_apart() {
        let start = Instant::now();
        let mut progress = seen(start, true);
        progress.update(start, "b", "track", Some((10.0, 100.0)), false, 1.0);
        assert_eq!(
            progress.update(start + SECOND, "a", "track", None, true, 1.0),
            Some((61.0, 300.0))
        );
        assert_eq!(
            progress.update(start + SECOND, "b", "track", None, false, 1.0),
            Some((10.0, 100.0))
        );
    }
}
