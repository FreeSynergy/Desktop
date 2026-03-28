//! Tests for fs-shell CSS/config outputs.

#[test]
fn theme_css_vars_not_empty() {
    let css = fs_theme::ThemeEngine::default().to_css();
    assert!(!css.is_empty(), "CSS vars output must not be empty");
    assert!(css.contains(":root"), "CSS must contain a :root block");
}

#[test]
fn theme_full_css_not_empty() {
    let css = fs_theme::ThemeEngine::default().to_full_css();
    assert!(!css.is_empty(), "Full CSS output must not be empty");
}

#[test]
fn theme_glass_css_not_empty() {
    let css = fs_theme::ThemeEngine::glass_css();
    assert!(!css.is_empty(), "Glass CSS output must not be empty");
}

#[test]
fn theme_animations_css_not_empty() {
    let css = fs_theme::ThemeEngine::animations_css();
    assert!(!css.is_empty(), "Animations CSS output must not be empty");
}
