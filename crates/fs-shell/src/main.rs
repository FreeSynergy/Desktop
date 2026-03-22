fn main() {
    // Initialize i18n before Dioxus starts — guarantees all translation keys
    // are resolved before any component renders for the first time.
    fs_shell::init_i18n();

    #[cfg(feature = "desktop")]
    fs_shell::launch_desktop(
        fs_shell::DesktopConfig::new().with_all_navigation(),
        fs_shell::Desktop,
    );
}
