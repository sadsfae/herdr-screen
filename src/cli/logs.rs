//! `herdr-screen logs` - open a dedicated workspace tailing service logs.
//!
//! Composes existing socket methods client-side (no new codec / no frozen
//! Method variants): `WorkspaceCreate` to make a fresh workspace, `PaneSplit`
//! per target to lay out panes, and `PaneSendInput` (the existing `pane run`
//! primitive) to start a follow-mode tail in each. One command gives a
//! screen-style "logs" workspace, one pane per service.
//!
//! Usage:
//!   herdr logs nginx                  # journalctl -fu nginx
//!   herdr logs nginx postgres         # two panes
//!   herdr logs -t /var/log/foo.log    # tail a file path instead of a systemd unit
use std::collections::HashMap;

use crate::api::schema::{
    Method, PaneRightClickTarget, PaneSendInputParams, PaneSplitParams, SplitDirection,
    WorkspaceCreateParams,
};

pub(super) fn run_logs_command(args: &[String]) -> std::io::Result<i32> {
    let mut targets: Vec<String> = Vec::new();
    let mut file_mode = false;
    let mut focus = true;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-t" | "--file" => {
                file_mode = true;
                index += 1;
            }
            "--no-focus" => {
                focus = false;
                index += 1;
            }
            "help" | "--help" | "-h" => {
                print_logs_help();
                return Ok(0);
            }
            other => {
                targets.push(other.to_string());
                index += 1;
            }
        }
    }

    if targets.is_empty() {
        eprintln!("usage: herdr logs [--file] <service-or-path> [more ...]");
        return Ok(2);
    }

    // 1. Fresh workspace.
    let ws = super::send_request(&crate::api::schema::Request {
        id: "cli:logs:workspace-create".into(),
        method: Method::WorkspaceCreate(WorkspaceCreateParams {
            source_workspace_id: None,
            cwd: None,
            focus,
            label: Some("logs".to_string()),
            env: HashMap::new(),
        }),
    })?;
    if let Some(err) = ws.get("error") {
        eprintln!("{}", serde_json::to_string(&err).unwrap());
        return Ok(1);
    }
    let workspace_id = ws["result"]["workspace"]["workspace_id"]
        .as_str()
        .ok_or_else(|| std::io::Error::other("workspace create did not return a workspace id"))?;
    let root_pane = ws["result"]["root_pane"]["pane_id"]
        .as_str()
        .ok_or_else(|| std::io::Error::other("workspace create did not return a root pane"))?;
    eprintln!("created workspace {workspace_id}");

    // 2. Split a pane per target (first target reuses the root pane).
    let mut pane_ids: Vec<String> = vec![root_pane.to_string()];
    for i in 1..targets.len() {
        // Alternate right/down so panes tile, not stack in one column.
        let direction = if i % 2 == 1 {
            SplitDirection::Right
        } else {
            SplitDirection::Down
        };
        let split = super::send_request(&crate::api::schema::Request {
            id: "cli:logs:pane-split".into(),
            method: Method::PaneSplit(PaneSplitParams {
                workspace_id: Some(workspace_id.to_string()),
                target_pane_id: None,
                direction,
                ratio: None,
                cwd: None,
                focus: false,
                right_click: PaneRightClickTarget::Herdr,
                env: HashMap::new(),
            }),
        })?;
        if let Some(err) = split.get("error") {
            eprintln!("{}", serde_json::to_string(&err).unwrap());
            return Ok(1);
        }
        let new_pane = split["result"]["pane"]["pane_id"]
            .as_str()
            .ok_or_else(|| std::io::Error::other("pane split did not return a pane id"))?;
        pane_ids.push(new_pane.to_string());
    }

    // 3. Run the tail command in each pane.
    for (i, pane_id) in pane_ids.iter().enumerate() {
        let command = tail_command(&targets[i], file_mode);
        super::send_ok_request(Method::PaneSendInput(PaneSendInputParams {
            pane_id: pane_id.clone(),
            text: command,
            keys: vec!["Enter".into()],
        }))?;
    }

    eprintln!("logs workspace ready: {} pane(s)", targets.len());
    Ok(0)
}

fn tail_command(target: &str, file_mode: bool) -> String {
    if file_mode {
        return format!("tail -n 100 -F {target}");
    }
    // A defined log service wins over the default systemd-unit assumption.
    if let Some(command) = super::logs_define::defined_command(target) {
        return command;
    }
    format!("sudo journalctl -fu {target} -n 100")
}

fn print_logs_help() {
    eprintln!("herdr logs commands:");
    eprintln!("  herdr logs <service> [more ...]        tail systemd unit logs (journalctl -fu)");
    eprintln!("  herdr logs --file <path> [more ...]    tail a file path (tail -F)");
    eprintln!();
    eprintln!("Opens a dedicated 'logs' workspace with one pane per target.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tail_command_systemd_unit_uses_journalctl_follow() {
        assert_eq!(
            tail_command("nginx", false),
            "sudo journalctl -fu nginx -n 100"
        );
    }

    #[test]
    fn tail_command_file_uses_tail_follow() {
        assert_eq!(
            tail_command("/var/log/foo.log", true),
            "tail -n 100 -F /var/log/foo.log"
        );
    }

    #[test]
    fn tail_command_file_mode_ignores_service_flag() {
        // A service name passed in file-mode is treated as a path.
        assert_eq!(tail_command("nginx", true), "tail -n 100 -F nginx");
    }
}
