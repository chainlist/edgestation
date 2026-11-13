//! A connection to the sound server, and the reads that the mixer and the meters share. Each
//! request has its own connection, on its own thread, so nothing runs between requests.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::introspect::{
    Introspector, SinkInfo, SinkInputInfo, SourceInfo, SourceOutputInfo,
};
use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::def::PortAvailable;
use libpulse_binding::error::PAErr;
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::{Proplist, properties};
use libpulse_binding::time::MicroSeconds;
use libpulse_binding::volume::ChannelVolumes;

/// The context comes first, so it is dropped before the main loop that it uses.
pub struct Pulse {
    pub context: Context,
    mainloop: Mainloop,
}

/// Where the callback of an operation puts its result.
type Slot<T> = Rc<RefCell<Option<T>>>;

/// An output device.
pub struct Sink {
    pub index: u32,
    pub name: String,
    pub description: String,
    pub volume: ChannelVolumes,
    pub mute: bool,
    /// The source that records what the sink plays.
    pub monitor: u32,
    /// False if its port is unplugged, for example headphones.
    pub available: bool,
}

/// An input device, or the monitor of a sink.
pub struct Source {
    pub index: u32,
    pub name: String,
    pub description: String,
    pub volume: ChannelVolumes,
    pub mute: bool,
    /// The sink of a monitor. A monitor is not a microphone.
    pub monitor_of_sink: Option<u32>,
}

/// A stream that an app plays (a sink input).
#[derive(Clone, Debug, Default)]
pub struct Input {
    pub index: u32,
    pub sink: u32,
    pub volume: ChannelVolumes,
    pub mute: bool,
    /// True while the app pauses the stream.
    pub corked: bool,
    /// The file name of the executable, from `application.process.binary`.
    pub binary: Option<String>,
    /// The name that the app gives itself, from `application.name`.
    pub app: Option<String>,
    pub pid: Option<u32>,
}

/// A stream that an app records (a source output).
pub struct Recording {
    pub source: u32,
    pub client: Option<u32>,
}

impl From<&SinkInfo<'_>> for Sink {
    fn from(info: &SinkInfo) -> Self {
        Self {
            index: info.index,
            name: text(&info.name),
            description: text(&info.description),
            volume: info.volume,
            mute: info.mute,
            monitor: info.monitor_source,
            available: info
                .active_port
                .as_ref()
                .is_none_or(|port| port.available != PortAvailable::No),
        }
    }
}

impl From<&SourceInfo<'_>> for Source {
    fn from(info: &SourceInfo) -> Self {
        Self {
            index: info.index,
            name: text(&info.name),
            description: text(&info.description),
            volume: info.volume,
            mute: info.mute,
            monitor_of_sink: info.monitor_of_sink,
        }
    }
}

impl From<&SinkInputInfo<'_>> for Input {
    fn from(info: &SinkInputInfo) -> Self {
        let property = |key| info.proplist.get_str(key).filter(|value| !value.is_empty());
        Self {
            index: info.index,
            sink: info.sink,
            volume: info.volume,
            mute: info.mute,
            corked: info.corked,
            binary: property(properties::APPLICATION_PROCESS_BINARY),
            app: property(properties::APPLICATION_NAME),
            pid: property(properties::APPLICATION_PROCESS_ID).and_then(|pid| pid.parse().ok()),
        }
    }
}

impl From<&SourceOutputInfo<'_>> for Recording {
    fn from(info: &SourceOutputInfo) -> Self {
        Self { source: info.source, client: info.client }
    }
}

/// The text of a PulseAudio error.
pub fn message(error: PAErr) -> String {
    error.to_string().unwrap_or_else(|| format!("PulseAudio error {}", error.0))
}

fn text(value: &Option<std::borrow::Cow<str>>) -> String {
    value.as_deref().unwrap_or_default().to_owned()
}

/// Runs list operation `method` and returns all its items as `$item`.
macro_rules! list {
    ($pulse:ident, $method:ident, $item:ty, $what:literal) => {
        $pulse.wait(|context, slot| {
            let mut found: Vec<$item> = Vec::new();
            context.introspect().$method(move |result| match result {
                ListResult::Item(item) => found.push(item.into()),
                ListResult::End => drop(slot.replace(Some(Ok(std::mem::take(&mut found))))),
                ListResult::Error => {
                    drop(slot.replace(Some(Err(concat!("cannot list the ", $what).to_owned()))))
                }
            });
        })?
    };
}

impl Pulse {
    /// Connects to the sound server: PulseAudio, or PipeWire through pipewire-pulse.
    pub fn connect() -> Result<Self, String> {
        let mainloop = Mainloop::new().ok_or("cannot start a PulseAudio main loop")?;
        let mut names = Proplist::new().ok_or("cannot make a PulseAudio property list")?;
        let _ = names.set_str(properties::APPLICATION_NAME, "EdgeStation");
        let _ = names.set_str(properties::APPLICATION_ID, "com.edgestation.dashboard");
        let mut context = Context::new_with_proplist(&mainloop, "EdgeStation", &names)
            .ok_or("cannot make a PulseAudio context")?;
        context.connect(None, FlagSet::NOAUTOSPAWN, None).map_err(message)?;
        let mut pulse = Self { context, mainloop };
        pulse.run_until(|pulse| pulse.context.get_state() == State::Ready)?;
        Ok(pulse)
    }

    /// Runs the main loop until `done` is true.
    fn run_until(&mut self, mut done: impl FnMut(&Self) -> bool) -> Result<(), String> {
        while !done(self) {
            self.check()?;
            if !matches!(self.mainloop.iterate(true), IterateResult::Success(_)) {
                return Err("the PulseAudio main loop stopped".into());
            }
        }
        Ok(())
    }

    /// Runs the main loop for at most `timeout`, or until something happens.
    pub fn run_for(&mut self, timeout: Duration) -> Result<(), String> {
        let timeout = MicroSeconds(timeout.as_micros().try_into().unwrap_or(u64::MAX));
        self.mainloop.prepare(Some(timeout)).map_err(message)?;
        self.mainloop.poll().map_err(message)?;
        self.mainloop.dispatch().map_err(message)?;
        self.check()
    }

    /// Takes in all that the server sent, without waiting.
    pub fn drain(&mut self) -> Result<(), String> {
        loop {
            match self.mainloop.iterate(false) {
                IterateResult::Success(0) => return self.check(),
                IterateResult::Success(_) => {}
                _ => return Err("the PulseAudio main loop stopped".into()),
            }
        }
    }

    /// Fails if the connection is lost.
    fn check(&self) -> Result<(), String> {
        match self.context.get_state() {
            State::Failed | State::Terminated => Err("no connection to the sound server".into()),
            _ => Ok(()),
        }
    }

    /// Starts an operation with `start`, which gets the slot for its result, and runs the main
    /// loop until its callback fills the slot.
    pub fn wait<T: 'static>(
        &mut self,
        start: impl FnOnce(&mut Context, Slot<T>),
    ) -> Result<T, String> {
        let slot: Slot<T> = Rc::new(RefCell::new(None));
        start(&mut self.context, Rc::clone(&slot));
        self.run_until(|_| slot.borrow().is_some())?;
        Ok(slot.borrow_mut().take().expect("the callback filled the slot"))
    }

    /// Runs a change that `start` begins, and returns true if the server made it.
    pub fn change(
        &mut self,
        start: impl FnOnce(&mut Introspector, Box<dyn FnMut(bool)>),
    ) -> Result<bool, String> {
        self.wait(|context, slot| {
            start(
                &mut context.introspect(),
                Box::new(move |done| {
                    slot.replace(Some(done));
                }),
            );
        })
    }

    /// The names of the default output and the default input.
    pub fn defaults(&mut self) -> Result<(Option<String>, Option<String>), String> {
        self.wait(|context, slot| {
            context.introspect().get_server_info(move |info| {
                let name =
                    |name: &Option<std::borrow::Cow<str>>| name.as_deref().map(str::to_owned);
                slot.replace(Some((
                    name(&info.default_sink_name),
                    name(&info.default_source_name),
                )));
            });
        })
    }

    pub fn sinks(&mut self) -> Result<Vec<Sink>, String> {
        list!(self, get_sink_info_list, Sink, "outputs")
    }

    pub fn sources(&mut self) -> Result<Vec<Source>, String> {
        list!(self, get_source_info_list, Source, "inputs")
    }

    pub fn inputs(&mut self) -> Result<Vec<Input>, String> {
        list!(self, get_sink_input_info_list, Input, "app streams")
    }

    pub fn recordings(&mut self) -> Result<Vec<Recording>, String> {
        list!(self, get_source_output_info_list, Recording, "recordings")
    }
}

impl Drop for Pulse {
    fn drop(&mut self) {
        self.context.disconnect();
    }
}
