//! `herdr-screen logs-define` - define named log/service watchers.
//!
//! Service definitions live in a small TOML file in the config dir
//! (`logs-services.toml`). `herdr-screen logs <name>` then tails whatever
//! command a definition carries instead of assuming a systemd unit.
//!
//! Usage:
//!   herdr logs-define ServiceName /path/or/command/to/watch
//!   herdr logs-define list
//!   herdr logs-define remove ServiceName
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use crate::config;

/// File name for the service definitions, stored alongside `config.toml`.
pub(super) const LOGS_SERVICES_FILE: &str = "logs-services.toml";

/// Baked-in reference definitions seeded into a brand-new file so `logs` has
/// something useful out of the box without any setup.
const DEFAULT_SERVICES: &[(&str, &str)] = &[
    ("nginx", "sudo journalctl -fu nginx -n 100"),
    ("postgres", "sudo journalctl -fu postgresql -n 100"),
    ("ssh", "sudo journalctl -fu sshd -n 100"),
    ("firewall", "sudo journalctl -fu firewalld -n 100"),
];

pub(super) fn run_logs_define_command(args: &[String]) -> std::io::Result<i32> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        print_logs_define_help();
        return Ok(2);
    };

    match subcommand {
        "list" => list_definitions(),
        "remove" => {
            let name = args.get(1).filter(|s| !s.is_empty());
            match name {
                Some(name) => remove_definition(name),
                None => {
                    eprintln!("usage: herdr logs-define remove <ServiceName>");
                    Ok(2)
                }
            }
        }
        "help" | "--help" | "-h" => {
            print_logs_define_help();
            Ok(0)
        }
        name => define_service(name, args.get(1)),
    }
}

fn services_path() -> PathBuf {
    config::config_dir().join(LOGS_SERVICES_FILE)
}

fn define_service(name: &str, target: Option<&String>) -> std::io::Result<i32> {
    let Some(target) = target else {
        eprintln!("usage: herdr logs-define <ServiceName> <path-or-command>");
        return Ok(2);
    };

    let path = services_path();
    let mut services = read_services(&path);
    services.insert(name.to_string(), target.clone());
    write_services(&path, &services)?;

    println!("defined {name} -> {target}");
    println!("file: {}", path.display());
    Ok(0)
}

fn list_definitions() -> std::io::Result<i32> {
    let path = services_path();
    let services = read_services(&path);
    if services.is_empty() {
        println!("no log services defined in {}", path.display());
        return Ok(0);
    }
    println!("log services ({}):", path.display());
    for (name, command) in services {
        println!("  {name:<12} {command}");
    }
    Ok(0)
}

fn remove_definition(name: &str) -> std::io::Result<i32> {
    let path = services_path();
    let mut services = read_services(&path);
    if services.remove(name).is_none() {
        eprintln!("no log service named '{name}'");
        return Ok(1);
    }
    write_services(&path, &services)?;
    println!("removed {name}");
    Ok(0)
}

/// Read the definitions file, seeding defaults on first run when the file is
/// absent or empty.
fn read_services(path: &PathBuf) -> BTreeMap<String, String> {
    let mut services: BTreeMap<String, String> = match std::fs::read_to_string(path) {
        Ok(content) => toml::from_str(&content).unwrap_or_default(),
        Err(_) => BTreeMap::new(),
    };
    if services.is_empty() {
        for (name, command) in DEFAULT_SERVICES {
            services
                .entry((*name).to_string())
                .or_insert((*command).to_string());
        }
    }
    services
}

fn write_services(path: &PathBuf, services: &BTreeMap<String, String>) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "# herdr-screen log service definitions")?;
    writeln!(file, "# `herdr-screen logs <name>` tails these commands.")?;
    for (name, command) in services {
        writeln!(file, "\"{name}\" = \"{command}\"")?;
    }
    Ok(())
}

/// Look up a defined service command, if any.
pub(super) fn defined_command(name: &str) -> Option<String> {
    let path = services_path();
    read_services(&path).remove(name).map(|mut c| {
        c = c.trim().to_string();
        if !c.is_empty() && c.as_bytes()[0] != b':' {
            c
        } else {
            format!("sudo journalctl -fu {name} -n 100")
        }
    })
}

fn print_logs_define_help() {
    eprintln!("herdr logs-define commands:");
    eprintln!("  herdr logs-define <ServiceName> <path-or-command>");
    eprintln!("     define or overwrite a log watcher");
    eprintln!("  herdr logs-define list");
    eprintln!("     list all defined log watchers");
    eprintln!("  herdr logs-define remove <ServiceName>");
    eprintln!("     remove a defined log watcher");
    eprintln!();
    eprintln!("Definitions live in {}.", services_path().display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_seed_on_empty_file() {
        let path = std::path::PathBuf::from("/nonexistent/logs-services.toml");
        let services = read_services(&path);
        assert!(services.contains_key("nginx"));
        assert!(services.contains_key("postgres"));
    }

    #[test]
    fn defined_command_returns_baked_nginx() {
        assert_eq!(
            defined_command("nginx").as_deref(),
            Some("sudo journalctl -fu nginx -n 100")
        );
    }

    #[test]
    fn roundtrip_preserves_entries() {
        let dir =
            std::env::temp_dir().join(format!("herdr-logs-define-test-{}", std::process::id()));
        let path = dir.join(LOGS_SERVICES_FILE);
        let mut services = BTreeMap::new();
        services.insert(
            "custom".to_string(),
            "tail -F /var/log/custom.log".to_string(),
        );
        write_services(&path, &services).unwrap();
        let loaded = read_services(&path);
        assert_eq!(
            loaded.get("custom").map(String::as_str),
            Some("tail -F /var/log/custom.log")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
