//! The Linux side: `/proc`, which sysinfo reads.

use super::Process;

/// Linux keeps the CPU time of each process in `/proc`, and shows it for the processes of all
/// users. sysinfo reads it.
#[derive(Default)]
pub(super) struct Lister {
    system: sysinfo::System,
}

impl Lister {
    /// All processes with their CPU time. The threads of a process count with the process.
    pub(super) fn processes(&mut self) -> Vec<Process> {
        use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, UpdateKind};

        let only = ProcessRefreshKind::nothing()
            .with_cpu()
            .with_exe(UpdateKind::OnlyIfNotSet)
            .without_tasks();
        self.system.refresh_processes_specifics(ProcessesToUpdate::All, true, only);
        let processes = self.system.processes().iter();
        processes
            .map(|(pid, process)| {
                // The name of the process is cut at 15 bytes, so prefer the name of the file.
                let file = process.exe().and_then(|exe| exe.file_name());
                let name = file.unwrap_or(process.name()).to_string_lossy().into_owned();
                Process {
                    key: (pid.as_u32() as usize, process.start_time() as i64),
                    name,
                    time: cpu_time_units(process.accumulated_cpu_time()),
                }
            })
            .collect()
    }
}

/// Changes milliseconds of CPU time into the units of `Process::time`, 100 ns.
fn cpu_time_units(milliseconds: u64) -> u64 {
    milliseconds * 10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_cpu_time_in_units_of_100_ns() {
        assert_eq!(cpu_time_units(1), 10_000);
        assert_eq!(cpu_time_units(1_500), 15_000_000);
    }
}
