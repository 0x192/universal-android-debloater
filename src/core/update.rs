use crate::core::utils::Release;

#[derive(Debug, PartialEq, Clone)]
pub enum SelfUpdateStatus {
    InProgress,
    Done,
    Failed,
}

impl Default for SelfUpdateStatus {
    fn default() -> Self {
        SelfUpdateStatus::InProgress
    }
}

impl std::fmt::Display for SelfUpdateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SelfUpdateStatus::InProgress => "Checking updates...",
            SelfUpdateStatus::Failed => "Failed to check update!",
            SelfUpdateStatus::Done => "Done",
        };
        write!(f, "{}", s)
    }
}

#[derive(Default, Debug, Clone)]
pub struct SelfUpdateState {
    pub latest_release: Option<Release>,
    pub status: SelfUpdateStatus,
}
