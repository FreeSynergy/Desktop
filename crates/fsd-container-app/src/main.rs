fn main() {
    #[cfg(feature = "desktop")]
    dioxus::launch(fsd_container_app::ContainerApp);
}
