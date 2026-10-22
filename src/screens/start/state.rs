use ratatui::widgets::ListState;

use super::StartMode;

pub struct StartState {
    current_mode: StartMode,
    list_state: ListState,
}

impl StartState {
    /// Construct a clean state for the start screen.
    pub fn new() -> Self {
        StartState {
            current_mode: StartMode::OptionSelection,
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn get_list_state(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn set_current_mode(&mut self, mode: StartMode) {
        self.current_mode = mode
    }

    pub fn current_mode(&self) -> StartMode {
        self.current_mode
    }
}
