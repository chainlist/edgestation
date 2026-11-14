//! The Linux side. The connectors come from RandR.

use tauri::Monitor;

use crate::os::linux::x11::kept;

pub fn name(monitors: Vec<Monitor>) -> Vec<(String, Monitor)> {
    let connectors = kept().map(|x11| x11.monitors()).unwrap_or_default();
    let connector = |monitor: &Monitor| {
        let at = monitor.position();
        let found = connectors.iter().find(|c| c.x == at.x && c.y == at.y)?;
        Some(found.name.clone()).filter(|name| !name.is_empty())
    };
    let name = |monitor: Monitor| {
        Some((unique_name(monitor.name().map(String::as_str), connector(&monitor))?, monitor))
    };
    monitors.into_iter().filter_map(name).collect()
}

/// The model and the connector, or the one that is known.
fn unique_name(model: Option<&str>, connector: Option<String>) -> Option<String> {
    match (model.filter(|model| !model.is_empty()), connector) {
        (Some(model), Some(connector)) => Some(format!("{model} ({connector})")),
        (model, connector) => connector.or(model.map(Into::into)),
    }
}

#[cfg(test)]
mod tests {
    use super::unique_name;

    #[test]
    fn names_have_the_connector() {
        let dp = || Some("DP-2".to_owned());
        assert_eq!(
            unique_name(Some("XENEON EDGE"), dp()).as_deref(),
            Some("XENEON EDGE (DP-2)")
        );
        assert_eq!(unique_name(None, dp()).as_deref(), Some("DP-2"));
        assert_eq!(unique_name(Some(""), dp()).as_deref(), Some("DP-2"));
        assert_eq!(unique_name(Some("XENEON EDGE"), None).as_deref(), Some("XENEON EDGE"));
        assert_eq!(unique_name(None, None), None);
    }
}
