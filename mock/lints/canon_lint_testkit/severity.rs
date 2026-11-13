//! The repository config's own say over a lint's severity, for the tests that ask it.

#![cfg(test)]

use mockspace::{Level, LintMode, Severity};

/// What `[lints.<lint>]` in a `mockspace.toml` text does to a lint's own severity.
///
/// The lint's `default_severity` is only the starting point. The config's
/// `commit`, `build` and `push` keys overlay it one gate at a time, so a lint
/// declared `HARD_ERROR` and keyed `push = "off"` in the repository's config
/// does not run at that gate while its own default still says it blocks. A test
/// asserting the default alone passes through exactly that.
///
/// What is read. The user layer: a `[lints.<lint>]` table with those three
/// keys, spelled as the engine spells a level. The presets a config may extend
/// and a `--severity-override` on the command line also apply in the engine and
/// are not read here, so a result from this is the repository's own config
/// layered over the default and nothing more.
pub fn severity_after_config(config: &str, lint: &str, default: Severity) -> Severity {
    let header = format!("[lints.{lint}]");
    let mut level = [
        default.effective(LintMode::Commit),
        default.effective(LintMode::Build),
        default.effective(LintMode::Push),
    ];
    let mut inside = false;
    for line in config.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == header;
            continue;
        }
        if !inside {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let slot = match key.trim() {
            "commit" => 0,
            "build" => 1,
            "push" => 2,
            _ => continue,
        };
        // A trailing comment after the value is not part of it.
        let value = value
            .split('#')
            .next()
            .unwrap_or(value)
            .trim()
            .trim_matches('"');
        if let Some(l) = Level::from_str_name(value) {
            level[slot] = l;
        }
    }
    Severity::new(level[0], level[1], level[2])
}
