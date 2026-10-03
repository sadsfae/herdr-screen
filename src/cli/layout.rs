use std::path::PathBuf;

use crate::api::schema::{
    LayoutApplyParams, LayoutDescription, LayoutExportParams, Method, Request,
};

pub(super) fn run_layout_command(args: &[String]) -> std::io::Result<i32> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        print_layout_help();
        return Ok(2);
    };

    match subcommand {
        "export" => layout_export(&args[1..]),
        "apply" => layout_apply(&args[1..]),
        "help" | "--help" | "-h" => {
            print_layout_help();
            Ok(0)
        }
        _ => {
            print_layout_help();
            Ok(2)
        }
    }
}

fn layout_export(args: &[String]) -> std::io::Result<i32> {
    let mut tab_id = None;
    let mut pane_id = None;
    let mut file = None;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--tab" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --tab");
                    return Ok(2);
                };
                tab_id = Some(super::normalize_workspace_id(value));
                index += 2;
            }
            "--pane" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --pane");
                    return Ok(2);
                };
                pane_id = Some(value.clone());
                index += 2;
            }
            "--file" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --file");
                    return Ok(2);
                };
                file = Some(PathBuf::from(value));
                index += 2;
            }
            other => {
                eprintln!("unknown option: {other}");
                return Ok(2);
            }
        }
    }

    if tab_id.is_some() && pane_id.is_some() {
        eprintln!("use either --tab or --pane, not both");
        return Ok(2);
    }

    let response = super::send_request(&Request {
        id: "cli:layout:export".into(),
        method: Method::LayoutExport(LayoutExportParams { tab_id, pane_id }),
    })?;

    if response.get("error").is_some() {
        eprintln!("{}", serde_json::to_string(&response).unwrap());
        return Ok(1);
    }

    let layout: LayoutDescription = response
        .pointer("/result/layout")
        .cloned()
        .map(serde_json::from_value::<LayoutDescription>)
        .transpose()
        .map_err(std::io::Error::other)?
        .ok_or_else(|| std::io::Error::other("response missing result.layout"))?;

    let json = serde_json::to_string_pretty(&layout).map_err(std::io::Error::other)?;
    match file {
        Some(path) => std::fs::write(path, json).map_err(std::io::Error::other)?,
        None => println!("{json}"),
    }
    Ok(0)
}

fn layout_apply(args: &[String]) -> std::io::Result<i32> {
    let mut workspace_id = None;
    let mut tab_label = None;
    let mut focus = false;
    let mut file = None;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--workspace" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --workspace");
                    return Ok(2);
                };
                workspace_id = Some(super::normalize_workspace_id(value));
                index += 2;
            }
            "--tab-label" => {
                let Some(value) = args.get(index + 1) else {
                    eprintln!("missing value for --tab-label");
                    return Ok(2);
                };
                tab_label = Some(value.clone());
                index += 2;
            }
            "--focus" => {
                focus = true;
                index += 1;
            }
            other if file.is_none() => {
                file = Some(PathBuf::from(other));
                index += 1;
            }
            other => {
                eprintln!("unknown option: {other}");
                return Ok(2);
            }
        }
    }

    let Some(file) = file else {
        eprintln!(
            "usage: herdr-screen layout apply <file> [--workspace ID] [--tab-label LABEL] [--focus]"
        );
        return Ok(2);
    };
    let content = std::fs::read_to_string(&file).map_err(std::io::Error::other)?;
    let description: LayoutDescription =
        serde_json::from_str(&content).map_err(std::io::Error::other)?;

    let response = super::send_request(&Request {
        id: "cli:layout:apply".into(),
        method: Method::LayoutApply(LayoutApplyParams {
            workspace_id,
            tab_id: None,
            tab_label,
            focus,
            root: description.root,
        }),
    })?;

    if response.get("error").is_some() {
        eprintln!("{}", serde_json::to_string(&response).unwrap());
        return Ok(1);
    }
    println!("{}", serde_json::to_string(&response).unwrap());
    Ok(0)
}

fn print_layout_help() {
    println!("usage: herdr-screen layout <export|apply> [options]");
    println!();
    println!("export: export a workspace/tab layout as a portable JSON description");
    println!("  --tab <tab_id>     export a specific tab");
    println!("  --pane <pane_id>   export the tab containing the pane");
    println!("  --file <path>      write the JSON to a file (default: stdout)");
    println!();
    println!("apply: rebuild a workspace/tab from a layout JSON description");
    println!("  <file>             read the layout JSON from this file");
    println!("  --workspace <id>   replace the given workspace");
    println!("  --tab-label <lbl>  label for the created tab");
    println!("  --focus            focus the created layout");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::schema::{LayoutNode, LayoutPane, SplitDirection};
    use std::collections::HashMap;

    #[test]
    fn layout_description_round_trips_through_json() {
        let layout = LayoutDescription {
            workspace_id: "ws-1".into(),
            tab_id: "tab-1".into(),
            zoomed: false,
            focused_pane_id: "pane-b".into(),
            root: LayoutNode::Split {
                direction: SplitDirection::Right,
                ratio: 0.5,
                first: Box::new(LayoutNode::Pane {
                    pane: LayoutPane {
                        pane_id: Some("pane-a".into()),
                        label: Some("api".into()),
                        cwd: Some("/srv/app".into()),
                        command: Some(vec!["zsh".into()]),
                        env: HashMap::from([("PORT".into(), "8080".into())]),
                    },
                }),
                second: Box::new(LayoutNode::Pane {
                    pane: LayoutPane {
                        pane_id: Some("pane-b".into()),
                        label: Some("logs".into()),
                        cwd: None,
                        command: None,
                        env: HashMap::new(),
                    },
                }),
            },
        };

        let json = serde_json::to_string_pretty(&layout).expect("serialize");
        let back: LayoutDescription = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, layout);
    }

    #[test]
    fn apply_params_root_comes_from_export_description_root() {
        let layout = LayoutDescription {
            workspace_id: "ws-x".into(),
            tab_id: "tab-x".into(),
            zoomed: false,
            focused_pane_id: "p".into(),
            root: LayoutNode::Pane {
                pane: LayoutPane {
                    pane_id: Some("p".into()),
                    label: Some("single".into()),
                    cwd: None,
                    command: None,
                    env: HashMap::new(),
                },
            },
        };
        let params = LayoutApplyParams {
            workspace_id: None,
            tab_id: None,
            tab_label: Some("restored".into()),
            focus: false,
            root: layout.root,
        };
        assert!(matches!(params.root, LayoutNode::Pane { .. }));
        assert_eq!(params.tab_label.as_deref(), Some("restored"));
    }
}
