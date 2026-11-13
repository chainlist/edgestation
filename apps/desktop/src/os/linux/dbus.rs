//! D-Bus calls for the Linux side of `system` and `media`. The app keeps one connection to each
//! bus. A connection that fails is dropped, so the next call connects again, for example after
//! the bus restarted.

use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::{Mutex, PoisonError};
use std::time::Duration;

use zbus::blocking::Connection;
use zbus::blocking::connection::Builder;
use zbus::zvariant::{DynamicType, OwnedValue};

const PROPERTIES: &str = "org.freedesktop.DBus.Properties";
/// A call fails after this time without an answer. A service that hangs, for example a media
/// player, must not hold a widget for the 25 seconds of the D-Bus default.
const TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone, Copy)]
pub enum Bus {
    System,
    Session,
}

static SYSTEM: Mutex<Option<Connection>> = Mutex::new(None);
static SESSION: Mutex<Option<Connection>> = Mutex::new(None);

/// Runs `f` with the connection to `bus`.
pub fn with<T>(bus: Bus, f: impl FnOnce(&Connection) -> zbus::Result<T>) -> zbus::Result<T> {
    let kept = match bus {
        Bus::System => &SYSTEM,
        Bus::Session => &SESSION,
    };
    let connection = {
        let mut kept = kept.lock().unwrap_or_else(PoisonError::into_inner);
        match &*kept {
            Some(connection) => connection.clone(),
            None => {
                let builder = match bus {
                    Bus::System => Builder::system()?,
                    Bus::Session => Builder::session()?,
                };
                kept.insert(builder.method_timeout(TIMEOUT).build()?).clone()
            }
        }
    };
    let result = f(&connection);
    if let Err(zbus::Error::InputOutput(error)) = &result
        && error.kind() != ErrorKind::TimedOut
    {
        kept.lock().unwrap_or_else(PoisonError::into_inner).take();
    }
    result
}

/// Calls `method` of `interface` on object `path` of `service`, and returns the reply.
pub fn call<B, R>(
    bus: &Connection,
    service: &str,
    path: &str,
    interface: &str,
    method: &str,
    body: &B,
) -> zbus::Result<R>
where
    B: serde::Serialize + DynamicType,
    R: for<'de> serde::Deserialize<'de> + zbus::zvariant::Type,
{
    let reply = bus.call_method(Some(service), path, Some(interface), method, body)?;
    reply.body().deserialize()
}

/// The value of property `name` of `interface`, on object `path` of `service`.
pub fn property(
    bus: &Connection,
    service: &str,
    path: &str,
    interface: &str,
    name: &str,
) -> zbus::Result<OwnedValue> {
    call(bus, service, path, PROPERTIES, "Get", &(interface, name))
}

/// All properties of `interface`, on object `path` of `service`.
pub fn properties(
    bus: &Connection,
    service: &str,
    path: &str,
    interface: &str,
) -> zbus::Result<HashMap<String, OwnedValue>> {
    call(bus, service, path, PROPERTIES, "GetAll", &(interface,))
}
