use crate::editor::state::ConfigState;

/// application state initialized at the start and persisted/mutated throughout runtime
pub struct AppState {
    pub config: ConfigState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            config: ConfigState::new(),
        }
    }
}
