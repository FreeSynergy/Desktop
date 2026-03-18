fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                Config::new().with_window(
                    WindowBuilder::new()
                        .with_title("FreeSynergy — Builder")
                        .with_inner_size(LogicalSize::new(1100.0_f64, 760.0_f64))
                        .with_resizable(true),
                ),
            )
            .launch(fsd_builder::BuilderApp);
    }
}
