/// Returns `Ok` if an app at `app_version` can run a widget that needs `min_app_version`.
/// Else it returns the reason.
pub(super) fn compatibility(
    app_version: &str,
    min_app_version: Option<&str>,
) -> Result<(), String> {
    let Some(min) = min_app_version else {
        return Ok(());
    };
    let need = parse_version(min)
        .ok_or_else(|| format!("its minAppVersion \"{min}\" is not a version like 0.1.0"))?;
    if parse_version(app_version).unwrap_or_default() < need {
        return Err(format!("It needs EdgeStation {min} or later, and this is {app_version}"));
    }
    Ok(())
}

/// True if `candidate` is a later `major.minor.patch` than `current`. A candidate that does not
/// parse is never newer. An installed version that does not parse is older than all others.
pub(super) fn is_newer(candidate: &str, current: &str) -> bool {
    parse_version(candidate).is_some_and(|c| c > parse_version(current).unwrap_or_default())
}

/// Reads `major.minor.patch` at the start of `version`, like `apps/ui/src/lib/widgets/version.ts`.
fn parse_version(version: &str) -> Option<[u64; 3]> {
    let mut parts = version.trim().splitn(3, '.');
    let mut next = || parts.next();
    let major = next()?.parse().ok()?;
    let minor = next()?.parse().ok()?;
    let rest = next()?;
    let digits = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    let patch = rest[..digits].parse().ok()?;
    Some([major, minor, patch])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions() {
        assert!(is_newer("1.2.0", "1.1.9"));
        assert!(is_newer("0.10.0", "0.9.0"));
        assert!(!is_newer("1.0.0", "1.0.0"));
        assert!(!is_newer("1.0.0", "1.0.1"));
        assert!(is_newer("1.0.0", "not a version"));
        assert!(!is_newer("soon", "1.0.0"));
        assert!(compatibility("0.3.0", None).is_ok());
        assert!(compatibility("0.3.0", Some("0.3.0")).is_ok());
        assert!(compatibility("0.4.0-beta", Some("0.4.0")).is_ok());
        assert!(compatibility("0.3.0", Some("0.4.0")).is_err());
        assert!(compatibility("0.3.0", Some("latest")).is_err());
    }
}
