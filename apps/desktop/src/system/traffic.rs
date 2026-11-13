use std::collections::VecDeque;
use std::time::{Duration, Instant};

use serde::Serialize;

/// The longest rolling average that widgets can ask for.
pub const LONGEST: Duration = Duration::from_secs(15 * 60);

/// Network throughput in bytes per second.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize)]
pub struct Rate {
    pub down: f64,
    pub up: f64,
}

/// The totals of the bytes received and sent. The host records them at each request and keeps
/// them as long as the longest average needs them. Each push holds all bytes moved since the
/// previous push, so the totals stay correct across gaps betwen requests.
pub struct Traffic(VecDeque<(Instant, u64, u64)>);

impl Traffic {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Records the bytes moved since the previous push.
    pub fn push(&mut self, at: Instant, down: u64, up: u64) {
        let (rx, tx) = self.0.back().map_or((0, 0), |&(_, rx, tx)| (rx, tx));
        self.0.push_back((at, rx + down, tx + up));
        while at - self.0[0].0 > LONGEST {
            self.0.pop_front();
        }
    }

    /// The rate between the last two pushes.
    pub fn latest(&self) -> Rate {
        self.since(self.0.len().saturating_sub(2))
    }

    /// The average over the pushes in the last `window`. When the requests coverred the full
    /// window, the average uses all of it. Right after requests start or resume after a gap, it
    /// uses less.
    pub fn average(&self, window: Duration) -> Rate {
        let Some(&(now, ..)) = self.0.back() else { return Rate::default() };
        self.since(self.0.iter().position(|&(at, ..)| now - at <= window).unwrap_or(0))
    }

    fn since(&self, from: usize) -> Rate {
        let (Some(&(t0, rx0, tx0)), Some(&(t1, rx1, tx1))) = (self.0.get(from), self.0.back()) else {
            return Rate::default();
        };
        let secs = (t1 - t0).as_secs_f64();
        if secs == 0.0 {
            return Rate::default();
        }
        Rate { down: (rx1 - rx0) as f64 / secs, up: (tx1 - tx0) as f64 / secs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECOND: Duration = Duration::from_secs(1);
    const MINUTE: Duration = Duration::from_secs(60);

    /// A baseline push at `at`, as the first request does.
    fn start(at: Instant) -> Traffic {
        let mut traffic = Traffic::new();
        traffic.push(at, 0, 0);
        traffic
    }

    /// Makes `seconds` requests, one second apart. Each request finds `down` bytes received and
    /// `up` bytes sent.
    fn run(traffic: &mut Traffic, at: &mut Instant, seconds: u32, down: u64, up: u64) {
        for _ in 0..seconds {
            *at += SECOND;
            traffic.push(*at, down, up);
        }
    }

    #[test]
    fn steady_traffic_gives_the_same_rate_for_every_window() {
        let mut at = Instant::now();
        let mut traffic = start(at);
        run(&mut traffic, &mut at, 120, 1000, 10);
        let steady = Rate { down: 1000.0, up: 10.0 };
        assert_eq!(traffic.latest(), steady);
        assert_eq!(traffic.average(MINUTE), steady);
    }

    #[test]
    fn averages_over_what_exists_before_the_window_fills() {
        let mut at = Instant::now();
        let mut traffic = start(at);
        run(&mut traffic, &mut at, 10, 1000, 0);
        assert_eq!(traffic.average(5 * MINUTE).down, 1000.0);
    }

    #[test]
    fn longer_windows_smooth_a_burst() {
        let mut at = Instant::now();
        let mut traffic = start(at);
        run(&mut traffic, &mut at, 60, 0, 0);
        run(&mut traffic, &mut at, 60, 1000, 0);
        assert_eq!(traffic.average(MINUTE).down, 1000.0);
        assert_eq!(traffic.average(5 * MINUTE).down, 500.0);
    }

    #[test]
    fn after_a_gap_averages_restart_from_the_requests_made_since() {
        let mut at = Instant::now();
        let mut traffic = start(at);
        run(&mut traffic, &mut at, 60, 1000, 0);
        // There is no request for ten minutes. Then requests start again with a fresh baseline.
        at += 10 * MINUTE;
        traffic.push(at, 0, 0);
        run(&mut traffic, &mut at, 10, 2000, 0);
        assert_eq!(traffic.average(5 * MINUTE).down, 2000.0);
    }

    #[test]
    fn drops_pushes_older_than_the_longest_window() {
        let mut at = Instant::now();
        let mut traffic = start(at);
        run(&mut traffic, &mut at, 2000, 1, 0);
        assert_eq!(traffic.0.len(), 15 * 60 + 1);
        assert_eq!(traffic.average(LONGEST).down, 1.0);
    }
}
