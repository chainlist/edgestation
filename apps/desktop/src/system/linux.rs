//! The Linux side.

/// The name of the active Wi-Fi connection in NetworkManager. This is the name of its profile,
/// which is the SSID unless the user renamed the profile, as on Windows. `None` without
/// NetworkManager.
pub fn wifi_name() -> Option<String> {
    use zbus::zvariant::OwnedObjectPath;

    use crate::os::linux::dbus::{self, Bus};

    const SERVICE: &str = "org.freedesktop.NetworkManager";
    const ACTIVE: &str = "org.freedesktop.NetworkManager.Connection.Active";
    /// `NM_ACTIVE_CONNECTION_STATE_ACTIVATED`.
    const ACTIVATED: u32 = 2;

    let lookup = |bus: &zbus::blocking::Connection| -> zbus::Result<Option<String>> {
        let manager = "/org/freedesktop/NetworkManager";
        let paths = dbus::property(bus, SERVICE, manager, SERVICE, "ActiveConnections")?;
        for path in Vec::<OwnedObjectPath>::try_from(paths)? {
            let mut connection = dbus::properties(bus, SERVICE, path.as_str(), ACTIVE)?;
            let mut take = |name| connection.remove(name);
            let wifi = take("Type").and_then(|v| String::try_from(v).ok());
            let id = take("Id").and_then(|v| String::try_from(v).ok());
            let state = take("State").and_then(|v| u32::try_from(v).ok());
            if wifi.as_deref() == Some("802-11-wireless") && state == Some(ACTIVATED) {
                return Ok(id);
            }
        }
        Ok(None)
    };
    dbus::with(Bus::System, lookup).ok().flatten()
}

/// True if the traffic of network interface `name` counts. Linux also lists the loopback and
/// virtual interfaces, for example of Docker, of virtual machines and their bridges. Their
/// traffic also goes through a real interface, or never leaves the machine. Only interfaces of a
/// device count.
pub fn counts(name: &str) -> bool {
    std::path::Path::new("/sys/class/net").join(name).join("device").exists()
}
