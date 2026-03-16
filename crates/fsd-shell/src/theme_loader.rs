// Theme loader — prefix injection for store themes.
//
// Store themes are saved WITHOUT a CSS variable prefix (e.g. `--bg-base`).
// Each program adds its own prefix when loading (e.g. `--fsn-bg-base` for Desktop).
// This mirrors the strategy described in technik/css.md.

// ── Required variables (validated on theme upload/import) ──────────────────────

/// Canonical theme variable names without prefix (as stored in the Store).
pub const REQUIRED_VARS: &[&str] = &[
    "bg-base", "bg-surface", "bg-elevated", "bg-card", "bg-input",
    "text-primary", "text-secondary", "text-muted",
    "primary", "primary-hover", "primary-text",
    "accent",
    "success", "warning", "error",
    "border", "border-focus",
];

// ── Prefix injection ──────────────────────────────────────────────────────────

/// Injects a CSS variable prefix into all `--` declarations in `css`.
///
/// `--bg-base` → `--{prefix}-bg-base`.
/// Variables that already start with `--{prefix}-` are left untouched.
///
/// # Example
/// ```
/// let store_css = ":root { --bg-base: #0c1222; --text-primary: #e8edf5; }";
/// let desktop_css = prefix_theme_css(store_css, "fsn");
/// assert!(desktop_css.contains("--fsn-bg-base: #0c1222"));
/// ```
pub fn prefix_theme_css(css: &str, prefix: &str) -> String {
    let _guard = format!("--{prefix}-");
    let mut out = String::with_capacity(css.len() + css.len() / 4);
    let mut chars = css.chars().peekable();

    while let Some(c) = chars.next() {
        out.push(c);
        // Detect `--` that is NOT already prefixed.
        if c == '-' && chars.peek() == Some(&'-') {
            out.push(chars.next().unwrap()); // second `-`

            // Read ahead to check if the prefix is already there.
            let mut ahead = String::new();
            let guard_inner = format!("{prefix}-");
            for _ in 0..guard_inner.len() {
                if let Some(nc) = chars.next() {
                    ahead.push(nc);
                } else {
                    break;
                }
            }
            if ahead == guard_inner {
                // Already prefixed — keep as-is.
                out.push_str(&ahead);
            } else {
                // Not yet prefixed — inject prefix.
                out.push_str(&format!("{prefix}-"));
                out.push_str(&ahead);
            }
        }
    }
    out
}

// ── Theme validation ──────────────────────────────────────────────────────────

/// Validates that all required variables are present in `css` (without prefix).
///
/// Returns a list of missing variable names (empty = valid).
pub fn validate_theme_vars(css: &str) -> Vec<&'static str> {
    REQUIRED_VARS
        .iter()
        .copied()
        .filter(|var| {
            let declaration = format!("--{var}");
            !css.contains(&declaration)
        })
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_injection() {
        let css = ":root { --bg-base: #000; --text-primary: #fff; }";
        let out = prefix_theme_css(css, "fsn");
        assert!(out.contains("--fsn-bg-base: #000"), "got: {out}");
        assert!(out.contains("--fsn-text-primary: #fff"), "got: {out}");
    }

    #[test]
    fn no_double_prefix() {
        let css = ":root { --fsn-bg-base: #000; }";
        let out = prefix_theme_css(css, "fsn");
        assert!(!out.contains("--fsn-fsn-"), "got: {out}");
        assert!(out.contains("--fsn-bg-base: #000"), "got: {out}");
    }

    #[test]
    fn validate_missing_vars() {
        let css = "--bg-base: #0c1222; --text-primary: #e8edf5;";
        let missing = validate_theme_vars(css);
        assert!(missing.contains(&"bg-surface"), "expected bg-surface missing");
        assert!(!missing.contains(&"bg-base"), "bg-base should be present");
    }
}
