use std::path::PathBuf;

const KEY_NAME: &str = "KLIPY_API_KEY";

/// Reads the KLIPY API key from the environment, falling back to
/// `$XDG_CONFIG_HOME/insertgif/env` (default `~/.config/insertgif/env`).
pub fn load_api_key() -> Result<String, String> {
    if let Ok(key) = std::env::var(KEY_NAME) {
        if !key.trim().is_empty() {
            return Ok(key.trim().to_string());
        }
    }

    let path = config_path().ok_or("could not determine config directory")?;
    let contents = std::fs::read_to_string(&path)
        .map_err(|e| format!("{KEY_NAME} not set and could not read {}: {e}", path.display()))?;

    contents
        .lines()
        .filter_map(|line| line.trim().strip_prefix("export ").or(Some(line.trim())))
        .filter_map(|line| line.split_once('='))
        .find(|(name, _)| name.trim() == KEY_NAME)
        .map(|(_, value)| value.trim().trim_matches(|c| c == '"' || c == '\'').to_string())
        .filter(|key| !key.is_empty())
        .ok_or_else(|| format!("{KEY_NAME} not found in {}", path.display()))
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))?;
    Some(base.join("insertgif").join("env"))
}
