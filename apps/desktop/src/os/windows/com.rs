//! COM, which the shell, Core Audio and the network list of Windows need on the thread that calls
//! them.

use std::sync::Once;

use windows::Win32::System::Com::{
    APTTYPE, APTTYPE_MTA, APTTYPEQUALIFIER, COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED,
    CoGetApartmentType, CoIncrementMTAUsage, CoInitializeEx, CoUninitialize,
};

/// The apartment that a thread joins.
#[derive(Clone, Copy)]
pub enum Apartment {
    /// Core Audio and the network list work here. Their objects work from any thread of the
    /// apartment.
    Multithreaded,
    /// The shell needs this one.
    SingleThreaded,
}

/// COM set up on this thread while the value lives. A thread that is already in the other
/// apartment stays there, and then the value does nothing. Release every COM object before this
/// value.
pub struct Com {
    set_up: bool,
}

impl Com {
    pub fn set_up(apartment: Apartment) -> Self {
        let model = match apartment {
            Apartment::Multithreaded => COINIT_MULTITHREADED,
            Apartment::SingleThreaded => COINIT_APARTMENTTHREADED,
        };
        // SAFETY: the value calls `CoUninitialize` once, on this thread, when the call worked.
        Self { set_up: unsafe { CoInitializeEx(None, model) }.is_ok() }
    }
}

impl Drop for Com {
    fn drop(&mut self) {
        if self.set_up {
            // SAFETY: `set_up` made the matching `CoInitializeEx` call on this thread.
            unsafe { CoUninitialize() };
        }
    }
}

/// Runs `f` with COM set up on this thread for this call only. Release every COM object inside
/// `f`.
pub fn with<T>(apartment: Apartment, f: impl FnOnce() -> T) -> T {
    let _com = Com::set_up(apartment);
    f()
}

/// Keeps the multithreaded apartment of the process alive between requests, for objects that the
/// app keeps, such as the meters of the audio mixer. Without this, the apartment ends when the
/// last thread leaves it.
pub fn keep_mta() {
    static KEEP: Once = Once::new();
    // The code never gives back the cookie, so the apartment lasts untill the app exits.
    KEEP.call_once(|| unsafe {
        let _ = CoIncrementMTAUsage();
    });
}

/// True if this thread is in the multithreaded apartment.
pub fn in_mta() -> bool {
    let (mut kind, mut qualifier) = (APTTYPE::default(), APTTYPEQUALIFIER::default());
    // SAFETY: both pointers are to locals that live for the call.
    unsafe { CoGetApartmentType(&mut kind, &mut qualifier).is_ok() && kind == APTTYPE_MTA }
}
