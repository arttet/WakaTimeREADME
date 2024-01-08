#[cfg(feature = "git")]
mod git_adapter;
mod wakatime_adapter;

#[cfg(feature = "git")]
pub use git_adapter::GitAdapter;
pub use wakatime_adapter::WakaTimeAdapter;
