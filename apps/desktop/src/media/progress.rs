//! Continues the position of a track when its app no longer reports it. Zen and Firefox publish
//! the timeline on play, pause and seek. Then, about once a minute, they replace it with an
//! empty one.

use std::time::{Duration, Instant};

/// Continue only from a request that is this recent. In a longer gap, the user can pause or
/// seek. The app reports this at that time, but can then replace it with an empty timeline.
/// While the host watches the sessions, a pause or a seek makes the widgets read again, so the
/// limit does not apply to a request after the watch started.
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

/// True if a position that a request saw at `seen` can carry on at `at`. `watched` is the start
/// of the watch that runs.
fn fresh(watched: Option<Instant>, seen: Instant, at: Instant) -> bool {
    at.saturating_duration_since(seen) < CARRY || watched.is_some_and(|start| start <= seen)
}

pub struct Progress {
    seen: Vec<Seen>,
    /// The start of the watch that runs, `None` while no widget listens to changes.
    watched: Option<Instant>,
}

impl Progress {
    pub const fn new() -> Self {
        Self { seen: Vec::new(), watched: None }
    }

    /// Tells when the watch of the sessions started, or `None` when it stopped.
    pub fn watching(&mut self, since: Option<Instant>) {
        self.watched = since;
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
        // A session that cannot carry on is forgotten. This also forgets closed sessions.
        let watched = self.watched;
        self.seen.retain(|seen| fresh(watched, seen.at, at));
        let index = self.seen.iter().position(|seen| seen.session == session);
        let carried = || {
            let seen = index.map(|index| &self.seen[index]).filter(|seen| seen.track == track)?;
            let since =
                at.checked_duration_since(seen.at).filter(|_| fresh(watched, seen.at, at))?;
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
    fn long_gap_carries_on_while_watched() {
        let start = Instant::now();
        let mut progress = Progress::new();
        progress.watching(Some(start));
        progress.update(start, "a", "track", Some((60.0, 300.0)), true, 1.0);
        assert_eq!(
            progress.update(start + 4 * CARRY, "a", "track", None, true, 1.0),
            Some((80.0, 300.0))
        );
    }

    #[test]
    fn watch_that_starts_later_does_not_carry_an_old_read() {
        let start = Instant::now();
        let mut progress = seen(start, true);
        progress.watching(Some(start + SECOND));
        assert_eq!(progress.update(start + CARRY, "a", "track", None, true, 1.0), None);
    }

    #[test]
    fn stopped_watch_limits_the_gap_again() {
        let start = Instant::now();
        let mut progress = Progress::new();
        progress.watching(Some(start));
        progress.update(start, "a", "track", Some((60.0, 300.0)), true, 1.0);
        progress.watching(None);
        assert_eq!(progress.update(start + CARRY, "a", "track", None, true, 1.0), None);
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
