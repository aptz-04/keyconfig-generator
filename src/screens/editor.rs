use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, layout::Rect, style::Stylize, text::Line};

use crate::{
    input_results::InputResult::{NoAction, ReturnFromScreen},
    screen::Screen,
    state::AppState,
};

use super::common_handling::common_event_handling;

mod mode;
mod state;

// re-export for simpler use
pub use mode::EditorMode;
pub use state::EditorState;

/// Screen responsible for manual editing of the matrix definitions, used keys and keymappings on those used keys.
pub struct Editor {}

impl Screen for Editor {
    /// Construct a new boxed instance.
    fn new() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Editor {})
    }

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
        match common_event_handling(&event) {
            NoAction => (),
            result => return result,
        }

        NoAction
    }
}

impl Editor {
    fn draw_internal(&self, state: &AppState, frame: &mut Frame) {
        frame.render_widget(Line::from("Editor Screen").yellow(), Rect::new(0, 0, 30, 1));
    }
}
