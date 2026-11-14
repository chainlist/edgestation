//! The performance of the machine for widgets. The host reads it when a widget asks, and nothing
//! runs between requests. Load and traffic are measured since the previous request. The network
//! averages come from the totals recorded at past requests.

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod native;

mod programs;
mod traffic;

use std::sync::{Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use serde::Serialize;
use sysinfo::{CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, Networks, RefreshKind, System};

use crate::blocking;

use programs::Program;
use traffic::{Rate, Traffic};

/// Requests closer together than this share one reading. Thus several widgets cost one read, and
/// the CPU load allways has enough time between two measures.
const SHARE: Duration = Duration::from_millis(500);
/// A previous request older than this is not a good baseline for the current values. Take a
/// fresh baseline.
const STALE: Duration = Duration::from_secs(3);

#[derive(Clone, Serialize)]
struct Cpu {
    /// Percent of all cores.
    load: f32,
}

#[derive(Clone, Serialize)]
struct Gpu {
    /// Percent.
    load: u32,
    /// Degrees Celsius.
    temperature: u32,
}

#[derive(Clone, Serialize)]
struct Net {
    /// In bytes per second, since the previous request.
    down: f64,
    up: f64,
    avg1m: Rate,
    avg5m: Rate,
    avg15m: Rate,
}

#[derive(Clone, Serialize)]
pub struct SystemInfo {
    hostname: String,
    wifi: Option<String>,
    cpu: Cpu,
    /// The first NVIDIA GPU. `None` if their is no NVIDIA GPU or no driver for it.
    gpu: Option<Gpu>,
    net: Net,
}

/// The first request makes the reader. It keeps the values that the next request compares
/// with.
struct Reader {
    cpu: System,
    networks: Networks,
    nvml: Option<Nvml>,
    traffic: Traffic,
    last: Option<(Instant, SystemInfo)>,
}

static READER: Mutex<Option<Reader>> = Mutex::new(None);

#[tauri::command]
pub async fn system_info() -> Result<SystemInfo, String> {
    // Run on a blocking thread, because a read can wait a short time for a fresh baseline.
    blocking::run(|| {
        let mut reader = READER.lock().unwrap_or_else(PoisonError::into_inner);
        Ok(reader.get_or_insert_with(Reader::new).read())
    })
    .await
}

static PROGRAMS: Mutex<Option<programs::Reader>> = Mutex::new(None);

/// The programs that used the most CPU since the previous call, busiest first, at most
/// `count`. This is a seperate command from `system_info` because it lists all processes. Only
/// the widgets that show programs pay for that.
#[tauri::command]
pub async fn system_top_programs(count: usize) -> Result<Vec<Program>, String> {
    blocking::run(move || {
        let mut reader = PROGRAMS.lock().unwrap_or_else(PoisonError::into_inner);
        let mut top = reader.get_or_insert_with(programs::Reader::new).read();
        top.truncate(count);
        Ok(top)
    })
    .await
}

impl Reader {
    fn new() -> Self {
        Self {
            cpu: System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
            ),
            networks: Networks::new_with_refreshed_list(),
            nvml: Nvml::init().ok(),
            traffic: Traffic::new(),
            last: None,
        }
    }

    fn read(&mut self) -> SystemInfo {
        if let Some((at, info)) = &self.last
            && at.elapsed() < SHARE
        {
            return info.clone();
        }
        // Load and traffic are differences between two measures. If there is no recent request
        // to compare with, measure now and again a short time later. Do not measure across the
        // gap.
        if self.last.as_ref().is_none_or(|(at, _)| at.elapsed() > STALE) {
            self.measure();
            thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        }
        let at = self.measure();

        let latest = self.traffic.latest();
        let info = SystemInfo {
            hostname: System::host_name().unwrap_or_default(),
            wifi: native::wifi_name(),
            cpu: Cpu { load: self.cpu.global_cpu_usage() },
            gpu: self.nvml.as_ref().and_then(read_gpu),
            net: Net {
                down: latest.down,
                up: latest.up,
                avg1m: self.traffic.average(Duration::from_secs(60)),
                avg5m: self.traffic.average(Duration::from_secs(5 * 60)),
                avg15m: self.traffic.average(traffic::LONGEST),
            },
        };
        self.last = Some((at, info.clone()));
        info
    }

    /// Refreshes the CPU and network counters, records the traffic, and returns the time of the
    /// measure.
    fn measure(&mut self) -> Instant {
        self.cpu.refresh_cpu_usage();
        // `received` and `transmitted` count from the previous refresh. Thus a new adapter, for
        // example a VPN that connects, does not add its lifetime total as a spike.
        self.networks.refresh(true);
        let (down, up) = self
            .networks
            .list()
            .iter()
            .filter(|(name, _)| native::counts(name))
            .fold((0, 0), |(down, up), (_, data)| {
                (down + data.received(), up + data.transmitted())
            });
        let at = Instant::now();
        self.traffic.push(at, down, up);
        at
    }
}

fn read_gpu(nvml: &Nvml) -> Option<Gpu> {
    let device = nvml.device_by_index(0).ok()?;
    Some(Gpu {
        load: device.utilization_rates().ok()?.gpu,
        temperature: device.temperature(TemperatureSensor::Gpu).ok()?,
    })
}
