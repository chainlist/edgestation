//! The Windows side.

/// The name of the connected Wi-Fi network. This is the name of its Windows network profile,
/// which is the SSID unless the user renamed the profile. On Windows 11 24H2 and later, the SSID
/// needs Location access, but the profile name does not.
pub fn wifi_name() -> Option<String> {
    use crate::os::windows::com::{self, Apartment};

    // The lookup uses COM, which is set up on this thread for this call only.
    com::with(Apartment::Multithreaded, lookup)
}

fn lookup() -> Option<String> {
    use windows::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetIfTable2, IF_TYPE_IEEE80211, MIB_IF_TABLE2,
    };
    use windows::Win32::Networking::NetworkListManager::{INetworkListManager, NetworkListManager};
    use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};

    unsafe {
        // Find the Wi-Fi adapters, to tell thier connections apart from Ethernet and VPN ones.
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        GetIfTable2(&mut table).ok().ok()?;
        let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), (*table).NumEntries as usize);
        let wifi: Vec<_> = rows
            .iter()
            .filter(|row| row.Type == IF_TYPE_IEEE80211)
            .map(|row| row.InterfaceGuid)
            .collect();
        FreeMibTable(table as _);

        let manager: INetworkListManager =
            CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL).ok()?;
        let connections = manager.GetNetworkConnections().ok()?;
        loop {
            let mut next = [None];
            connections.Next(&mut next, None).ok()?;
            let connection = next[0].take()?;
            let adapter = connection.GetAdapterId().ok()?;
            if connection.IsConnected().ok()?.as_bool() && wifi.contains(&adapter) {
                return Some(connection.GetNetwork().ok()?.GetName().ok()?.to_string());
            }
        }
    }
}

/// Every interface that sysinfo lists counts.
pub fn counts(_name: &str) -> bool {
    true
}
