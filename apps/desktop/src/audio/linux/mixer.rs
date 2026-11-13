use libpulse_binding::volume::{ChannelVolumes, Volume};

use super::connection::{Input, Pulse, Sink, Source};
use crate::audio::{App, Device, Mixer, Output, Target};

pub fn mixer() -> Result<Mixer, String> {
    let mut pulse = Pulse::connect()?;
    let (output, input) = pulse.defaults()?;
    let sinks = pulse.sinks()?;
    let output = sinks.iter().find(|sink| Some(&sink.name) == output.as_ref());
    let apps = match output {
        Some(sink) => {
            apps(&pulse.inputs()?, sink.index, |pid| crate::process::image_path(pid as usize))
        }
        None => Vec::new(),
    };
    let sources = pulse.sources()?;
    let input = microphone(&sources, input.as_deref());
    Ok(Mixer {
        output: output.map(|sink| device(&sink.name, &sink.description, &sink.volume, sink.mute)),
        input: input
            .map(|source| device(&source.name, &source.description, &source.volume, source.mute)),
        apps,
    })
}

pub fn set_volume(target: Target, level: f32) -> Result<bool, String> {
    let mut pulse = Pulse::connect()?;
    match target {
        Target::Output => {
            let Some(sink) = default_sink(&mut pulse)? else {
                return Ok(false);
            };
            let volume = with_level(sink.volume, level);
            pulse.change(|server, done| {
                server.set_sink_volume_by_name(&sink.name, &volume, Some(done));
            })
        }
        Target::Input => {
            let Some(source) = default_source(&mut pulse)? else {
                return Ok(false);
            };
            let volume = with_level(source.volume, level);
            pulse.change(|server, done| {
                server.set_source_volume_by_name(&source.name, &volume, Some(done));
            })
        }
        Target::App(id) => {
            let streams = app_streams(&mut pulse, &id)?;
            for stream in &streams {
                let volume = with_level(stream.volume, level);
                pulse.change(|server, done| {
                    server.set_sink_input_volume(stream.index, &volume, Some(done));
                })?;
            }
            Ok(!streams.is_empty())
        }
    }
}

pub fn set_muted(target: Target, muted: bool) -> Result<bool, String> {
    let mut pulse = Pulse::connect()?;
    match target {
        Target::Output => {
            let Some(sink) = default_sink(&mut pulse)? else {
                return Ok(false);
            };
            pulse.change(|server, done| {
                server.set_sink_mute_by_name(&sink.name, muted, Some(done));
            })
        }
        Target::Input => {
            let Some(source) = default_source(&mut pulse)? else {
                return Ok(false);
            };
            pulse.change(|server, done| {
                server.set_source_mute_by_name(&source.name, muted, Some(done));
            })
        }
        Target::App(id) => {
            let streams = app_streams(&mut pulse, &id)?;
            for stream in &streams {
                pulse.change(|server, done| {
                    server.set_sink_input_mute(stream.index, muted, Some(done));
                })?;
            }
            Ok(!streams.is_empty())
        }
    }
}

/// The output devices that are plugged in, in name order.
pub fn outputs() -> Result<Vec<Output>, String> {
    let mut pulse = Pulse::connect()?;
    let mut outputs: Vec<Output> = pulse
        .sinks()?
        .into_iter()
        .filter(|sink| sink.available)
        .map(|sink| Output { id: sink.name, name: sink.description })
        .collect();
    outputs.sort_by_cached_key(|output| output.name.to_lowercase());
    Ok(outputs)
}

/// Makes output `id` the default output. Apps that follow the default move to it. Apps that the
/// user moved to a device stay there.
pub fn set_output(id: &str) -> Result<bool, String> {
    let mut pulse = Pulse::connect()?;
    if !pulse.sinks()?.iter().any(|sink| sink.name == id && sink.available) {
        return Ok(false);
    }
    pulse.wait(|context, slot| {
        context.set_default_sink(id, move |done| {
            slot.replace(Some(done));
        });
    })
}

fn default_sink(pulse: &mut Pulse) -> Result<Option<Sink>, String> {
    let (name, _) = pulse.defaults()?;
    Ok(pulse.sinks()?.into_iter().find(|sink| Some(&sink.name) == name.as_ref()))
}

fn default_source(pulse: &mut Pulse) -> Result<Option<Source>, String> {
    let (_, name) = pulse.defaults()?;
    let sources = pulse.sources()?;
    let index = microphone(&sources, name.as_deref()).map(|source| source.index);
    Ok(sources.into_iter().find(|source| Some(source.index) == index))
}

/// The streams of app `id` on the default output.
fn app_streams(pulse: &mut Pulse, id: &str) -> Result<Vec<Input>, String> {
    let Some(sink) = default_sink(pulse)? else {
        return Ok(Vec::new());
    };
    let inputs = pulse.inputs()?.into_iter();
    Ok(inputs.filter(|input| input.sink == sink.index && app_id(input) == id).collect())
}

/// The default input `default`, if it is a microphone. When there is no microphone, the default
/// input is the monitor of an output.
pub(super) fn microphone<'a>(sources: &'a [Source], default: Option<&str>) -> Option<&'a Source> {
    let source = sources.iter().find(|source| Some(source.name.as_str()) == default)?;
    source.monitor_of_sink.is_none().then_some(source)
}

/// The id of the app of `input`: its executable, which groups all its streams as one app. A
/// stream without one is an app of its own.
pub(super) fn app_id(input: &Input) -> String {
    match &input.binary {
        Some(binary) => format!("app {binary}"),
        None => format!("stream {}", input.index),
    }
}

/// The apps that play on sink `sink`, one for each executable, in name order. `path` gives the
/// executable of a process id.
fn apps(inputs: &[Input], sink: u32, path: impl Fn(u32) -> Option<String>) -> Vec<App> {
    let mut apps: Vec<App> = Vec::new();
    for input in inputs.iter().filter(|input| input.sink == sink) {
        let id = app_id(input);
        if let Some(app) = apps.iter_mut().find(|app| app.id == id) {
            app.muted &= input.mute;
            app.active |= !input.corked;
            continue;
        }
        let path = input.pid.and_then(&path);
        let name = input.app.clone().or_else(|| {
            let program = path.as_deref().and_then(crate::process::program_name);
            program.or_else(|| input.binary.clone())
        });
        apps.push(App {
            id,
            name: name.unwrap_or_default(),
            path,
            program: input.binary.clone(),
            volume: level(&input.volume),
            muted: input.mute,
            active: !input.corked,
        });
    }
    apps.sort_by_cached_key(|app| app.name.to_lowercase());
    apps
}

fn device(id: &str, name: &str, volume: &ChannelVolumes, muted: bool) -> Device {
    Device { id: id.to_owned(), name: name.to_owned(), volume: level(volume), muted }
}

/// The volume from 0 to 1, as the sound settings show it. 1 is 100 %. The settings can go
/// higher, but a widget shows up to 100 %.
pub(super) fn level(volume: &ChannelVolumes) -> f32 {
    (volume.max().0 as f32 / Volume::NORMAL.0 as f32).min(1.0)
}

/// `volume` with its loudest channel at `level`, from 0 to 1. The balance stays the same.
fn with_level(mut volume: ChannelVolumes, level: f32) -> ChannelVolumes {
    let level = Volume((level.clamp(0.0, 1.0) * Volume::NORMAL.0 as f32).round() as u32);
    volume.scale(level);
    volume
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stereo(left: u32, right: u32) -> ChannelVolumes {
        let mut volume = ChannelVolumes::default();
        volume.set(2, Volume(left));
        volume.get_mut()[1] = Volume(right);
        volume
    }

    fn input(index: u32, binary: Option<&str>, app: Option<&str>) -> Input {
        Input {
            index,
            sink: 1,
            volume: stereo(Volume::NORMAL.0 / 2, Volume::NORMAL.0 / 2),
            binary: binary.map(Into::into),
            app: app.map(Into::into),
            pid: Some(100 + index),
            ..Input::default()
        }
    }

    #[test]
    fn volumes_go_from_0_to_1() {
        let normal = Volume::NORMAL.0;
        assert_eq!(level(&stereo(normal / 2, normal / 4)), 0.5);
        assert_eq!(level(&stereo(normal * 3 / 2, normal)), 1.0);
        let set = with_level(stereo(normal / 2, normal / 4), 0.8);
        assert!((level(&set) - 0.8).abs() < 1e-4);
        // The balance stays.
        assert!(set.get()[1].0.abs_diff(set.get()[0].0 / 2) <= 1);
        // A silent stream gets the level on all channels.
        assert_eq!(with_level(stereo(0, 0), 0.5).get()[1].0, normal / 2);
    }

    #[test]
    fn groups_the_streams_of_an_executable_as_one_app() {
        let mut second = input(2, Some("firefox"), Some("Firefox"));
        second.mute = true;
        let mut third = input(3, Some("firefox"), Some("Firefox"));
        third.corked = true;
        let mut elsewhere = input(5, Some("mpv"), Some("mpv"));
        elsewhere.sink = 2;
        let inputs = [
            input(1, Some("spotify"), Some("Spotify")),
            second,
            third,
            input(4, None, None),
            elsewhere,
        ];
        let apps = apps(&inputs, 1, |pid| Some(format!("/proc/{pid}")));
        let ids: Vec<_> = apps.iter().map(|app| app.id.as_str()).collect();
        // In name order. A stream without a name or an executable comes first, with no name.
        assert_eq!(ids, ["stream 4", "app firefox", "app spotify"]);
        let firefox = &apps[1];
        assert_eq!(firefox.name, "Firefox");
        assert_eq!(firefox.program.as_deref(), Some("firefox"));
        assert_eq!(firefox.path.as_deref(), Some("/proc/102"));
        assert_eq!(firefox.volume, 0.5);
        // Muted only if all its streams are, active if one plays.
        assert!(!firefox.muted && firefox.active);
    }

    #[test]
    fn a_monitor_is_not_a_microphone() {
        let source = |index, name: &str, monitor_of_sink| Source {
            index,
            name: name.into(),
            description: String::new(),
            volume: ChannelVolumes::default(),
            mute: false,
            monitor_of_sink,
        };
        let sources = [source(1, "speakers.monitor", Some(0)), source(2, "mic", None)];
        assert_eq!(microphone(&sources, Some("mic")).map(|s| s.index), Some(2));
        assert!(microphone(&sources, Some("speakers.monitor")).is_none());
        assert!(microphone(&sources, None).is_none());
    }
}
