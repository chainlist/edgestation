//! The programs that use the most CPU, for widgets that show them. As in the rest of `system`,
//! the host reads them when a widget asks, and nothing runs between requests. Usage is the CPU
//! time that each program used since the previous request. A program is all the processes that
//! show the same name.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

use std::collections::HashMap;
use std::num::NonZero;
use std::thread;
use std::time::Instant;

use serde::Serialize;
use sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;

use super::{SHARE, STALE};
use crate::process::{image_path, program_name};
use native::Lister;

#[derive(Clone, Serialize)]
pub struct Program {
    /// The name that Task Manager shows, for example `THE FINALS` or `Google Chrome`.
    name: String,
    /// Percent of all logical processors, as in Task Manager.
    cpu: f64,
}

/// A process id and the start time of the proces. Thus a reused id gives a different key.
type Key = (usize, i64);

struct Process {
    key: Key,
    /// The file name of the executable.
    name: String,
    /// The CPU time that the process used since it started, in units of 100 ns.
    time: u64,
}

/// The first request makes the reader. It keeps the values that the next request compares
/// with.
pub struct Reader {
    /// The number of logical processors. CPU usage is a share of all of them.
    cpus: f64,
    /// Lists the processes. It keeps what the platform reuses from one list to the next.
    lister: Lister,
    /// The time of the last process list, and the CPU time of each process at that time.
    times: (Instant, HashMap<Key, u64>),
    /// The name of each executable seen so far, by path. The host reads each file only once.
    names: HashMap<String, String>,
    last: Option<(Instant, Vec<Program>)>,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            cpus: thread::available_parallelism().map_or(1, NonZero::get) as f64,
            lister: Lister::default(),
            times: (Instant::now(), HashMap::new()),
            names: HashMap::new(),
            last: None,
        }
    }

    /// All programs that used CPU since the previous request, busiest first.
    pub fn read(&mut self) -> Vec<Program> {
        if let Some((at, programs)) = &self.last
            && at.elapsed() < SHARE
        {
            return programs.clone();
        }
        // Usage is a difference between two measures, so measure twice if no request is recent.
        if self.last.as_ref().is_none_or(|(at, _)| at.elapsed() > STALE) {
            self.measure();
            thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        }
        let mut used: HashMap<String, f64> = HashMap::new();
        for (process, cpu) in self.measure() {
            *used.entry(self.name(process)).or_default() += cpu;
        }
        let mut programs: Vec<_> =
            used.into_iter().map(|(name, cpu)| Program { name, cpu }).collect();
        programs.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
        self.last = Some((self.times.0, programs.clone()));
        programs
    }

    /// Lists the processes and returns the ones that used CPU since the previous measure. Each
    /// one comes with the percent of all logical processors that it used.
    fn measure(&mut self) -> Vec<(Process, f64)> {
        let processes = self.lister.processes();
        let at = Instant::now();
        let (previous_at, previous) = &self.times;
        // The CPU time of all logical processors between the two measures, in units of 100 ns.
        let available = (at - *previous_at).as_secs_f64() * 1e7 * self.cpus;

        let mut times = HashMap::with_capacity(processes.len());
        let mut used = Vec::new();
        for process in processes {
            // A process that is not in the previous measure started after it, so all its time
            // is new.
            let before = previous.get(&process.key).copied().unwrap_or(0);
            let since = process.time.saturating_sub(before);
            times.insert(process.key, process.time);
            if since > 0 {
                used.push((process, since as f64 / available * 100.0));
            }
        }
        self.times = (at, times);
        used
    }

    /// The name of a process: the name that people know its executable by (see
    /// `process::program_name`), or else its file name. Some system processes have no file at
    /// all.
    fn name(&mut self, process: Process) -> String {
        let Some(path) = image_path(process.key.0) else { return process.name };
        let name = self.names.entry(path);
        name.or_insert_with_key(|path| program_name(path).unwrap_or(process.name)).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads the real processes. Run it with `--ignored --nocapture` to see the busiest ones.
    #[test]
    #[ignore]
    fn reads_the_busiest_programs() {
        let mut reader = Reader::new();
        let busy = std::thread::spawn(|| (0..u64::MAX).take(300_000_000).sum::<u64>());
        let programs = reader.read();
        busy.join().unwrap();
        for program in programs.iter().take(10) {
            println!("{:>6.2}% {}", program.cpu, program.name);
        }
        assert!(!programs.is_empty());
    }
}
