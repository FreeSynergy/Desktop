#![deny(clippy::all, clippy::pedantic, warnings)]
pub mod app;

pub use app::{LinkedAccount, PersonalCapability, ProfileApp, ProfileMessage, SshKey, UserProfile};
