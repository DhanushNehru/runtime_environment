use std::env;

pub fn is_mac_os() -> bool {
    env::consts::OS == "macos"
}

pub fn is_windows() -> bool {
    env::consts::OS == "windows"
}

pub fn is_linux() -> bool {
    env::consts::OS == "linux"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mac_os() {
        if cfg!(target_os = "macos") {
            assert!(is_mac_os());
        } else {
            assert!(!is_mac_os());
        }
    }

    #[test]
    fn test_is_windows() {
        if cfg!(target_os = "windows") {
            assert!(is_windows());
        } else {
            assert!(!is_windows());
        }
    }

    #[test]
    fn test_is_linux() {
        if cfg!(target_os = "linux") {
            assert!(is_linux());
        } else {
            assert!(!is_linux());
        }
    }
}