//! Update-install guidance.
//!
//! Herdr does not self-update. This module only reports how the current
//! installation is managed (Homebrew/mise/Nix/direct) so the CLI and remote
//! attach can print the right install command. No outbound network calls.

use std::env;
use std::path::{Path, PathBuf};

const HERDR_UPDATE_COMMAND: &str = "herdr update";
const HOMEBREW_UPDATE_COMMAND: &str = "brew update && brew upgrade herdr";
const MISE_UPDATE_COMMAND: &str = "mise upgrade herdr";
const NIX_UPDATE_COMMAND: &str = "update through Nix";
const MISE_INSTALLS_DIR_ENV: &str = "MISE_INSTALLS_DIR";

// ---------------------------------------------------------------------------
// Version
// ---------------------------------------------------------------------------

/// Parsed semver version for comparison.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.strip_prefix('v').unwrap_or(s);
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Self {
            major: parts[0].parse().ok()?,
            minor: parts[1].parse().ok()?,
            patch: parts[2].parse().ok()?,
        })
    }

    pub fn current() -> Self {
        Self::parse(crate::build_info::BASE_VERSION).expect("invalid CARGO_PKG_VERSION")
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub(crate) const WINDOWS_INSTALLER: &str = include_str!("../distribution/install.ps1");

pub(crate) fn update_install_command() -> &'static str {
    if is_homebrew_managed_install() {
        HOMEBREW_UPDATE_COMMAND
    } else if is_mise_managed_install() {
        MISE_UPDATE_COMMAND
    } else if is_nix_managed_install() {
        NIX_UPDATE_COMMAND
    } else {
        HERDR_UPDATE_COMMAND
    }
}

pub(crate) fn update_install_instruction(install_command: &str) -> String {
    match install_command {
        HERDR_UPDATE_COMMAND => {
            "detach, run `herdr update`, then run Herdr again to reconnect".to_string()
        }
        HOMEBREW_UPDATE_COMMAND => {
            "detach, run `brew update && brew upgrade herdr`, then run Herdr again to reconnect"
                .to_string()
        }
        MISE_UPDATE_COMMAND => {
            "detach, run `mise upgrade herdr`, then run Herdr again to reconnect".to_string()
        }
        NIX_UPDATE_COMMAND => {
            "detach, update through Nix, then run Herdr again to reconnect".to_string()
        }
        command => format!("detach, run `{command}`, then run Herdr again to reconnect"),
    }
}

fn is_homebrew_managed_install() -> bool {
    let Ok(current_exe) = env::current_exe() else {
        return false;
    };

    is_homebrew_managed_exe_path_following_links(&current_exe)
}

fn is_nix_managed_install() -> bool {
    let Ok(current_exe) = env::current_exe() else {
        return false;
    };

    is_nix_store_exe_path_following_links(&current_exe)
}

fn is_mise_managed_install() -> bool {
    let Ok(current_exe) = env::current_exe() else {
        return false;
    };

    is_mise_managed_exe_path_following_links(&current_exe)
}

pub(crate) fn preview_channel_rejection_for_current_install() -> Option<&'static str> {
    let Ok(current_exe) = env::current_exe() else {
        return None;
    };

    preview_channel_rejection_for_exe_path(&current_exe)
}

pub(crate) fn package_manager_channel_update_guidance_for_current_install() -> Option<&'static str>
{
    if is_homebrew_managed_install() {
        Some("Use `brew update && brew upgrade herdr` to update Homebrew installs.")
    } else if is_mise_managed_install() {
        Some("Use `mise upgrade herdr` to update mise installs.")
    } else if is_nix_managed_install() {
        Some("Update through Nix to update Nix-managed Herdr installs.")
    } else {
        None
    }
}

fn preview_channel_rejection_for_exe_path(path: &Path) -> Option<&'static str> {
    if is_homebrew_managed_exe_path_following_links(path) {
        Some(
            "preview channel is only available for direct Herdr installs; Homebrew installs update through `brew update && brew upgrade herdr`",
        )
    } else if is_mise_managed_exe_path_following_links(path) {
        Some(
            "preview channel is only available for direct Herdr installs; mise installs update through `mise upgrade herdr`",
        )
    } else if is_nix_store_exe_path_following_links(path) {
        Some("preview channel is only available for direct Herdr installs; Nix installs update through Nix")
    } else {
        None
    }
}

#[cfg(unix)]
pub(crate) fn is_package_manager_managed_exe_path(path: &Path) -> bool {
    is_homebrew_managed_exe_path_following_links(path)
        || is_mise_managed_exe_path_following_links(path)
        || is_nix_store_exe_path_following_links(path)
}

#[cfg(not(unix))]
pub(crate) fn is_package_manager_managed_exe_path(_path: &Path) -> bool {
    false
}

fn is_homebrew_managed_exe_path_following_links(path: &Path) -> bool {
    if is_homebrew_managed_exe_path(path) {
        return true;
    }

    path.canonicalize()
        .is_ok_and(|path| is_homebrew_managed_exe_path(&path))
}

fn is_nix_store_exe_path_following_links(path: &Path) -> bool {
    if is_nix_store_exe_path(path) {
        return true;
    }

    path.canonicalize()
        .is_ok_and(|path| is_nix_store_exe_path(&path))
}

fn is_mise_managed_exe_path_following_links(path: &Path) -> bool {
    if is_mise_managed_exe_path(path) {
        return true;
    }

    path.canonicalize()
        .is_ok_and(|path| is_mise_managed_exe_path(&path))
}

fn is_nix_store_exe_path(path: &Path) -> bool {
    path.starts_with("/nix/store")
}

fn is_mise_managed_exe_path(path: &Path) -> bool {
    mise_install_root(path).is_some()
}

fn mise_install_root(path: &Path) -> Option<PathBuf> {
    if let Some(root) = mise_install_root_under_configured_installs_dir(path) {
        return Some(root);
    }

    mise_install_root_under_named_installs_dir(path)
}

fn mise_install_root_under_configured_installs_dir(path: &Path) -> Option<PathBuf> {
    let installs_dir = env::var_os(MISE_INSTALLS_DIR_ENV)
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())?;
    let version_dir = mise_tool_version_dir(path)?;
    let tool_dir = version_dir.parent()?;
    paths_match(tool_dir.parent()?, &installs_dir).then_some(version_dir.to_path_buf())
}

fn mise_install_root_under_named_installs_dir(path: &Path) -> Option<PathBuf> {
    let version_dir = mise_tool_version_dir(path)?;
    let tool_dir = version_dir.parent()?;
    let installs_dir = tool_dir.parent()?;
    if installs_dir.file_name()? != "installs" {
        return None;
    }
    Some(version_dir.to_path_buf())
}

fn mise_tool_version_dir(path: &Path) -> Option<&Path> {
    if path.file_name()? != "herdr" {
        return None;
    }
    let bin_dir = path.parent()?;
    if bin_dir.file_name()? != "bin" {
        return None;
    }
    let version_dir = bin_dir.parent()?;
    let tool_dir = version_dir.parent()?;
    if tool_dir.file_name()? != "herdr" {
        return None;
    }
    Some(version_dir)
}

fn paths_match(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }

    let Ok(left) = left.canonicalize() else {
        return false;
    };
    let Ok(right) = right.canonicalize() else {
        return false;
    };
    left == right
}

fn is_homebrew_managed_exe_path(path: &Path) -> bool {
    homebrew_cellar_keg_root(path).is_some()
}

fn homebrew_cellar_keg_root(path: &Path) -> Option<PathBuf> {
    if path.file_name()? != "herdr" {
        return None;
    }
    let bin_dir = path.parent()?;
    if bin_dir.file_name()? != "bin" {
        return None;
    }
    let version_dir = bin_dir.parent()?;
    let formula_dir = version_dir.parent()?;
    if formula_dir.file_name()? != "herdr" {
        return None;
    }
    let cellar_dir = formula_dir.parent()?;
    if cellar_dir.file_name()? != "Cellar" {
        return None;
    }
    Some(version_dir.to_path_buf())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn parse_version_basic() {
        assert_eq!(
            Version::parse("1.2.3"),
            Some(Version {
                major: 1,
                minor: 2,
                patch: 3
            })
        );
    }

    #[test]
    fn parse_version_with_v_prefix() {
        assert_eq!(
            Version::parse("v0.1.0"),
            Some(Version {
                major: 0,
                minor: 1,
                patch: 0
            })
        );
    }

    #[test]
    fn parse_version_invalid() {
        assert_eq!(Version::parse("1.2"), None);
        assert_eq!(Version::parse("abc"), None);
        assert_eq!(Version::parse(""), None);
    }

    #[test]
    fn homebrew_cellar_path_is_detected() {
        let path = Path::new("/opt/homebrew/Cellar/herdr/0.5.9/bin/herdr");

        assert!(is_homebrew_managed_exe_path(path));
        assert_eq!(
            homebrew_cellar_keg_root(path).unwrap(),
            PathBuf::from("/opt/homebrew/Cellar/herdr/0.5.9")
        );
    }

    #[test]
    fn homebrew_linux_cellar_path_is_detected() {
        let path = Path::new("/home/linuxbrew/.linuxbrew/Cellar/herdr/0.5.9/bin/herdr");

        assert!(is_homebrew_managed_exe_path(path));
    }

    #[test]
    fn homebrew_opt_path_requires_canonicalized_cellar_target() {
        let path = Path::new("/opt/homebrew/opt/herdr/bin/herdr");

        assert!(!is_homebrew_managed_exe_path(path));
    }

    #[test]
    fn non_homebrew_path_is_not_detected() {
        let path = Path::new("/usr/local/bin/herdr");

        assert!(!is_homebrew_managed_exe_path(path));
    }

    #[test]
    fn mise_install_path_is_detected() {
        let path = Path::new("/home/user/.local/share/mise/installs/herdr/0.6.6/bin/herdr");

        assert!(is_mise_managed_exe_path(path));
        assert_eq!(
            mise_install_root(path).unwrap(),
            PathBuf::from("/home/user/.local/share/mise/installs/herdr/0.6.6")
        );
    }

    #[test]
    fn mise_alias_install_path_is_detected() {
        let path = Path::new("/home/user/.local/share/mise/installs/herdr/latest/bin/herdr");

        assert!(is_mise_managed_exe_path(path));
    }

    #[test]
    fn mise_custom_installs_dir_path_is_detected() {
        let path = Path::new("/opt/mise-tools/installs/herdr/0.6.6/bin/herdr");

        assert!(is_mise_managed_exe_path(path));
    }

    #[test]
    fn mise_configured_installs_dir_path_is_detected() {
        let _guard = env_lock().lock().unwrap();
        let previous = std::env::var_os(MISE_INSTALLS_DIR_ENV);
        std::env::set_var(MISE_INSTALLS_DIR_ENV, "/opt/mise-tools");
        let path = Path::new("/opt/mise-tools/herdr/0.6.6/bin/herdr");

        assert!(is_mise_managed_exe_path(path));
        assert_eq!(
            mise_install_root(path).unwrap(),
            PathBuf::from("/opt/mise-tools/herdr/0.6.6")
        );

        if let Some(previous) = previous {
            std::env::set_var(MISE_INSTALLS_DIR_ENV, previous);
        } else {
            std::env::remove_var(MISE_INSTALLS_DIR_ENV);
        }
    }

    #[test]
    fn non_mise_install_path_is_not_detected() {
        let path = Path::new("/home/user/.local/bin/herdr");

        assert!(!is_mise_managed_exe_path(path));
    }

    #[test]
    fn package_manager_path_detection_follows_homebrew_symlink() {
        #[cfg(unix)]
        {
            let root = std::env::temp_dir().join(format!(
                "herdr-homebrew-symlink-test-{}",
                std::process::id()
            ));
            let cellar_bin = root.join("Cellar/herdr/0.6.2/bin");
            let opt_bin = root.join("opt/herdr/bin");
            fs::create_dir_all(&cellar_bin).unwrap();
            fs::create_dir_all(&opt_bin).unwrap();
            let cellar_binary = cellar_bin.join("herdr");
            let opt_binary = opt_bin.join("herdr");
            fs::write(&cellar_binary, b"").unwrap();
            std::os::unix::fs::symlink(&cellar_binary, &opt_binary).unwrap();

            assert!(is_package_manager_managed_exe_path(&opt_binary));

            let _ = fs::remove_dir_all(root);
        }
    }

    #[test]
    fn package_manager_path_detection_follows_mise_symlink() {
        #[cfg(unix)]
        {
            let root = std::env::temp_dir()
                .join(format!("herdr-mise-symlink-test-{}", std::process::id()));
            let version_bin = root.join("installs/herdr/0.6.2/bin");
            let latest_bin = root.join("installs/herdr/latest/bin");
            fs::create_dir_all(&version_bin).unwrap();
            fs::create_dir_all(&latest_bin).unwrap();
            let version_binary = version_bin.join("herdr");
            let latest_binary = latest_bin.join("herdr");
            fs::write(&version_binary, b"").unwrap();
            std::os::unix::fs::symlink(&version_binary, &latest_binary).unwrap();

            assert!(is_package_manager_managed_exe_path(&latest_binary));

            let _ = fs::remove_dir_all(root);
        }
    }

    #[test]
    fn nix_store_path_is_detected() {
        let path = Path::new("/nix/store/abc123-herdr-0.6.1/bin/herdr");

        assert!(is_nix_store_exe_path(path));
        assert!(is_package_manager_managed_exe_path(path));
    }

    #[test]
    fn preview_channel_is_rejected_for_package_manager_paths() {
        let homebrew = Path::new("/opt/homebrew/Cellar/herdr/0.6.6/bin/herdr");
        let mise = Path::new("/home/user/.local/share/mise/installs/herdr/0.6.6/bin/herdr");
        let nix = Path::new("/nix/store/abc123-herdr-0.6.6/bin/herdr");
        let direct = Path::new("/home/user/.local/bin/herdr");

        assert!(preview_channel_rejection_for_exe_path(homebrew)
            .is_some_and(|message| message.contains("Homebrew")));
        assert!(preview_channel_rejection_for_exe_path(mise)
            .is_some_and(|message| message.contains("mise")));
        assert!(preview_channel_rejection_for_exe_path(nix)
            .is_some_and(|message| message.contains("Nix")));
        assert!(preview_channel_rejection_for_exe_path(direct).is_none());
    }

    #[test]
    fn non_nix_store_path_is_not_detected() {
        let path = Path::new("/usr/local/bin/herdr");

        assert!(!is_nix_store_exe_path(path));
    }

    #[test]
    fn update_install_instruction_distinguishes_install_from_restart() {
        assert_eq!(
            update_install_instruction(HERDR_UPDATE_COMMAND),
            "detach, run `herdr update`, then run Herdr again to reconnect"
        );
        assert_eq!(
            update_install_instruction(HOMEBREW_UPDATE_COMMAND),
            "detach, run `brew update && brew upgrade herdr`, then run Herdr again to reconnect"
        );
        assert_eq!(
            update_install_instruction(MISE_UPDATE_COMMAND),
            "detach, run `mise upgrade herdr`, then run Herdr again to reconnect"
        );
    }

    #[test]
    fn version_ordering() {
        let v010 = Version::parse("0.1.0").unwrap();
        let v011 = Version::parse("0.1.1").unwrap();
        let v020 = Version::parse("0.2.0").unwrap();
        let v100 = Version::parse("1.0.0").unwrap();

        assert!(v010 < v011);
        assert!(v011 < v020);
        assert!(v020 < v100);
        assert!(v010 == Version::parse("0.1.0").unwrap());
    }

    #[test]
    fn version_display() {
        let v = Version {
            major: 0,
            minor: 1,
            patch: 0,
        };
        assert_eq!(v.to_string(), "0.1.0");
    }

    #[test]
    fn current_version_parses() {
        let v = Version::current();
        assert!(v.major < 100);
    }
}
