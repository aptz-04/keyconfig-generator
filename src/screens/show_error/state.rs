use std::ops::RangeFrom;

use tui_scrollview::ScrollViewState;

pub struct ShowErrorScreenState {
    scroll_view_state: ScrollViewState,
    errors: Vec<String>,
    new_errors: RangeFrom<usize>,
    currently_shown_errors: Vec<String>,
}

impl ShowErrorScreenState {
    pub fn new() -> Self {
        ShowErrorScreenState {
            scroll_view_state: ScrollViewState::new(),
            errors: Vec::<String>::new(),
            new_errors: RangeFrom { start: 0 },
            currently_shown_errors: Vec::<String>::new(),
        }
    }

    pub fn get_scrollview_state(&mut self) -> &mut ScrollViewState {
        &mut self.scroll_view_state
    }

    pub fn has_unseen_errors(&self) -> bool {
        self.errors.len() > self.new_errors.start
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    /// sets the currently shown errors to the range of unseen values
    pub fn set_currently_shown_errors(&mut self) {
        let unseen_range = &self.errors[self.new_errors.clone()];
        self.new_errors = self.errors.len()..;

        self.currently_shown_errors = unseen_range.into()
    }

    pub fn get_currently_shown_errors(&mut self) -> Vec<String> {
        self.currently_shown_errors.clone()
    }

    pub fn view_state(&mut self) -> &mut ScrollViewState {
        &mut self.scroll_view_state
    }
}
