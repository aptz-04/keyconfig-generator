use std::path::PathBuf;

use ratatui::{Frame, layout::Rect, style::Stylize, text::Line};

use crate::{
    error::ConfigParserError,
    input_results::InputResult::{NoAction, ReturnFromScreen},
    screen::Screen,
    screens::{common_handling::is_escape_etc, editor::config::Config},
    state::AppState,
};

mod config;
mod mode;
mod state;

// re-export for simpler use
pub use mode::EditorMode;
pub use state::EditorState;

/// Screen responsible for manual editing of the matrix definitions, used keys and keymappings on those used keys.
pub struct Editor {}

impl Screen for Editor {
    /// Handles the drawing of headers, footers and editor grid.
    fn draw(&self, state: &mut AppState, terminal: &mut ratatui::DefaultTerminal) {
        let _ = terminal.draw(|frame| self.draw_internal(state, frame));
    }

    /// Handles Input events specific to this screen.
    fn handle_crossterm_events(
        &mut self,
        state: &mut crate::state::AppState,
        event: crossterm::event::Event,
    ) -> crate::input_results::InputResult {
        if is_escape_etc(&event) {
            return ReturnFromScreen;
        }

        NoAction
    }
}

impl Editor {
    /// Construct a new boxed instance.
    pub fn new() -> Box<Self> {
        Box::new(Editor {})
    }

    /// Tries to parse the currently chosen file in the file explorer and sets up the state and returns the new screen if successful. Returns an error if the input file could not be parsed.
    pub fn try_from_currently_chosen_file(
        state: &mut AppState,
    ) -> Result<Box<Self>, ConfigParserError> {
        let path = state.file_explorer.current().clone().path;
        Self::try_from_file(state, &path)
    }

    /// Tries to parse the file at the given path. Sets up the application state and returns the new screen if successful. Returns an error if the input file could not be parsed.
    pub fn try_from_file(
        state: &mut AppState,
        file: &PathBuf,
    ) -> Result<Box<Self>, ConfigParserError> {
        let config = Config::try_from_file(&file)?;
        state.editor.set_imported_config(&file, config);
        Ok(Box::new(Editor {}))
    }

    fn draw_internal(&self, state: &AppState, frame: &mut Frame) {
        frame.render_widget(Line::from("Editor Screen").yellow(), Rect::new(0, 0, 30, 1));
    }
}
