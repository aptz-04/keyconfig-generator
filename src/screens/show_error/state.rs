use std::ops::RangeFrom;

use tui_scrollview::ScrollViewState;

pub type ErrorType = anyhow::Error;

pub struct ShowErrorState {
    scroll_view_state: ScrollViewState,
    errors: Vec<ErrorType>,
    new_errors: RangeFrom<usize>,
    currently_shown_errors: RangeFrom<usize>,
}

impl ShowErrorState {
    pub fn new() -> Self {
        ShowErrorState {
            scroll_view_state: ScrollViewState::new(),
            errors: Vec::<ErrorType>::new(),
            new_errors: RangeFrom { start: 0 },
            currently_shown_errors: RangeFrom { start: 0 },
        }
    }

    pub fn get_scrollview_state(&mut self) -> &mut ScrollViewState {
        &mut self.scroll_view_state
    }

    pub fn has_unseen_errors(&self) -> bool {
        self.errors.len() > self.new_errors.start
    }

    pub fn add_error(&mut self, error: ErrorType) {
        self.errors.push(error);
    }

    /// sets the currently shown errors to the range of unseen values
    pub fn set_currently_shown_errors(&mut self) {
        self.currently_shown_errors = self.new_errors.clone();
        self.new_errors = self.errors.len()..;
    }

    /// get resources for rendering of the central scrollable widget
    pub fn get_error_scroll_resources(&mut self) -> (&[ErrorType], &mut ScrollViewState) {
        (
            &self.errors[self.currently_shown_errors.clone()],
            &mut self.scroll_view_state,
        )
    }
}
