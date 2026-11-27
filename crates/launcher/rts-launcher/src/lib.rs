//! Desktop launcher, configuration, and non-blocking process supervision.

pub mod app;
pub mod config;
pub mod supervisor;

pub use app::LauncherApp;
pub use config::LauncherConfig;
pub use supervisor::{AppTarget, LauncherEvent, LauncherService};
