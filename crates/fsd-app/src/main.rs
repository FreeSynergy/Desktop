use tracing_subscriber::EnvFilter;

fn main() {
    // Initialize tracing first so the panic hook can use it.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Global panic handler — logs the panic via tracing before unwinding.
    std::panic::set_hook(Box::new(|info| {
        tracing::error!("PANIC: {info}");
        // TODO: surface via NotificationBus once available
    }));

    tracing::info!("Starting FreeSynergy.Desktop");

    // Launch the desktop shell.
    // Dioxus 0.6 desktop: configure the main window via LaunchBuilder,
    // then spawn per-app native windows on demand via window().new_window().
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::Config;

        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                Config::new()
                    .with_window(
                        dioxus::desktop::WindowBuilder::new()
                            .with_title("FreeSynergy.Desktop")
                            .with_decorations(false)
                            .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0_f64, 800.0_f64))
                            .with_min_inner_size(dioxus::desktop::LogicalSize::new(900.0_f64, 600.0_f64))
                            .with_resizable(true),
                    )
                    .with_background_color((12, 18, 34, 255))
                    // Allow iframes to load any external URL (needed for the Browser app).
                    // The custom_head injects a permissive CSP so WebKit does not block
                    // cross-origin frame navigation from the dioxus:// protocol context.
                    .with_custom_head(
                        r#"<meta http-equiv="Content-Security-Policy"
                             content="default-src * 'unsafe-inline' 'unsafe-eval' data: blob:;
                                      frame-src *;">"#.to_string()
                    ),
            )
            .launch(fsd_shell::Desktop);
    }

    #[cfg(feature = "web")]
    dioxus::launch(fsd_shell::WebDesktop);
}
