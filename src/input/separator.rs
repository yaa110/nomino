#[cfg(not(target_os = "windows"))]
pub const MAIN_SEPARATOR: &str = "/";

#[cfg(target_os = "windows")]
pub const MAIN_SEPARATOR: &str = "\\\\";
