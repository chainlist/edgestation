//! The macOS side, a stub. It finds no executable and no name.

pub fn image_path(_pid: usize) -> Option<String> {
    None
}

pub fn program_name(_path: &str) -> Option<String> {
    None
}
