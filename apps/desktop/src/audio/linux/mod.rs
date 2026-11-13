//! The Linux side: the PulseAudio API, which PipeWire also serves through pipewire-pulse. The
//! sound settings of GNOME and KDE, and pavucontrol, show the same devices and app streams.

mod connection;
mod meters;
mod mixer;
mod watch;

pub use meters::{Meter, levels};
pub use mixer::{mixer, outputs, set_muted, set_output, set_volume};
pub use watch::{unwatch, watch};

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::audio::Target;

    /// Reads the real sound server. Run it with `--ignored --nocapture` to see the values.
    #[test]
    #[ignore]
    fn reads_the_sound_server() {
        let found = mixer().unwrap();
        println!("{}", serde_json::to_string_pretty(&found).unwrap());
        println!("{}", serde_json::to_string_pretty(&outputs().unwrap()).unwrap());
        println!("levels(): {}", serde_json::to_string(&levels().unwrap()).unwrap());
        let mut meter = Meter::open();
        for _ in 0..5 {
            std::thread::sleep(Duration::from_millis(100));
            println!("meter: {}", serde_json::to_string(&meter.read().unwrap()).unwrap());
        }
        let output = found.output.expect("an output");
        assert!(set_volume(Target::Output, output.volume).unwrap());
        assert!(set_output(&output.id).unwrap());
        assert!(!set_output("no such output").unwrap());
        for app in &found.apps {
            assert!(set_volume(Target::App(app.id.clone()), app.volume).unwrap());
            assert!(set_muted(Target::App(app.id.clone()), app.muted).unwrap());
        }
        assert!(!set_volume(Target::App("app none".into()), 0.5).unwrap());
    }
}
