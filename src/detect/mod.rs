//! Agent state detection via terminal tail pattern matching.
//!
//! Each pane's live bottom-of-buffer text is read periodically and matched
//! against known agent output patterns to determine state.

pub mod manifest;
pub mod manifest_update;

/// The detected state of a terminal pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AgentState {
    /// Agent finished, prompt visible, nothing happening.
    Idle,
    /// Agent is actively working/processing.
    Working,
    /// Agent needs human input and is blocked on a response.
    Blocked,
    /// Plain shell or unrecognized program.
    Unknown,
}

/// Screen-derived agent state plus confidence metadata used for source arbitration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentDetection {
    pub state: AgentState,
    /// True when the current screen is an agent-owned viewer that shows
    /// transcript/history instead of the live prompt state.
    pub skip_state_update: bool,
    /// True when the current screen visibly shows live idle chrome.
    pub visible_idle: bool,
    /// True when the current screen visibly shows live UI chrome that needs
    /// human input. This is stronger than arbitrary prompt-like text in the
    /// scrollback and may override a non-blocked integration state.
    pub visible_blocker: bool,
    /// True when the current screen visibly shows live working chrome. PTY
    /// activity is the normal working authority; this remains diagnostic
    /// metadata and for non-PTY fallback paths.
    pub visible_working: bool,
}

/// Which agent we detected running in a pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Agent {
    Pi,
    Claude,
    Codex,
    Gemini,
    Cursor,
    Devin,
    Antigravity,
    Cline,
    Omp,
    Mastracode,
    OpenCode,
    GithubCopilot,
    Kimi,
    Kiro,
    Droid,
    Amp,
    Grok,
    Hermes,
    Kilo,
    Qodercli,
    Qwen,
    Letta,
    Maki,
    Muse,
    Quads,
    Switch,
}

impl Agent {
    pub const ALL: [Self; 26] = [
        Self::Pi,
        Self::Claude,
        Self::Codex,
        Self::Gemini,
        Self::Cursor,
        Self::Devin,
        Self::Antigravity,
        Self::Cline,
        Self::Omp,
        Self::Mastracode,
        Self::OpenCode,
        Self::GithubCopilot,
        Self::Kimi,
        Self::Kiro,
        Self::Droid,
        Self::Amp,
        Self::Grok,
        Self::Hermes,
        Self::Kilo,
        Self::Qodercli,
        Self::Qwen,
        Self::Letta,
        Self::Maki,
        Self::Muse,
        Self::Quads,
        Self::Switch,
    ];
    pub const SCREEN_MANIFEST_AGENTS: [Self; 24] = [
        Self::Pi,
        Self::Claude,
        Self::Codex,
        Self::Gemini,
        Self::Cursor,
        Self::Devin,
        Self::Antigravity,
        Self::Cline,
        Self::OpenCode,
        Self::GithubCopilot,
        Self::Kimi,
        Self::Kiro,
        Self::Droid,
        Self::Amp,
        Self::Grok,
        Self::Hermes,
        Self::Kilo,
        Self::Qodercli,
        Self::Qwen,
        Self::Letta,
        Self::Maki,
        Self::Muse,
        Self::Quads,
        Self::Switch,
    ];
}

pub fn agent_label(agent: Agent) -> &'static str {
    match agent {
        Agent::Pi => "pi",
        Agent::Claude => "claude",
        Agent::Codex => "codex",
        Agent::Gemini => "gemini",
        Agent::Cursor => "cursor",
        Agent::Devin => "devin",
        Agent::Antigravity => "agy",
        Agent::Cline => "cline",
        Agent::Omp => "omp",
        Agent::Mastracode => "mastracode",
        Agent::OpenCode => "opencode",
        Agent::GithubCopilot => "copilot",
        Agent::Kimi => "kimi",
        Agent::Kiro => "kiro",
        Agent::Droid => "droid",
        Agent::Amp => "amp",
        Agent::Grok => "grok",
        Agent::Hermes => "hermes",
        Agent::Kilo => "kilo",
        Agent::Qodercli => "qodercli",
        Agent::Qwen => "qwen",
        Agent::Letta => "letta",
        Agent::Maki => "maki",
        Agent::Muse => "muse",
        Agent::Quads => "quads",
        Agent::Switch => "switch",
    }
}

pub fn interactive_agent_executable(agent: Agent) -> &'static str {
    match agent {
        Agent::Pi => "pi",
        Agent::Claude => "claude",
        Agent::Codex => "codex",
        Agent::Gemini => "gemini",
        Agent::Cursor => {
            if cfg!(windows) {
                "cursor-agent.cmd"
            } else {
                "cursor-agent"
            }
        }
        Agent::Devin => "devin",
        Agent::Antigravity => "agy",
        Agent::Cline => "cline",
        Agent::Omp => "omp",
        Agent::Mastracode => "mastracode",
        Agent::OpenCode => "opencode",
        Agent::GithubCopilot => "copilot",
        Agent::Kimi => "kimi",
        Agent::Kiro => "kiro-cli",
        Agent::Droid => "droid",
        Agent::Amp => "amp",
        Agent::Grok => "grok",
        Agent::Hermes => "hermes",
        Agent::Kilo => "kilo",
        Agent::Qodercli => "qodercli",
        Agent::Qwen => "qwen",
        Agent::Letta => "letta",
        Agent::Maki => "maki",
        Agent::Muse => "muse",
        Agent::Quads => "quads-client",
        Agent::Switch => "ssh",
    }
}

pub fn parse_agent_label(agent: &str) -> Option<Agent> {
    let name = normalized_agent_lookup_name(agent);
    parse_canonical_agent_label(&name).or_else(|| lookup_agent(&name))
}

pub(crate) fn parse_canonical_agent_label(label: &str) -> Option<Agent> {
    let agent = lookup_agent(label)?;
    (agent_label(agent) == label).then_some(agent)
}

fn lookup_agent(name: &str) -> Option<Agent> {
    let name = path_basename(name);
    match name {
        "pi" => Some(Agent::Pi),
        "claude" | "claude-code" => Some(Agent::Claude),
        "codex" => Some(Agent::Codex),
        "gemini" => Some(Agent::Gemini),
        "cursor" | "cursor-agent" => Some(Agent::Cursor),
        "devin" | "devin-cli" | "devin cli" => Some(Agent::Devin),
        "agy" | "antigravity" | "antigravity-cli" => Some(Agent::Antigravity),
        "cline" | ".cline" => Some(Agent::Cline),
        "omp" => Some(Agent::Omp),
        "mastracode" | "mastra-code" | "mastra code" => Some(Agent::Mastracode),
        "opencode" | "opencode2" | "open-code" => Some(Agent::OpenCode),
        "copilot" | "github-copilot" | "ghcs" => Some(Agent::GithubCopilot),
        "kimi" | "kimi-code" | "kimi code" => Some(Agent::Kimi),
        "kiro" | "kiro-cli" => Some(Agent::Kiro),
        "droid" => Some(Agent::Droid),
        "amp" | "amp-local" => Some(Agent::Amp),
        "grok" | "grok-build" => Some(Agent::Grok),
        "hermes" | "hermes-agent" => Some(Agent::Hermes),
        "kilo" | "kilo-code" | "kilo code" => Some(Agent::Kilo),
        "qodercli" | "qoderclicn" | "qoder" | "qodercn" => Some(Agent::Qodercli),
        "qwen" | "qwen-code" | "qwen code" => Some(Agent::Qwen),
        "letta" | "letta-code" | "letta code" => Some(Agent::Letta),
        "maki" => Some(Agent::Maki),
        "muse" | "muse-code" | "muse-cli" => Some(Agent::Muse),
        "quads" | "quads-client" | "quads client" => Some(Agent::Quads),
        "switch" | "network-device" => Some(Agent::Switch),
        _ if is_muse_versioned_binary(name) => Some(Agent::Muse),
        _ => None,
    }
}

/// Muse's install-dir launcher script resolves the active release and execs
/// `muse-bin-<version>` (e.g. `muse-bin-0.1.0-R708.1`), so the running
/// process never carries a bare `muse`/`muse-bin` alias. Require a digit
/// immediately after the `muse-bin-` prefix so unrelated binaries such as
/// `muse-binary` or a bare `muse-bin` stay unmatched.
/// Accepts path-qualified `argv0` values by checking only the basename, since
/// the launcher may `exec` with an absolute install-dir path.
fn is_muse_versioned_binary(name: &str) -> bool {
    path_basename(name)
        .strip_prefix("muse-bin-")
        .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
}

/// Identify which agent is running from the process name.
/// Returns `None` for plain shells or unrecognized programs.
pub fn identify_agent(process_name: &str) -> Option<Agent> {
    parse_agent_label(process_name)
}

pub fn identify_agent_in_job(job: &crate::platform::ForegroundJob) -> Option<(Agent, String)> {
    if let Some(process) = job
        .processes
        .iter()
        .find(|process| process.pid == job.process_group_id)
    {
        let candidate = normalized_process_name(process);
        if let Some(agent) = identify_agent(&candidate) {
            if agent != Agent::Letta || is_interactive_letta_process(process) {
                return Some((agent, candidate));
            }
        }
    }

    let mut best: Option<(u8, Agent, String)> = None;

    for process in &job.processes {
        let candidate = normalized_process_name(process);
        let Some(agent) = identify_agent(&candidate) else {
            continue;
        };
        if agent == Agent::Letta && !is_interactive_letta_process(process) {
            continue;
        }
        let score = process_priority(process, &candidate);

        match &best {
            Some((best_score, _, _)) if *best_score >= score => {}
            _ => best = Some((score, agent, candidate)),
        }
    }

    best.map(|(_, agent, name)| (agent, name))
}

/// Detect the state of an agent from the live terminal tail snapshot.
/// If `agent` is `None`, returns `Unknown`.
#[cfg(test)]
pub fn detect_state(agent: Option<Agent>, screen_content: &str) -> AgentState {
    detect_agent(agent, screen_content).state
}

/// Detect state and whether a visible blocker is present on the current screen.
#[allow(dead_code)] // shim for existing callers; detect_agent_with_osc is the real path
pub fn detect_agent(agent: Option<Agent>, screen_content: &str) -> AgentDetection {
    detect_agent_with_osc(agent, screen_content, "", "")
}

/// Detect state using screen content plus OSC title/progress strings.
pub fn detect_agent_with_osc(
    agent: Option<Agent>,
    screen_content: &str,
    osc_title: &str,
    osc_progress: &str,
) -> AgentDetection {
    let Some(agent) = agent else {
        return AgentDetection {
            state: AgentState::Unknown,
            skip_state_update: false,
            visible_idle: false,
            visible_blocker: false,
            visible_working: false,
        };
    };
    manifest::detect_with_osc(
        agent,
        manifest::DetectionInput {
            screen: screen_content,
            osc_title,
            osc_progress,
        },
    )
}

pub(crate) fn full_lifecycle_hook_authority(source: &str, agent_label: &str) -> bool {
    matches!(
        (source, agent_label),
        ("herdr:pi", "pi")
            | ("herdr:omp", "omp")
            | ("herdr:mastracode", "mastracode")
            | ("herdr:opencode", "opencode")
            | ("herdr:kilo", "kilo")
            | ("herdr:kimi", "kimi")
    )
}

pub(crate) fn session_identity_only_integration(source: &str, agent_label: &str) -> bool {
    matches!(
        (source, agent_label),
        ("herdr:hermes", "hermes")
            | ("herdr:qwen", "qwen")
            | ("herdr:letta", "letta")
            | ("herdr:antigravity_cli", "agy")
    )
}

// ---------------------------------------------------------------------------
// Process identification (platform-specific)
// ---------------------------------------------------------------------------

/// Get the foreground job for a given child PID.
/// Delegates to platform-specific implementation.
pub fn foreground_job(child_pid: u32) -> Option<crate::platform::ForegroundJob> {
    crate::platform::foreground_job(child_pid)
}

/// Get the foreground process group leader as a one-process job.
/// This is cheaper than collecting every process in the foreground job.
pub fn foreground_group_leader_job(
    process_group_id: u32,
) -> Option<crate::platform::ForegroundJob> {
    crate::platform::foreground_group_leader_job(process_group_id)
}

/// Get the foreground process group for a pane shell PID.
/// This is cheaper than collecting every process in the foreground job.
pub fn foreground_process_group_id(child_pid: u32) -> Option<u32> {
    crate::platform::foreground_process_group_id(child_pid)
}

/// True when the pane's own shell is at its prompt with nothing running in it.
pub fn pane_shell_is_idle(child_pid: u32) -> bool {
    crate::platform::available_pane_shell(child_pid).is_some()
}

fn normalized_process_name(process: &crate::platform::ForegroundProcess) -> String {
    let effective = process.argv0.as_deref().unwrap_or(&process.name);
    let lower_effective = effective.to_lowercase();

    if is_generic_runtime_or_shell(&lower_effective) {
        if let Some(wrapped_agent) =
            wrapped_agent_name_from_runtime_argv(&lower_effective, process.argv.as_deref())
        {
            return wrapped_agent;
        }
    }

    if identify_agent(effective).is_some() {
        return effective.to_string();
    }

    if let Some(runtime) = process.argv.as_deref().and_then(|argv| argv.first()) {
        let runtime_name = normalized_agent_lookup_name(path_basename(runtime));
        if matches!(runtime_name.as_str(), "node" | "bun") {
            if let Some(wrapped_agent) =
                wrapped_agent_name_from_runtime_argv(runtime, process.argv.as_deref())
            {
                if matches!(
                    identify_agent(&wrapped_agent),
                    Some(Agent::Qwen | Agent::Cline | Agent::Letta)
                ) {
                    return wrapped_agent;
                }
            }
        }
    }

    if let Some(wrapped_agent) = argv0_agent_name(process.argv.as_deref())
        .or_else(|| cmdline_argv0_agent_name(process.cmdline.as_deref().unwrap_or_default()))
    {
        return wrapped_agent;
    }

    effective.to_string()
}

fn wrapped_agent_name_from_runtime_argv(runtime: &str, argv: Option<&[String]>) -> Option<String> {
    let argv = argv?;
    let runtime_name = normalized_agent_lookup_name(path_basename(runtime));

    match runtime_name.as_str() {
        "node" => cursor_agent_name_from_bundled_node_argv(argv)
            .or_else(|| script_arg_agent_name(argv, &["-e", "--eval", "-p", "--print"], &[])),
        "bun" => script_arg_agent_name(argv, &["-e", "--eval", "-p", "--print"], &[]),
        name if is_python_runtime(name) => script_arg_agent_name(argv, &["-c"], &["-m"]),
        "sh" | "bash" | "zsh" | "fish" => script_arg_agent_name(argv, &["-c"], &[]),
        "cmd" => windows_cmd_arg_agent_name(argv),
        "powershell" | "pwsh" => powershell_arg_agent_name(argv),
        "tmux" => None,
        _ => None,
    }
}

fn cursor_agent_name_from_bundled_node_argv(argv: &[String]) -> Option<String> {
    let (runtime_parent, runtime_name) = path_parent_and_basename(argv.first()?)?;
    let (script_parent, script_name) = path_parent_and_basename(argv.get(1)?)?;
    if !runtime_name.eq_ignore_ascii_case("node.exe")
        || !script_name.eq_ignore_ascii_case("index.js")
        || !runtime_parent.eq_ignore_ascii_case(script_parent)
    {
        return None;
    }

    let mut tail = runtime_parent
        .rsplit(['/', '\\'])
        .filter(|component| !component.is_empty());
    let (Some(version), Some(versions), Some(package)) = (tail.next(), tail.next(), tail.next())
    else {
        return None;
    };
    (package.eq_ignore_ascii_case("cursor-agent")
        && versions.eq_ignore_ascii_case("versions")
        && !version.trim().is_empty())
    .then(|| agent_label(Agent::Cursor).to_string())
}

fn path_parent_and_basename(path: &str) -> Option<(&str, &str)> {
    let split = path.rfind(['/', '\\'])?;
    let parent = path[..split].trim_end_matches(['/', '\\']);
    let basename = &path[split + 1..];
    (!parent.is_empty() && !basename.is_empty()).then_some((parent, basename))
}

fn windows_cmd_arg_agent_name(argv: &[String]) -> Option<String> {
    let mut args = argv.iter().skip(1);
    while let Some(arg) = args.next() {
        let flag = arg.trim_matches('"').to_lowercase();
        match flag.as_str() {
            "/c" | "/k" => {
                return args
                    .next()
                    .and_then(|command| command_text_agent_name(command))
            }
            "/d" | "/s" | "/q" | "/a" | "/u" | "/e:on" | "/e:off" | "/f:on" | "/f:off"
            | "/v:on" | "/v:off" => continue,
            _ => {}
        }
    }
    None
}

fn powershell_arg_agent_name(argv: &[String]) -> Option<String> {
    let mut args = argv.iter().skip(1);
    while let Some(arg) = args.next() {
        let flag = arg.trim_matches('"').to_lowercase();
        match flag.as_str() {
            "-file" | "-f" | "/file" => {
                return args
                    .next()
                    .and_then(|path| agent_name_from_path_token(path));
            }
            "-command" | "-c" | "/command" | "/c" => {
                return args
                    .next()
                    .and_then(|command| command_text_agent_name(command));
            }
            "-encodedcommand" | "-enc" | "/encodedcommand" | "/enc" => return None,
            "-configurationname" | "-executionpolicy" | "-outputformat" | "-psconsolefile"
            | "-version" | "-windowstyle" | "-workingdirectory" => {
                let _ = args.next();
            }
            _ if flag.starts_with('-') || flag.starts_with('/') => {}
            _ => return agent_name_from_path_token(arg),
        }
    }
    None
}

fn command_text_agent_name(command: &str) -> Option<String> {
    let mut rest = command;
    while let Some((token, next)) = command_text_token(rest) {
        let token = token.trim();
        if token.eq_ignore_ascii_case("&")
            || token.eq_ignore_ascii_case(".")
            || token.eq_ignore_ascii_case("call")
        {
            rest = next;
            continue;
        }
        return agent_name_from_path_token(token);
    }
    None
}

fn command_text_token(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    let first = input.chars().next()?;
    if first == '"' || first == '\'' {
        let start = first.len_utf8();
        if let Some(end) = input[start..].find(first) {
            let end = start + end;
            return Some((&input[start..end], &input[end + first.len_utf8()..]));
        }
        return Some((&input[start..], ""));
    }

    let end = input.find(char::is_whitespace).unwrap_or(input.len());
    Some((&input[..end], &input[end..]))
}

fn script_arg_agent_name(
    argv: &[String],
    eval_flags: &[&str],
    module_flags: &[&str],
) -> Option<String> {
    let mut args = argv.iter().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--" {
            return args
                .next()
                .and_then(|token| agent_name_from_path_token(token));
        }

        if flag_matches(arg, eval_flags) || flag_matches(arg, module_flags) {
            return None;
        }

        if arg.starts_with('-') {
            if option_takes_value(arg) {
                let _ = args.next();
            }
            continue;
        }

        return agent_name_from_path_token(arg);
    }

    None
}

fn flag_matches(arg: &str, flags: &[&str]) -> bool {
    flags
        .iter()
        .any(|flag| arg == *flag || short_flag_payload(arg, flag) || long_flag_value(arg, flag))
}

fn short_flag_payload(arg: &str, flag: &str) -> bool {
    flag.starts_with('-')
        && !flag.starts_with("--")
        && arg.starts_with(flag)
        && arg.len() > flag.len()
}

fn long_flag_value(arg: &str, flag: &str) -> bool {
    flag.starts_with("--")
        && arg
            .strip_prefix(flag)
            .is_some_and(|rest| rest.starts_with('='))
}

fn option_takes_value(arg: &str) -> bool {
    matches!(
        arg,
        "-r" | "--require"
            | "--loader"
            | "--import"
            | "--experimental-loader"
            | "--inspect-port"
            | "-W"
            | "-X"
            | "-S"
            | "-L"
            | "-o"
    )
}

fn argv0_agent_name(argv: Option<&[String]>) -> Option<String> {
    agent_name_from_path_token(argv?.first()?)
}

fn cmdline_argv0_agent_name(cmdline: &str) -> Option<String> {
    agent_name_from_path_token(cmdline.split_whitespace().next()?)
}

fn agent_name_from_path_token(token: &str) -> Option<String> {
    let trimmed = token.trim_matches(|c| matches!(c, '"' | '\''));
    if trimmed.is_empty() || trimmed.starts_with('-') {
        return None;
    }

    agent_name_from_basename(path_basename(trimmed))
        .or_else(|| agent_name_from_known_package_path(trimmed))
        .or_else(|| resolved_agent_name_from_path_token(trimmed))
}

fn agent_name_from_known_package_path(path: &str) -> Option<String> {
    let raw_components: Vec<&str> = path
        .split(['/', '\\'])
        .filter(|component| !component.is_empty())
        .collect();
    let ends_with = |suffix: &[&str]| {
        raw_components.len() >= suffix.len()
            && raw_components[raw_components.len() - suffix.len()..]
                .iter()
                .zip(suffix)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    };
    if ends_with(&[
        "node_modules",
        "@earendil-works",
        "pi-coding-agent",
        "dist",
        "cli.js",
    ]) || ends_with(&[
        "node_modules",
        "@earendil-works",
        "pi-coding-agent",
        "dist",
        "bundle",
        "cli.js",
    ]) {
        return Some(agent_label(Agent::Pi).to_string());
    }
    if ends_with(&[
        "node_modules",
        "@oh-my-pi",
        "pi-coding-agent",
        "dist",
        "cli.js",
    ]) {
        return Some(agent_label(Agent::Omp).to_string());
    }
    if ends_with(&[
        "node_modules",
        "@moonshot-ai",
        "kimi-code",
        "dist",
        "main.mjs",
    ]) {
        return Some(agent_label(Agent::Kimi).to_string());
    }

    let components: Vec<String> = raw_components
        .into_iter()
        .map(normalized_agent_lookup_name)
        .collect();
    for window in components.windows(5) {
        if window == ["node_modules", "@qwen-code", "qwen-code", "dist", "index"] {
            return Some(agent_label(Agent::Qwen).to_string());
        }
    }
    for window in components.windows(4) {
        if window == ["node_modules", "mastracode", "dist", "cli"] {
            return Some(agent_label(Agent::Mastracode).to_string());
        }
        if window == ["node_modules", "@letta-ai", "letta-code", "letta"] {
            return Some(agent_label(Agent::Letta).to_string());
        }
    }
    None
}

fn letta_entrypoint_index(argv: &[String]) -> Option<usize> {
    let is_letta =
        |arg: &str| agent_name_from_path_token(arg).as_deref() == Some(agent_label(Agent::Letta));
    if argv.first().is_some_and(|arg| is_letta(arg)) {
        return Some(0);
    }

    let runtime = argv
        .first()
        .map(|arg| normalized_agent_lookup_name(path_basename(arg)))?;
    if !matches!(runtime.as_str(), "node" | "bun") {
        return None;
    }

    let mut index = 1;
    while let Some(arg) = argv.get(index) {
        if arg == "--" {
            return argv
                .get(index + 1)
                .is_some_and(|arg| is_letta(arg))
                .then_some(index + 1);
        }
        if flag_matches(arg, &["-e", "--eval", "-p", "--print"]) {
            return None;
        }
        if arg.starts_with('-') {
            index += if option_takes_value(arg) { 2 } else { 1 };
            continue;
        }
        return is_letta(arg).then_some(index);
    }
    None
}

fn letta_first_arg_after_backend_selection(args: &[String]) -> Option<&str> {
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if arg == "--backend" {
            let _ = args.next();
            continue;
        }
        if arg.starts_with("--backend=") {
            continue;
        }
        return Some(arg);
    }
    None
}

fn is_interactive_letta_process(process: &crate::platform::ForegroundProcess) -> bool {
    let parsed_cmdline;
    let argv = if let Some(argv) = process.argv.as_deref() {
        argv
    } else {
        parsed_cmdline = process
            .cmdline
            .as_deref()
            .unwrap_or_default()
            .split_whitespace()
            .map(|arg| arg.trim_matches(|ch| matches!(ch, '\'' | '"')).to_string())
            .collect::<Vec<_>>();
        if parsed_cmdline.is_empty() {
            return true;
        }
        &parsed_cmdline
    };

    let cli_args = letta_entrypoint_index(argv)
        .map(|index| &argv[index + 1..])
        .unwrap_or(argv);

    if cli_args.iter().any(|arg| {
        let option = arg.split_once('=').map_or(arg.as_str(), |(name, _)| name);
        matches!(
            option,
            "-p" | "--print"
                | "--prompt"
                | "--json"
                | "--stream-json"
                | "--run"
                | "--disable-memory-guard"
                | "--output-format"
                | "--input-format"
                | "--include-partial-messages"
                | "--from-agent"
                | "--environment"
                | "--env"
                | "--pre-load-skills"
                | "--tags"
                | "--ephemeral"
                | "--stateless"
                | "--max-turns"
                | "--memfs-startup"
                | "-h"
                | "--help"
                | "-v"
                | "--version"
                | "--info"
                | "--update"
                | "--upgrade"
        )
    }) {
        return false;
    }

    letta_first_arg_after_backend_selection(cli_args).is_none_or(|arg| arg.starts_with('-'))
}

fn resolved_agent_name_from_path_token(token: &str) -> Option<String> {
    let path = std::path::Path::new(token);
    if path.components().count() < 2 {
        return None;
    }

    let resolved = std::fs::canonicalize(path).ok()?;
    let basename = resolved.file_name()?.to_str()?;
    agent_name_from_basename(basename)
}

fn agent_name_from_basename(basename: &str) -> Option<String> {
    let agent = parse_agent_label(basename)?;
    Some(agent_label(agent).to_string())
}

fn normalized_agent_lookup_name(name: &str) -> String {
    let mut name = name.trim().to_lowercase();
    for suffix in [".exe", ".cmd", ".bat", ".ps1", ".js"] {
        if name.ends_with(suffix) {
            name.truncate(name.len() - suffix.len());
            break;
        }
    }
    name
}

fn path_basename(path: &str) -> &str {
    path.rsplit(['/', '\\'])
        .find(|component| !component.is_empty())
        .unwrap_or(path)
}

/// True when a foreground process name is an interactive network login client
/// (the common way to reach a managed switch). Switches are reached over an
/// interactive SSH or telnet session, so a running client is the gate for
/// attempting content-based switch identification.
pub fn is_network_client_process_name(name: &str) -> bool {
    let base = normalized_agent_lookup_name(path_basename(name));
    matches!(base.as_str(), "ssh" | "telnet" | "slogin")
}

/// Short ssh options that consume a value (`-o`, `-p`, `-l`, ...). Everything
/// else in a cluster is a flag.
const SSH_OPTIONS_WITH_VALUE: &[char] = &[
    'b', 'B', 'c', 'D', 'E', 'e', 'F', 'I', 'i', 'J', 'L', 'l', 'm', 'O', 'o', 'P', 'p', 'Q', 'R',
    'S', 'W', 'w',
];

/// Short telnet options that consume a value. telnet(1) shares the `host`
/// argument shape but not the option letters, so the ssh table cannot be used
/// for it: `-X atype`, `-k realm` and `-n tracefile` take values there, while
/// `-c`, `-D`, `-E`, `-F`, `-K` and `-L` are flags.
const TELNET_OPTIONS_WITH_VALUE: &[char] = &['b', 'e', 'k', 'l', 'n', 'S', 'V', 'X'];

/// Destination host of an ssh or telnet command line: the first bare token
/// after the options, with any `user@`, `ssh://` prefix and `:port` / `/path`
/// suffix removed. `None` when the command line carries no destination (plain
/// `ssh` reading from stdin, or `ssh -v`).
pub fn network_client_destination(argv: &[String]) -> Option<String> {
    let options_with_value = match argv.first() {
        Some(program) if normalized_agent_lookup_name(path_basename(program)) == "telnet" => {
            TELNET_OPTIONS_WITH_VALUE
        }
        _ => SSH_OPTIONS_WITH_VALUE,
    };
    let mut index = 1;
    let mut after_terminator = false;
    while let Some(token) = argv.get(index) {
        let token = token.as_str();
        if after_terminator || !token.starts_with('-') || token == "-" {
            return Some(clean_network_client_destination(token));
        }
        if token == "--" {
            after_terminator = true;
            index += 1;
            continue;
        }
        if token.starts_with("--") {
            index += 1;
            continue;
        }
        // A cluster of short options: the first option that consumes a value
        // decides whether the next token is that value or the value is inline.
        let mut takes_next_value = false;
        for (offset, flag) in token[1..].char_indices() {
            if options_with_value.contains(&flag) {
                takes_next_value = token[2 + offset..].is_empty();
                break;
            }
        }
        index += if takes_next_value { 2 } else { 1 };
    }
    None
}

fn clean_network_client_destination(token: &str) -> String {
    let without_scheme = token.strip_prefix("ssh://").unwrap_or(token);
    let without_user = without_scheme
        .rsplit_once('@')
        .map_or(without_scheme, |(_, host)| host);
    if let Some(rest) = without_user.strip_prefix('[') {
        return rest.split(']').next().unwrap_or(rest).trim().to_string();
    }
    // More than one colon means an unbracketed IPv6 literal, not `host:port`:
    // ssh requires brackets to add a port to an address.
    let host = if without_user.matches(':').count() > 1 {
        without_user
    } else {
        without_user
            .split_once(':')
            .map_or(without_user, |(host, _port)| host)
    };
    host.split('/').next().unwrap_or(host).trim().to_string()
}

/// One configured `[switches] hosts` entry: an exact hostname, or a compiled
/// glob when the entry contains `*` or `?`.
struct SwitchHostPattern {
    literal: String,
    glob: Option<regex::Regex>,
}

impl SwitchHostPattern {
    fn compile(pattern: &str) -> Option<Self> {
        let trimmed = pattern.trim().to_lowercase();
        if trimmed.is_empty() {
            return None;
        }
        if !trimmed.contains(['*', '?']) {
            return Some(Self {
                literal: trimmed,
                glob: None,
            });
        }
        let mut regex = String::from("^");
        for character in trimmed.chars() {
            match character {
                '*' => regex.push_str(".*"),
                '?' => regex.push('.'),
                other => regex.push_str(&regex::escape(&other.to_string())),
            }
        }
        regex.push('$');
        Some(Self {
            literal: trimmed,
            glob: regex::Regex::new(&regex).ok(),
        })
    }

    fn matches(&self, host: &str) -> bool {
        match self.glob.as_ref() {
            Some(glob) => glob.is_match(host),
            None => self.literal == host,
        }
    }
}

/// Switch hostnames declared by the operator (`[switches] hosts`). Read by the
/// pane detection loop; refreshed whenever the config is applied.
static SWITCH_HOST_PATTERNS: std::sync::LazyLock<std::sync::RwLock<Vec<SwitchHostPattern>>> =
    std::sync::LazyLock::new(|| std::sync::RwLock::new(Vec::new()));

/// Replace the configured switch host list. Entries are trimmed and matched
/// case-insensitively; empty entries are ignored.
pub fn set_switch_host_patterns(patterns: &[String]) {
    let compiled: Vec<SwitchHostPattern> = patterns
        .iter()
        .filter_map(|pattern| SwitchHostPattern::compile(pattern))
        .collect();
    let mut guard = SWITCH_HOST_PATTERNS
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = compiled;
}

/// Title for a pane whose ssh/telnet destination matches a configured switch
/// host: the host's short name (`leaf1.dc1.example.com` -> `leaf1`), or the
/// address itself for an IP literal. Entries are matched case-insensitively
/// against the destination's host part (`user@`, a `ssh://` scheme, and any
/// port or path are stripped) and its short name, so `core-sw1` in the config
/// matches `ssh core-sw1.dc1.example.com` as well as `ssh core-sw1`.
pub fn switch_host_title(destination: &str) -> Option<String> {
    let patterns = SWITCH_HOST_PATTERNS
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if patterns.is_empty() {
        return None;
    }
    switch_host_title_with(&patterns, destination)
}

fn switch_host_title_with(patterns: &[SwitchHostPattern], destination: &str) -> Option<String> {
    let host = clean_network_client_destination(destination).to_lowercase();
    if host.is_empty() {
        return None;
    }
    let short = short_host_name(&host);
    let matched = patterns
        .iter()
        .any(|pattern| pattern.matches(&host) || (short != host && pattern.matches(short)));
    matched.then_some(short.to_string())
}

/// First label of a hostname (`leaf1.dc1.example.com` -> `leaf1`). IP literals
/// are returned unchanged: `10.0.0.5` is not a name with a short form.
fn short_host_name(host: &str) -> &str {
    if host.parse::<std::net::IpAddr>().is_ok() {
        return host;
    }
    host.split('.').next().unwrap_or(host)
}

/// Network OS family proven by an identifying signature. Used for the pane's
/// display label (`sw-juniper`), never for the agent kind, which stays
/// `switch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchVendor {
    Juniper,
    Arista,
    Cisco,
}

impl SwitchVendor {
    /// Sidebar label used when the vendor is known.
    pub fn label(self) -> &'static str {
        match self {
            Self::Juniper => "sw-juniper",
            Self::Arista => "sw-arista",
            Self::Cisco => "sw-cisco",
        }
    }
}

/// Prompt and login-banner signatures are matched against this many trailing
/// lines: a prompt further up in scrollback must not identify the session, or
/// claim its vendor.
const SWITCH_SIGNATURE_TAIL_LINES: usize = 8;

/// One identifying signature with the vendor it proves. `vendor` is `None` for
/// shared device prompt forms that only prove "some network OS CLI".
#[derive(Debug)]
pub(crate) struct SwitchIdentitySignature {
    pub(crate) vendor: Option<SwitchVendor>,
    /// Prompt and login-banner forms are matched against the screen tail only,
    /// so a stale prompt further up in scrollback cannot keep a pane
    /// classified. Vendor output is matched against the whole visible screen,
    /// because `show version`-class output scrolls past the tail in one frame.
    /// The tradeoff of the wider region: a pane whose ssh/telnet client
    /// displays captured device output, or a previous session's output still on
    /// screen, is classified as a switch while that content is visible.
    pub(crate) tail_only: bool,
    pub(crate) regex: regex::Regex,
}

/// Vendor named in a manifest identity entry
/// (`switch.toml`'s `[[identity]]` tables).
pub(crate) fn parse_switch_vendor(value: &str) -> Option<SwitchVendor> {
    match value.trim().to_ascii_lowercase().as_str() {
        "juniper" => Some(SwitchVendor::Juniper),
        "arista" => Some(SwitchVendor::Arista),
        "cisco" => Some(SwitchVendor::Cisco),
        _ => None,
    }
}

/// Identifying signatures for switch sessions: the loaded switch manifest's
/// `[[identity]]` entries when it has any, otherwise the compile-time fallback
/// below, so a manifest without identity (or one that failed to load) cannot
/// regress identification.
pub(crate) fn switch_signatures() -> std::sync::Arc<[SwitchIdentitySignature]> {
    let from_manifest = manifest::switch_identity_signatures();
    if from_manifest.is_empty() {
        fallback_switch_identity_signatures()
    } else {
        from_manifest
    }
}

/// Compile-once fallback signatures, used only when the manifest carries no
/// identity section.
///
/// Every prompt pattern is line-anchored and excludes the `:` path and `~` that
/// a normal Linux/BSD shell prompt carries, so a plain shell with a path
/// component (e.g. `user@host:~$`, `root@host:~#`) never matches; a PS1
/// without a path (`user@host#`) is indistinguishable from a JunOS prompt and
/// does match. A bare `host>` / `host#` EXEC prompt is deliberately NOT an
/// identity signal: a Linux box whose PS1 is just the hostname presents
/// exactly the same line, so such a session must not be classified as a switch.
/// Arista/Cisco EXEC-only sessions are picked up once the session shows a
/// stronger signature (vendor output or config mode), or via
/// `agent start --kind switch`, which seeds the pane's identity from the
/// launched ssh process.
///
/// Arista EOS ships no vendor text at login (`no banner login` and
/// `no banner motd` in its default hardening config) and its login line is
/// byte-identical to sshd's `Last login: ...`, so EOS sessions are identified
/// from what the device prints once the operator runs anything:
/// - `Arista DCS-...` / `Arista vEOS` model line and `Software image version:`
///   from `show version`,
/// - the `Arista Networks EOS shell` banner printed when entering `bash`
///   (the EOS shell prompt itself is a plain `[admin@host ~]$`),
/// - the EOS CLI error grammar `Invalid input (at token N: 'x')` and
///   `Invalid input (privileged mode required)`.
///
/// Cisco IOS / IOS-XE `show version` output is the equivalent signal there.
fn fallback_switch_identity_signatures() -> std::sync::Arc<[SwitchIdentitySignature]> {
    static SIGNATURES: std::sync::LazyLock<std::sync::Arc<[SwitchIdentitySignature]>> =
        std::sync::LazyLock::new(|| {
            use SwitchVendor::{Arista, Cisco, Juniper};
            let signatures: [(Option<SwitchVendor>, bool, &str); 11] = [
                // Arista EOS `show version` model line ("Arista DCS-7280SRA-48C6-F",
                // "Arista vEOS"), including the legacy vendor-word header form
                // ("Arista Networks 7050TX-64").
                (
                    Some(Arista),
                    false,
                    r"^\s*Arista\s+(?:(?:DCS|CCS|vEOS)\S*|Networks\s+\S+)\s*$",
                ),
                // Software version line from a device `show version`. Other network
                // OS families print this label too, so it identifies a device CLI
                // without claiming a vendor.
                (None, false, r"^\s*Software image version:\s*\S+\s*$"),
                // Arista EOS bash banner: "Arista Networks EOS shell".
                (Some(Arista), false, r"^\s*Arista Networks EOS shell\s*$"),
                // Arista EOS CLI errors ("% Invalid input (at token 1: 'show foo')",
                // "% Invalid input (privileged mode required)").
                (
                    Some(Arista),
                    false,
                    r"^\s*%?\s*Invalid input \(at token \d+: '[^']*'\)\s*$",
                ),
                (
                    Some(Arista),
                    false,
                    r"^\s*%?\s*Invalid input \(privileged mode required\)\s*$",
                ),
                // Cisco IOS / IOS-XE `show version` header.
                (Some(Cisco), false, r"^\s*Cisco IOS(?: XE)? Software"),
                // Juniper JunOS login banner (strong vendor signature).
                (Some(Juniper), true, r"^---\s*JUNOS\s+\S+"),
                // Juniper config breadcrumb banner (e.g. "[edit protocols ospf]").
                (Some(Juniper), true, r"^\[edit[^\]]*\]\s*$"),
                // Juniper operational prompt: user@host> (no `:` path, no `~`).
                (Some(Juniper), true, r"^\S+@[^:\s>]+>\s*$"),
                // Juniper config prompt: user@host#.
                (Some(Juniper), true, r"^\S+@[^:\s#]+#\s*$"),
                // Arista / Cisco / EOS config and sub-config prompt: host(config...)#
                // (shared form: identifies a device CLI, not a vendor).
                (
                    None,
                    true,
                    r"^[A-Za-z0-9][A-Za-z0-9._-]*\(config(?:-[^)]+)?\)#\s*$",
                ),
            ];
            signatures
                .into_iter()
                .map(|(vendor, tail_only, pattern)| SwitchIdentitySignature {
                    vendor,
                    tail_only,
                    regex: regex::Regex::new(pattern).expect("valid switch identity signature"),
                })
                .collect::<Vec<_>>()
                .into()
        });
    SIGNATURES.clone()
}

/// Whether a signature may match a line `depth_from_bottom` lines above the end
/// of the screen: prompt and login-banner forms are tail-only, vendor output is
/// matched across the whole visible screen.
fn signature_in_region(signature: &SwitchIdentitySignature, depth_from_bottom: usize) -> bool {
    !signature.tail_only || depth_from_bottom < SWITCH_SIGNATURE_TAIL_LINES
}

fn switch_signature_matches(signature: &SwitchIdentitySignature, screen: &str) -> bool {
    screen.lines().rev().enumerate().any(|(depth, line)| {
        signature_in_region(signature, depth)
            && signature.regex.is_match(line.trim_end_matches('\r'))
    })
}

/// True when the visible screen shows a network-switch session. Used to
/// identify a pane (whose foreground process is an SSH/telnet client) as a
/// `switch` agent; the switch screen manifest then classifies its state.
pub fn recognize_switch_session(screen: &str) -> bool {
    switch_signatures()
        .iter()
        .any(|signature| switch_signature_matches(signature, screen))
}

/// Vendor proven by the visible screen, when a vendor-specific signature
/// matched. The bottom-most eligible match wins: the line closest to the prompt
/// belongs to the session the operator is looking at, so stale or pasted device
/// output further up the screen cannot override it. Prompt signatures honour the
/// same tail boundary as switch identification, so an old prompt above the
/// window cannot claim a vendor either. `None` means the session looks like a
/// device CLI whose vendor is not identifiable (for example a bare
/// `host(config)#` prompt).
pub fn switch_session_vendor(screen: &str) -> Option<SwitchVendor> {
    screen
        .lines()
        .rev()
        .enumerate()
        .filter_map(|(depth, line)| {
            let line = line.trim_end_matches('\r');
            switch_signatures()
                .iter()
                .find(|signature| {
                    signature.vendor.is_some()
                        && signature_in_region(signature, depth)
                        && signature.regex.is_match(line)
                })
                .and_then(|signature| signature.vendor)
        })
        .next()
}

fn process_priority(process: &crate::platform::ForegroundProcess, normalized_name: &str) -> u8 {
    let lower_name = normalized_name.to_lowercase();
    if lower_name != process.name.to_lowercase() {
        return 3;
    }
    if !is_generic_runtime_or_shell(&lower_name) {
        return 2;
    }
    1
}

fn is_generic_runtime_or_shell(name: &str) -> bool {
    let name = normalized_agent_lookup_name(path_basename(name));
    is_python_runtime(&name)
        || matches!(
            name.as_str(),
            "sh" | "bash"
                | "zsh"
                | "fish"
                | "tmux"
                | "node"
                | "bun"
                | "cmd"
                | "powershell"
                | "pwsh"
        )
}

fn is_python_runtime(name: &str) -> bool {
    name == "python"
        || name.strip_prefix("python").is_some_and(|version| {
            !version.is_empty()
                && version
                    .split('.')
                    .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
        })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn foreground_process(
        pid: u32,
        name: &str,
        argv: &[&str],
    ) -> crate::platform::ForegroundProcess {
        crate::platform::ForegroundProcess {
            pid,
            name: name.to_string(),
            argv0: None,
            argv: Some(argv.iter().map(|arg| (*arg).to_string()).collect()),
            cmdline: Some(argv.join(" ")),
        }
    }

    #[cfg(unix)]
    fn temp_detection_path(name: &str) -> std::path::PathBuf {
        let unique = format!(
            "herdr-detect-tests-{}-{}-{}",
            name,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

    // ---- Agent identification ----

    #[test]
    fn identify_known_agents() {
        assert_eq!(identify_agent("pi"), Some(Agent::Pi));
        assert_eq!(identify_agent("claude"), Some(Agent::Claude));
        assert_eq!(identify_agent("claude-code"), Some(Agent::Claude));
        assert_eq!(identify_agent("codex"), Some(Agent::Codex));
        assert_eq!(identify_agent("gemini"), Some(Agent::Gemini));
        assert_eq!(identify_agent("cursor"), Some(Agent::Cursor));
        assert_eq!(identify_agent("cursor-agent"), Some(Agent::Cursor));
        assert_eq!(identify_agent("devin"), Some(Agent::Devin));
        assert_eq!(identify_agent("devin-cli"), Some(Agent::Devin));
        assert_eq!(identify_agent("agy"), Some(Agent::Antigravity));
        assert_eq!(identify_agent("antigravity-cli"), Some(Agent::Antigravity));
        assert_eq!(identify_agent("cline"), Some(Agent::Cline));
        assert_eq!(identify_agent("omp"), Some(Agent::Omp));
        assert_eq!(identify_agent("mastracode"), Some(Agent::Mastracode));
        assert_eq!(identify_agent("mastra-code"), Some(Agent::Mastracode));
        assert_eq!(identify_agent("opencode"), Some(Agent::OpenCode));
        assert_eq!(identify_agent("opencode.exe"), Some(Agent::OpenCode));
        assert_eq!(identify_agent("opencode2"), Some(Agent::OpenCode));
        assert_eq!(identify_agent("opencode2.exe"), Some(Agent::OpenCode));
        assert_eq!(identify_agent("kimi"), Some(Agent::Kimi));
        assert_eq!(identify_agent("Kimi Code"), Some(Agent::Kimi));
        assert_eq!(identify_agent("kiro"), Some(Agent::Kiro));
        assert_eq!(identify_agent("kiro-cli"), Some(Agent::Kiro));
        assert_eq!(identify_agent("copilot"), Some(Agent::GithubCopilot));
        assert_eq!(identify_agent("ghcs"), Some(Agent::GithubCopilot));
        assert_eq!(identify_agent("grok"), Some(Agent::Grok));
        assert_eq!(identify_agent("grok-build"), Some(Agent::Grok));
        assert_eq!(identify_agent("hermes"), Some(Agent::Hermes));
        assert_eq!(identify_agent("hermes-agent"), Some(Agent::Hermes));
        assert_eq!(identify_agent("kilo"), Some(Agent::Kilo));
        assert_eq!(identify_agent("kilo-code"), Some(Agent::Kilo));
        assert_eq!(identify_agent("qwen"), Some(Agent::Qwen));
        assert_eq!(identify_agent("Qwen Code"), Some(Agent::Qwen));
        assert_eq!(identify_agent("letta"), Some(Agent::Letta));
        assert_eq!(identify_agent("Letta Code"), Some(Agent::Letta));
        assert_eq!(identify_agent("maki"), Some(Agent::Maki));
        assert_eq!(identify_agent("muse"), Some(Agent::Muse));
        assert_eq!(identify_agent("muse-code"), Some(Agent::Muse));
        assert_eq!(identify_agent("muse-cli"), Some(Agent::Muse));
        assert_eq!(identify_agent("muse-bin-0.1.0-R708.1"), Some(Agent::Muse));
        assert_eq!(identify_agent("muse-bin-1.2.3"), Some(Agent::Muse));
        assert_eq!(
            identify_agent("/home/user/.local/bin/muse-bin-0.2.1-R1215.1"),
            Some(Agent::Muse)
        );
        assert_eq!(
            identify_agent(r"C:\Users\user\muse-bin-0.2.1-R1215.1.exe"),
            Some(Agent::Muse)
        );
    }

    #[test]
    fn parse_known_agent_labels() {
        assert_eq!(parse_agent_label("pi"), Some(Agent::Pi));
        assert_eq!(parse_agent_label("claude"), Some(Agent::Claude));
        assert_eq!(parse_agent_label("cursor-agent"), Some(Agent::Cursor));
        assert_eq!(parse_agent_label("devin-cli"), Some(Agent::Devin));
        assert_eq!(parse_agent_label("agy"), Some(Agent::Antigravity));
        assert_eq!(parse_agent_label("antigravity"), Some(Agent::Antigravity));
        assert_eq!(parse_agent_label("omp"), Some(Agent::Omp));
        assert_eq!(parse_agent_label("mastracode"), Some(Agent::Mastracode));
        assert_eq!(parse_agent_label("mastra code"), Some(Agent::Mastracode));
        assert_eq!(parse_agent_label("opencode.exe"), Some(Agent::OpenCode));
        assert_eq!(parse_agent_label("copilot"), Some(Agent::GithubCopilot));
        assert_eq!(parse_agent_label("kimi-code"), Some(Agent::Kimi));
        assert_eq!(
            parse_agent_label("github-copilot"),
            Some(Agent::GithubCopilot)
        );
        assert_eq!(parse_agent_label("amp-local"), Some(Agent::Amp));
        assert_eq!(parse_agent_label("kiro-cli"), Some(Agent::Kiro));
        assert_eq!(parse_agent_label("grok-build"), Some(Agent::Grok));
        assert_eq!(parse_agent_label("hermes-agent"), Some(Agent::Hermes));
        assert_eq!(parse_agent_label("qwen-code"), Some(Agent::Qwen));
        assert_eq!(parse_agent_label("letta-code"), Some(Agent::Letta));
        assert_eq!(parse_agent_label("maki"), Some(Agent::Maki));
        assert_eq!(parse_agent_label("kilo-code"), Some(Agent::Kilo));
        assert_eq!(parse_agent_label("quads-client"), Some(Agent::Quads));
        assert_eq!(parse_agent_label("quads"), Some(Agent::Quads));
        assert_eq!(parse_agent_label("switch"), Some(Agent::Switch));
        assert_eq!(parse_agent_label("network-device"), Some(Agent::Switch));
    }

    #[test]
    fn every_agent_label_round_trips_through_canonical_and_alias_parsers() {
        for agent in Agent::ALL {
            let label = agent_label(agent);
            assert_eq!(parse_canonical_agent_label(label), Some(agent));
            assert_eq!(parse_agent_label(label), Some(agent));
        }
    }

    #[test]
    fn every_agent_has_a_canonical_interactive_executable() {
        let expected = [
            (Agent::Pi, "pi"),
            (Agent::Claude, "claude"),
            (Agent::Codex, "codex"),
            (Agent::Gemini, "gemini"),
            (
                Agent::Cursor,
                if cfg!(windows) {
                    "cursor-agent.cmd"
                } else {
                    "cursor-agent"
                },
            ),
            (Agent::Devin, "devin"),
            (Agent::Antigravity, "agy"),
            (Agent::Cline, "cline"),
            (Agent::Omp, "omp"),
            (Agent::Mastracode, "mastracode"),
            (Agent::OpenCode, "opencode"),
            (Agent::GithubCopilot, "copilot"),
            (Agent::Kimi, "kimi"),
            (Agent::Kiro, "kiro-cli"),
            (Agent::Droid, "droid"),
            (Agent::Amp, "amp"),
            (Agent::Grok, "grok"),
            (Agent::Hermes, "hermes"),
            (Agent::Kilo, "kilo"),
            (Agent::Qodercli, "qodercli"),
            (Agent::Qwen, "qwen"),
            (Agent::Letta, "letta"),
            (Agent::Maki, "maki"),
            (Agent::Muse, "muse"),
            (Agent::Quads, "quads-client"),
            (Agent::Switch, "ssh"),
        ];
        assert_eq!(expected.len(), Agent::ALL.len());
        for (agent, executable) in expected {
            assert_eq!(interactive_agent_executable(agent), executable);
        }
    }

    #[test]
    fn canonical_agent_labels_are_strict() {
        assert_eq!(parse_canonical_agent_label("claude-code"), None);
        assert_eq!(parse_canonical_agent_label("Pi"), None);
        assert_eq!(parse_canonical_agent_label(" pi "), None);
        assert_eq!(parse_canonical_agent_label("opencode.exe"), None);
    }

    #[test]
    fn mastracode_is_hook_authority_without_screen_manifest() {
        assert!(full_lifecycle_hook_authority(
            "herdr:mastracode",
            "mastracode"
        ));
        assert!(!Agent::SCREEN_MANIFEST_AGENTS.contains(&Agent::Mastracode));
    }

    #[test]
    fn session_identity_integrations_leave_state_to_screen_detection() {
        for (source, label, agent) in [
            ("herdr:hermes", "hermes", Agent::Hermes),
            ("herdr:qwen", "qwen", Agent::Qwen),
            ("herdr:letta", "letta", Agent::Letta),
            ("herdr:antigravity_cli", "agy", Agent::Antigravity),
        ] {
            assert!(!full_lifecycle_hook_authority(source, label));
            assert!(session_identity_only_integration(source, label));
            assert!(Agent::SCREEN_MANIFEST_AGENTS.contains(&agent));
        }
    }

    #[test]
    fn identify_unknown_processes() {
        assert_eq!(identify_agent("bash"), None);
        assert_eq!(identify_agent("zsh"), None);
        assert_eq!(identify_agent("vim"), None);
        assert_eq!(identify_agent("node"), None);
        assert_eq!(identify_agent("museum"), None);
        assert_eq!(identify_agent("muse-helper"), None);
        assert_eq!(identify_agent("muser"), None);
        assert_eq!(identify_agent("musescore"), None);
        assert_eq!(identify_agent("muse-bin"), None);
        assert_eq!(identify_agent("muse-bin-"), None);
        assert_eq!(identify_agent("muse-binary"), None);
    }

    #[test]
    fn network_client_process_names_are_recognized() {
        assert!(is_network_client_process_name("ssh"));
        assert!(is_network_client_process_name("/usr/bin/ssh"));
        assert!(is_network_client_process_name("ssh.exe"));
        assert!(is_network_client_process_name("telnet"));
        assert!(is_network_client_process_name("slogin"));
        assert!(!is_network_client_process_name("bash"));
        assert!(!is_network_client_process_name("zsh"));
        assert!(!is_network_client_process_name("scp"));
        assert!(!is_network_client_process_name("quads-client"));
    }

    #[test]
    fn recognizes_switch_sessions_from_prompt_and_banner() {
        for screen in [
            // Juniper JunOS EVO login banner.
            "Last login: Thu Oct  8 14:29:57 2026\n--- JUNOS 23.4R2-S6.9-EVO Linux (none) 5.2.60-yocto-standard-g644b307 #1 SMP PREEMPT x86_64 GNU/Linux",
            // Juniper operational prompt.
            "admin@switch1> ",
            // Juniper config breadcrumb and prompt.
            "[edit protocols ospf area 0.0.0.0]\nadmin@switch1#",
            // Arista / Cisco / EOS config prompt (strong identity signal).
            "core-switch(config-if-Et1)# ",
            // Arista EOS `show version` model and software lines.
            "Arista DCS-7280SRA-48C6-F\nHardware version: 21.00\nSoftware image version: 4.26.4M",
            "Arista vEOS\nSoftware image version: 4.22.1FX-VEOSRouter-cloud",
            // Arista EOS bash entry banner (documented EOS shell).
            "localhost#bash\n\nArista Networks EOS shell\n\n[admin@localhost ~]$",
            // Arista EOS CLI error grammar (eAPI and CLI forms).
            "% Invalid input (at token 1: 'show foo')",
            "Invalid input (at token 2: 'trident')",
            "% Invalid input (privileged mode required)",
            // EOS vendor output above the prompt window: identity scans the
            // whole screen for it, because show version output scrolls past
            // the eight-line prompt tail in one frame.
            "Arista DCS-7050TX-64-R\nSoftware image version: 4.28.3M\na\nb\nc\nd\ne\nf\ng\nh\ni\ncore-sw1#",
            // Prompts padded with extra trailing spaces.
            "admin@switch1>  ",
            "admin@switch1#  ",
        ] {
            assert!(
                recognize_switch_session(screen),
                "expected switch session for {screen:?}"
            );
        }
    }

    #[test]
    fn does_not_recognize_plain_shell_prompts_as_switch_sessions() {
        for screen in [
            "user@workstation:~$ ",
            "root@workstation:/var/log# ",
            "bash-5.2$ ",
            "hostname# ",
            "hostname#   ",
            "user@host:~# ",
            "",
            "just some output line\nmore output",
            // Arista EOS over SSH shows only this at login, exactly like sshd:
            // it must stay unclassified until EOS-specific output appears.
            "Last login: Sat Feb  2 14:45:16 2019 from 192.0.2.1\narista1# ",
            // The EOS bash prompt is a plain Linux shell prompt.
            "[admin@localhost ~]$ ",
        ] {
            assert!(
                !recognize_switch_session(screen),
                "unexpected switch session for {screen:?}"
            );
        }
    }

    #[test]
    fn bundled_manifest_identity_matches_the_compiled_fallback() {
        let signatures = switch_signatures();
        assert!(
            !signatures.is_empty(),
            "the bundled switch manifest must declare identity signatures"
        );
        let keys = |set: &[SwitchIdentitySignature]| {
            let mut keys = set
                .iter()
                .map(|signature| {
                    (
                        signature.vendor.map(SwitchVendor::label),
                        signature.tail_only,
                        signature.regex.as_str().to_string(),
                    )
                })
                .collect::<Vec<_>>();
            keys.sort();
            keys
        };
        assert_eq!(
            keys(&signatures),
            keys(&fallback_switch_identity_signatures()),
            "the manifest identity table and the compiled fallback must stay in sync"
        );
    }

    #[test]
    fn identifies_switch_vendor_from_signatures() {
        assert_eq!(
            switch_session_vendor("admin@switch1> "),
            Some(SwitchVendor::Juniper)
        );
        assert_eq!(
            switch_session_vendor("--- JUNOS 23.4R2-S6.9-EVO Linux (none)\nadmin@switch1> "),
            Some(SwitchVendor::Juniper)
        );
        assert_eq!(
            switch_session_vendor("Arista DCS-7280SRA-48C6-F"),
            Some(SwitchVendor::Arista)
        );
        // Legacy EOS headers name the vendor instead of the DCS prefix.
        assert!(recognize_switch_session("Arista Networks 7050TX-64"));
        assert_eq!(
            switch_session_vendor("Arista Networks 7050TX-64"),
            Some(SwitchVendor::Arista)
        );
        assert_eq!(
            switch_session_vendor("Arista Networks EOS shell"),
            Some(SwitchVendor::Arista)
        );
        // The software version line is shared by several network OS families:
        // it identifies a device CLI but not a vendor.
        assert!(recognize_switch_session("Software image version: 4.26.4M"));
        assert_eq!(
            switch_session_vendor("Software image version: 4.26.4M"),
            None
        );
        assert_eq!(
            switch_session_vendor("% Invalid input (at token 1: 'show foo')"),
            Some(SwitchVendor::Arista)
        );
        assert_eq!(
            switch_session_vendor(
                "Cisco IOS Software, C2960 Software (C2960-LANBASEK9-M), Version 15.0(2)SE11"
            ),
            Some(SwitchVendor::Cisco)
        );
        assert_eq!(
            switch_session_vendor("Cisco IOS XE Software, Version 17.03.04"),
            Some(SwitchVendor::Cisco)
        );
        // A shared device prompt identifies a switch but not a vendor, so the
        // pane keeps the plain `switch` label.
        assert!(recognize_switch_session("core-switch(config)# "));
        assert_eq!(switch_session_vendor("core-switch(config)# "), None);
        // The bottom-most vendor signal wins, so pasted or stale output above
        // the current prompt cannot override the session's own device.
        assert_eq!(
            switch_session_vendor("Cisco IOS Software, C2960\nadmin@switch1> "),
            Some(SwitchVendor::Juniper)
        );
        assert_eq!(
            switch_session_vendor("admin@switch1> \nCisco IOS Software, C2960"),
            Some(SwitchVendor::Cisco)
        );
        // A vendor prompt inside the identity window claims that vendor.
        assert_eq!(
            switch_session_vendor("admin@switch1> \ncore-sw(config)# "),
            Some(SwitchVendor::Juniper)
        );
        // A stale prompt above the identity window does not: the tail boundary
        // applies to vendor selection exactly as it does to identification.
        let stale_prompt = format!("admin@switch1> \n{}core-sw(config)# ", "filler\n".repeat(8));
        assert_eq!(switch_session_vendor(&stale_prompt), None);
        // Linux prompts prove neither.
        assert_eq!(switch_session_vendor("user@workstation:~$ "), None);
    }

    #[test]
    fn switch_vendor_labels_are_display_labels() {
        assert_eq!(SwitchVendor::Juniper.label(), "sw-juniper");
        assert_eq!(SwitchVendor::Arista.label(), "sw-arista");
        assert_eq!(SwitchVendor::Cisco.label(), "sw-cisco");
    }

    #[test]
    fn network_client_destination_reads_the_first_bare_token() {
        let argv = |parts: &[&str]| {
            parts
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            network_client_destination(&argv(&["ssh", "core-sw1"])).as_deref(),
            Some("core-sw1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "admin@core-sw1.dc1.example.com"]))
                .as_deref(),
            Some("core-sw1.dc1.example.com")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-p", "2222", "leaf1.example.com"]))
                .as_deref(),
            Some("leaf1.example.com")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-p2222", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-vp", "2222", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-vp2222", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-l", "admin", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-o", "StrictHostKeyChecking=no", "leaf1"]))
                .as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-oStrictHostKeyChecking=no", "leaf1"]))
                .as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-J", "bastion", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-B", "eth0", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-P", "deploy", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-W", "leaf1:22", "bastion"])).as_deref(),
            Some("bastion")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "ssh://admin@leaf1:2222"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "[2001:db8::1]:22"])).as_deref(),
            Some("2001:db8::1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "2001:db8::1"])).as_deref(),
            Some("2001:db8::1")
        );
        assert_eq!(
            network_client_destination(&argv(&["ssh", "--", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["telnet", "leaf1", "23"])).as_deref(),
            Some("leaf1")
        );
        // telnet has its own option table: `-E` is a flag there, `-X` and `-l`
        // take values.
        assert_eq!(
            network_client_destination(&argv(&["telnet", "-E", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["telnet", "-X", "KRB5", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["telnet", "-l", "admin", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            network_client_destination(&argv(&["telnet", "-8", "leaf1"])).as_deref(),
            Some("leaf1")
        );
        // No destination at all: the session is stdin or just flags.
        assert_eq!(network_client_destination(&argv(&["ssh"])), None);
        assert_eq!(network_client_destination(&argv(&["ssh", "-v"])), None);
        assert_eq!(
            network_client_destination(&argv(&["ssh", "-p", "22"])),
            None
        );
    }

    fn host_patterns(patterns: &[&str]) -> Vec<SwitchHostPattern> {
        patterns
            .iter()
            .filter_map(|pattern| SwitchHostPattern::compile(pattern))
            .collect()
    }

    #[test]
    fn switch_host_patterns_match_hosts_and_short_names() {
        let patterns = host_patterns(&["core-sw1", "leaf*.dc1.example.com"]);
        assert_eq!(
            switch_host_title_with(&patterns, "core-sw1").as_deref(),
            Some("core-sw1")
        );
        assert_eq!(
            switch_host_title_with(&patterns, "user@core-sw1").as_deref(),
            Some("core-sw1")
        );
        assert_eq!(
            switch_host_title_with(&patterns, "CORE-SW1.dc1.example.com").as_deref(),
            Some("core-sw1")
        );
        assert_eq!(
            switch_host_title_with(&patterns, "leaf1.dc1.example.com").as_deref(),
            Some("leaf1")
        );
        assert_eq!(
            switch_host_title_with(&patterns, "ssh://admin@leaf2.dc1.example.com:22").as_deref(),
            Some("leaf2")
        );
        assert_eq!(
            switch_host_title_with(&patterns, "leaf3.dc2.example.com"),
            None
        );
        assert_eq!(switch_host_title_with(&patterns, "core-sw2"), None);
        assert_eq!(switch_host_title_with(&patterns, ""), None);

        // IP literals match as written and keep their full form as the title.
        let addresses = host_patterns(&["10.0.0.5", "2001:db8::1"]);
        assert_eq!(
            switch_host_title_with(&addresses, "10.0.0.5").as_deref(),
            Some("10.0.0.5")
        );
        assert_eq!(
            switch_host_title_with(&addresses, "2001:db8::1").as_deref(),
            Some("2001:db8::1")
        );

        // Blank and whitespace-only entries are ignored.
        assert!(host_patterns(&["", "  "]).is_empty());
    }

    #[test]
    fn switch_host_pattern_globs_are_anchored() {
        let patterns = host_patterns(&["sw-?", "*.dc1.example.com"]);
        assert_eq!(
            switch_host_title_with(&patterns, "sw-1").as_deref(),
            Some("sw-1")
        );
        assert_eq!(switch_host_title_with(&patterns, "sw-12"), None);
        assert_eq!(
            switch_host_title_with(&patterns, "core-sw1.dc1.example.com").as_deref(),
            Some("core-sw1")
        );
        // The glob is anchored to the whole host, and the short-name angle does
        // not turn a suffix pattern into a prefix match.
        assert_eq!(switch_host_title_with(&patterns, "dc1.example.com"), None);
    }

    #[test]
    fn identify_case_insensitive() {
        assert_eq!(identify_agent("Pi"), Some(Agent::Pi));
        assert_eq!(identify_agent("CLAUDE"), Some(Agent::Claude));
        assert_eq!(identify_agent("Codex"), Some(Agent::Codex));
        assert_eq!(identify_agent("Devin"), Some(Agent::Devin));
    }

    #[test]
    fn identify_agent_in_job_prefers_wrapped_codex() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![
                foreground_process(1, "node", &["node", "/path/to/bin/codex"]),
                foreground_process(2, "bash", &["bash"]),
            ],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Codex, "codex".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_node_wrapped_qwen() {
        for argv in [
            vec!["node", "/home/user/.fnm/bin/qwen"],
            vec![
                "node.exe",
                r"C:\Users\user\AppData\Roaming\npm\node_modules\@qwen-code\qwen-code\dist\index.js",
            ],
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, "MainThread", &argv)],
            };

            assert_eq!(
                identify_agent_in_job(&job),
                Some((Agent::Qwen, "qwen".to_string()))
            );
        }
    }

    #[test]
    fn identify_agent_in_job_detects_cline_native_binaries() {
        for (name, executable) in [
            (
                ".cline",
                "/home/user/.npm/lib/node_modules/cline/bin/.cline",
            ),
            (
                "cline",
                "/usr/local/lib/node_modules/@cline/cli-darwin-arm64/bin/cline",
            ),
            (
                "cline.exe",
                r"C:\Users\user\AppData\Roaming\npm\node_modules\@cline\cli-windows-x64\bin\cline.exe",
            ),
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, name, &[executable, "--tui"])],
            };

            assert_eq!(
                identify_agent_in_job(&job),
                Some((Agent::Cline, name.to_string()))
            );
        }
    }

    #[test]
    fn identify_agent_in_job_detects_cline_node_wrapper() {
        for (name, argv) in [
            (
                "MainThread",
                vec!["node", "/home/user/.fnm/bin/cline", "--tui"],
            ),
            (
                "node",
                vec!["node", "/usr/local/lib/node_modules/cline/bin/cline"],
            ),
            (
                "node.exe",
                vec![
                    r"C:\Program Files\nodejs\node.exe",
                    r"C:\Users\user\AppData\Roaming\npm\node_modules\cline\bin\cline",
                ],
            ),
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, name, &argv)],
            };

            assert_eq!(
                identify_agent_in_job(&job),
                Some((Agent::Cline, "cline".to_string()))
            );
        }
    }

    #[test]
    fn identify_agent_in_job_rejects_unrelated_cline_mentions() {
        for argv in [
            vec!["node"],
            vec!["node", "/path/to/other.js", "cline"],
            vec!["node", "-e", "cline"],
            vec!["node", "/path/to/cline-helper"],
            vec!["/path/to/.cline-helper"],
            vec!["/path/to/other", "/path/to/cline"],
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, "MainThread", &argv)],
            };

            assert_eq!(identify_agent_in_job(&job), None);
        }
        assert_eq!(identify_agent("MainThread"), None);
    }

    #[test]
    fn identify_agent_in_job_detects_interactive_letta_entrypoints() {
        for argv in [
            vec!["letta", "--backend", "local"],
            vec![
                "node",
                "/home/user/project/node_modules/.bin/letta",
                "--conversation",
                "conversation-id",
            ],
            vec![
                "node.exe",
                r"C:\Users\user\AppData\Roaming\npm\node_modules\@letta-ai\letta-code\letta.js",
                "--agent",
                "agent-id",
            ],
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, "MainThread", &argv)],
            };

            assert_eq!(
                identify_agent_in_job(&job),
                Some((Agent::Letta, "letta".to_string()))
            );
        }
    }

    #[test]
    fn identify_agent_in_job_ignores_noninteractive_letta_processes() {
        for args in [
            vec!["--prompt", "hello"],
            vec!["--output-format", "json"],
            vec!["--input-format=stream-json"],
            vec!["--ephemeral"],
            vec!["--max-turns=1"],
            vec!["server"],
            vec!["--backend", "local", "server"],
            vec!["fix this bug"],
            vec!["agents", "list"],
            vec!["version"],
        ] {
            let mut argv = vec!["node", "/home/user/project/node_modules/.bin/letta"];
            argv.extend(args);
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, "MainThread", &argv)],
            };

            assert_eq!(identify_agent_in_job(&job), None, "argv: {argv:?}");
        }

        let unrelated = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node",
                &["node", "/tmp/server.js", "letta"],
            )],
        };
        assert_eq!(identify_agent_in_job(&unrelated), None);

        let source_checkout = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node",
                &["node", "/home/user/src/letta-code/letta/build.js"],
            )],
        };
        assert_eq!(identify_agent_in_job(&source_checkout), None);
    }

    #[test]
    fn identify_agent_in_job_detects_windows_cursor_install() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node.exe",
                &[
                    r"C:\Users\user\AppData\Local\cursor-agent\versions\2026.08.11-e8db854\node.exe",
                    r"C:\Users\user\AppData\Local\cursor-agent\versions\2026.08.11-e8db854\index.js",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Cursor, "cursor".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_ignores_invalid_windows_cursor_install_paths() {
        for script in [
            r"C:\Users\user\AppData\Local\cursor-agent\versions\2026.08.11-e8db854\scripts\postinstall.js",
            r"C:\Users\user\AppData\Local\cursor-agent\versions\2026.08.11-e8db854\index",
            r"C:\Users\user\AppData\Local\cursor-agent\versions\2026.08.11-e8db854\index.exe",
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(
                    123,
                    "node.exe",
                    &[
                        r"C:\Users\user\AppData\Local\cursor-agent\versions\2026.08.11-e8db854\node.exe",
                        script,
                    ],
                )],
            };

            assert_eq!(identify_agent_in_job(&job), None, "script: {script}");
        }

        let lookalike = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node.exe",
                &[
                    r"C:\Program Files\nodejs\node.exe",
                    r"C:\workspace\cursor-agent\versions\test\index.js",
                ],
            )],
        };
        assert_eq!(identify_agent_in_job(&lookalike), None);
    }

    #[test]
    fn identify_agent_in_job_prefers_recognized_process_group_leader() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 42,
            processes: vec![
                foreground_process(42, "claude", &["claude"]),
                foreground_process(43, "node", &["node", "/tmp/mcp/bin/codex"]),
            ],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Claude, "claude".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_falls_back_when_process_group_leader_is_unrecognized() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 42,
            processes: vec![
                foreground_process(42, "bash", &["bash"]),
                foreground_process(43, "node", &["node", "/tmp/mcp/bin/codex"]),
            ],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Codex, "codex".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_python_version_wrapped_hermes() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "python3.12",
                &[
                    "/nix/store/example/bin/python3.12",
                    "/nix/store/example/bin/hermes",
                    "--resume",
                    "session-id",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Hermes, "hermes".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_nix_wrapped_codex_from_cmdline_argv0() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                ".codex-wrapped",
                &["/etc/profiles/per-user/user/bin/codex", "--model", "gpt-5"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Codex, "codex".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_canonicalizes_nix_wrapped_aliases_from_cmdline_argv0() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                ".claude-code-wrapped",
                &["/nix/store/example/bin/claude-code"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Claude, "claude".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_shell_wrapped_pi() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "sh",
                &["/bin/sh", "/tmp/test-bin/pi"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Pi, "pi".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_bun_wrapped_omp() {
        for (runtime, script) in [
            ("bun", "/home/can/.bun/bin/omp"),
            (
                "bun.exe",
                r"C:\Users\herdr\AppData\Roaming\npm\node_modules\@oh-my-pi\pi-coding-agent\dist\cli.js",
            ),
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, runtime, &[runtime, script])],
            };
            assert_eq!(
                identify_agent_in_job(&job),
                Some((Agent::Omp, "omp".to_string())),
                "script: {script}"
            );
        }

        let other_script = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "bun.exe",
                &[
                    "bun.exe",
                    r"C:\Users\herdr\AppData\Roaming\npm\node_modules\@oh-my-pi\pi-coding-agent\dist\setup.js",
                ],
            )],
        };
        assert_eq!(identify_agent_in_job(&other_script), None);
    }

    #[test]
    fn identify_agent_in_job_detects_node_wrapped_pi_package_cli() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node.exe",
                &[
                    "node.exe",
                    "C:\\Users\\herdr\\AppData\\Roaming\\npm\\node_modules\\@earendil-works\\pi-coding-agent\\dist\\cli.js",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Pi, "pi".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_node_wrapped_pi_bundled_cli() {
        // Hardened runtimes deny `PROCESS_VM_READ`, so the command line can come
        // from a LimitedInformation query with the launcher path intact. The
        // detection path must not depend on how that command line was obtained.
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node.exe",
                &[
                    r"C:\Users\herdr\AppData\Local\pi-node\current\node.exe",
                    r"C:\Users\herdr\AppData\Local\pi-node\current/node_modules/@earendil-works/pi-coding-agent/dist/bundle/cli.js",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Pi, "pi".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_node_wrapped_mastracode_package_cli() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node.exe",
                &[
                    "node.exe",
                    "C:\\Users\\herdr\\AppData\\Roaming\\npm\\node_modules\\mastracode\\dist\\cli.js",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Mastracode, "mastracode".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_node_wrapped_kimi_package_cli() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "node.exe",
                &[
                    r"C:\Program Files\nodejs\node.exe",
                    r"C:\repro-3317-kimi-prefix\node_modules\@moonshot-ai\kimi-code\dist\main.mjs",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Kimi, "kimi".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_ignores_non_cli_pi_package_scripts() {
        for script in [
            r"C:\Users\herdr\AppData\Roaming\npm\node_modules\@earendil-works\pi-coding-agent\scripts\build.js",
            r"C:\Users\herdr\AppData\Local\pi-node\current\node_modules\@earendil-works\pi-coding-agent\dist\bundle\update.js",
            r"C:\workspace\dist\bundle\cli.js",
            r"C:\workspace\node_modules\other-package\dist\bundle\cli.js",
            r"C:\workspace\node_modules\@earendil-works\pi-coding-agent\dist\cli.exe",
            r"C:\workspace\node_modules\@earendil-works\pi-coding-agent\dist\cli.js\other.js",
            r"C:\workspace\node_modules\@earendil-works\pi-coding-agent\dist\bundle\cli.exe",
            r"C:\workspace\node_modules\@earendil-works\pi-coding-agent\dist\bundle\cli.js\other.js",
        ] {
            let job = crate::platform::ForegroundJob {
                process_group_id: 123,
                processes: vec![foreground_process(123, "node.exe", &["node.exe", script])],
            };

            assert_eq!(identify_agent_in_job(&job), None, "script: {script}");
        }
    }

    #[test]
    fn identify_agent_in_job_detects_windows_cmd_wrapped_codex() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "cmd.exe",
                &[
                    "cmd.exe",
                    "/D",
                    "/S",
                    "/C",
                    "C:\\Users\\herdr\\AppData\\Roaming\\npm\\codex.cmd --model gpt-5",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Codex, "codex".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_powershell_file_wrapped_claude() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "powershell.exe",
                &[
                    "powershell.exe",
                    "-NoProfile",
                    "-File",
                    "C:\\Users\\herdr\\Documents\\PowerShell\\Scripts\\claude.ps1",
                ],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Claude, "claude".to_string()))
        );
    }

    // A plain shell pane launched with herdr's injected prompt integration
    // must still classify as a shell, not an agent, even though its argv now
    // carries a -Command payload.
    #[test]
    fn identify_agent_in_job_ignores_herdr_powershell_shell_integration_argv() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "powershell.exe",
                &[
                    "powershell.exe",
                    "-NoExit",
                    "-Command",
                    crate::pane::WINDOWS_POWERSHELL_SHELL_INTEGRATION_COMMAND,
                ],
            )],
        };

        assert_eq!(identify_agent_in_job(&job), None);
    }

    #[test]
    fn identify_agent_in_job_detects_opencode2_as_opencode() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "opencode2",
                &["opencode2", "--standalone"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::OpenCode, "opencode2".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_opencode_exe_from_pnpm_package() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "opencode.exe",
                &["/home/user/.local/share/pnpm/global/node_modules/opencode-ai/bin/opencode.exe"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::OpenCode, "opencode.exe".to_string()))
        );
    }

    #[test]
    fn identify_agent_in_job_detects_opencode_exe_from_argv0_path() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                123,
                "MainThread",
                &["/home/user/.local/share/pnpm/global/node_modules/opencode-ai/bin/opencode.exe"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::OpenCode, "opencode".to_string()))
        );
    }

    #[test]
    fn wrapped_agent_name_from_runtime_argv_ignores_plain_shell_flags() {
        assert_eq!(
            wrapped_agent_name_from_runtime_argv("bash", Some(&["bash".into(), "-lc".into()])),
            None
        );
    }

    #[test]
    fn identify_agent_in_job_ignores_python_c_argument_named_codex() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "python3",
                &["python3", "-c", "import time; time.sleep(60)", "/tmp/codex"],
            )],
        };

        assert_eq!(identify_agent_in_job(&job), None);
    }

    #[test]
    fn identify_agent_in_job_ignores_node_eval_argument_named_codex() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "node",
                &["node", "-e", "setTimeout(() => {}, 60000)", "/tmp/codex"],
            )],
        };

        assert_eq!(identify_agent_in_job(&job), None);
    }

    #[test]
    fn identify_agent_in_job_ignores_shell_c_argument_named_codex() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "bash",
                &["bash", "-c", "sleep 60", "/tmp/codex"],
            )],
        };

        assert_eq!(identify_agent_in_job(&job), None);
    }

    #[test]
    fn identify_agent_in_job_detects_python_script_named_codex() {
        let job = crate::platform::ForegroundJob {
            process_group_id: 123,
            processes: vec![foreground_process(
                1,
                "python3",
                &["python3", "/tmp/codex", "--model", "gpt-5"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Codex, "codex".to_string()))
        );
    }

    #[test]
    fn cmdline_argv0_agent_name_canonicalizes_known_aliases() {
        assert_eq!(
            cmdline_argv0_agent_name("/nix/store/example/bin/ghcs"),
            Some("copilot".to_string())
        );
    }

    #[test]
    fn cmdline_argv0_agent_name_requires_exact_agent_basename() {
        assert_eq!(cmdline_argv0_agent_name("/tmp/my-codex-helper"), None);
    }

    #[cfg(unix)]
    #[test]
    fn identify_agent_in_job_resolves_cursor_agent_symlink_argv0() {
        let dir = temp_detection_path("cursor-agent-symlink");
        std::fs::create_dir_all(&dir).expect("test directory should be created");
        let target = dir.join("cursor-agent");
        let link = dir.join("agent");
        std::fs::write(&target, b"#!/bin/sh\n").expect("target should be written");
        std::os::unix::fs::symlink(&target, &link).expect("symlink should be created");

        let argv0 = link.to_string_lossy().into_owned();
        let job = crate::platform::ForegroundJob {
            process_group_id: 42,
            processes: vec![foreground_process(
                42,
                "MainThread",
                &[&argv0, "--use-system-ca", "/tmp/index.js"],
            )],
        };

        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Cursor, "cursor".to_string()))
        );

        std::fs::remove_dir_all(&dir).ok();
    }

    // ---- Screen detection routing ----

    #[test]
    fn no_agent_returns_unknown() {
        assert_eq!(detect_state(None, "anything"), AgentState::Unknown);
    }

    // ---- Process identification (real PTY) ----

    #[cfg(target_os = "linux")]
    fn open_test_pty() -> portable_pty::PtyPair {
        portable_pty::native_pty_system()
            .openpty(portable_pty::PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("failed to open pty")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn foreground_job_detects_sleep() {
        use portable_pty::CommandBuilder;

        let pair = open_test_pty();

        // Spawn "sleep 999" — a known, deterministic process
        let mut cmd = CommandBuilder::new("sleep");
        cmd.arg("999");
        let mut child = pair.slave.spawn_command(cmd).expect("failed to spawn");
        let pid = child.process_id().expect("no pid");

        // Give the process a moment to become the foreground group
        std::thread::sleep(std::time::Duration::from_millis(50));

        let job = foreground_job(pid).expect("expected foreground job");
        assert!(
            job.processes.iter().any(|p| p.name == "sleep"),
            "expected sleep in {job:?}"
        );
        assert_eq!(
            identify_agent_in_job(&job),
            None,
            "sleep should not map to an agent"
        );

        // Clean up
        child.kill().ok();
        child.wait().ok();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn foreground_job_detects_shell_running_command() {
        use portable_pty::CommandBuilder;
        use std::io::Write;

        let pair = open_test_pty();

        // Spawn a shell, then run a command inside it
        let cmd = CommandBuilder::new("sh");
        let mut child = pair.slave.spawn_command(cmd).expect("failed to spawn");
        let pid = child.process_id().expect("no pid");

        // Write a command to the shell
        let mut writer = pair.master.take_writer().expect("no writer");
        // Use exec so sleep replaces sh as the foreground process
        writer.write_all(b"exec sleep 999\n").ok();
        drop(writer);

        std::thread::sleep(std::time::Duration::from_millis(100));

        let job = foreground_job(pid).expect("expected foreground job");
        assert!(
            job.processes.iter().any(|p| p.name == "sleep"),
            "expected sleep in {job:?}"
        );
        assert_eq!(
            identify_agent_in_job(&job),
            None,
            "sleep should not map to an agent"
        );

        child.kill().ok();
        child.wait().ok();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn foreground_job_detects_agent_behind_shell_wrapper() {
        use portable_pty::CommandBuilder;

        let pair = open_test_pty();

        let mut cmd = CommandBuilder::new("bash");
        cmd.arg("-c");
        cmd.arg("bash -c 'exec -a codex sleep 999' & wait");
        let mut child = pair.slave.spawn_command(cmd).expect("failed to spawn");
        let pid = child.process_id().expect("no pid");
        std::thread::sleep(std::time::Duration::from_millis(100));

        let job = foreground_job(pid);
        let process_group_id = job.as_ref().map(|job| job.process_group_id).unwrap_or(pid);
        unsafe {
            libc::kill(-(process_group_id as i32), libc::SIGKILL);
        }
        child.wait().ok();

        let job = job.expect("expected foreground job");
        assert!(
            job.processes.iter().any(|process| process.name == "bash")
                && job.processes.iter().any(|process| {
                    process.name == "sleep"
                        && process
                            .argv
                            .as_deref()
                            .and_then(|argv| argv.first())
                            .is_some_and(|argv0| argv0 == "codex")
                }),
            "expected wrapper and agent child in {job:?}"
        );
        assert_eq!(
            identify_agent_in_job(&job),
            Some((Agent::Codex, "codex".to_string()))
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn pane_shell_is_idle_tracks_foreground_command() {
        use portable_pty::CommandBuilder;
        use std::io::Write;

        fn wait_for(expected: bool, pid: u32) -> bool {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            while std::time::Instant::now() < deadline {
                if pane_shell_is_idle(pid) == expected {
                    return true;
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            false
        }

        let pair = open_test_pty();
        let mut cmd = CommandBuilder::new("sh");
        cmd.arg("-i");
        cmd.env("ENV", "/dev/null");
        let mut child = pair.slave.spawn_command(cmd).expect("failed to spawn");
        let pid = child.process_id().expect("no pid");
        let mut reader = pair.master.try_clone_reader().expect("no reader");
        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            while matches!(std::io::Read::read(&mut reader, &mut buf), Ok(n) if n > 0) {}
        });
        let mut writer = pair.master.take_writer().expect("no writer");

        assert!(wait_for(true, pid), "interactive shell should start idle");
        // End the foreground command with EOF rather than SIGINT. An interrupt
        // sent while the shell is still moving the command into its own
        // foreground process group can be delivered before the command resets
        // its signal dispositions, leaving it running in a stale foreground
        // group while the shell shows a prompt. `cat` blocks until stdin hits
        // EOF, so it stays reliably busy until we end it.
        writer.write_all(b"cat\n").unwrap();
        assert!(wait_for(false, pid), "shell running a command is not idle");
        writer.write_all(&[4]).unwrap();
        assert!(
            wait_for(true, pid),
            "shell should be idle after the command ends"
        );

        child.kill().ok();
        child.wait().ok();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn proc_stat_parsing_handles_spaces_in_comm() {
        // Verify our /proc/pid/stat parser correctly extracts fields
        // even when (comm) could contain spaces.
        let pid = std::process::id();
        let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).unwrap();

        // Our parsing: find last ')' then split the rest
        let close_paren = stat.rfind(')').expect("should have closing paren");
        let rest = &stat[close_paren + 2..];
        let fields: Vec<&str> = rest.split_whitespace().collect();

        // We should have enough fields (at least 6 for tpgid)
        assert!(
            fields.len() >= 6,
            "not enough fields in stat: {}",
            fields.len()
        );

        // Field 0 should be a valid state char (S, R, D, etc.)
        let state = fields[0];
        assert!(
            ["S", "R", "D", "Z", "T", "t", "W", "X", "I"].contains(&state),
            "unexpected state: {state}"
        );

        // Field 5 (tpgid) should parse as i32 (can be -1 if no controlling terminal)
        let tpgid: i32 = fields[5].parse().expect("tpgid should be a number");
        // In CI/test environments without a terminal, tpgid is typically -1
        let _ = tpgid;
    }
}
