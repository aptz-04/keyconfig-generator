use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, layout::Rect, style::Stylize, text::Line};

use crate::{
    input_results::InputResult::{NoAction, ReturnFromScreen},
    screen::Screen,
    state::AppState,
};

pub struct Editor {}

impl Screen for Editor {
    fn new() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Editor {})
    }

    fn draw(&self, state: &AppState, terminal: &mut ratatui::DefaultTerminal) {
        let _ = terminal.draw(|frame| self.draw_internal(state, frame));
    }

    fn handle_crossterm_events(
        &mut self,
        state: &mut crate::state::AppState,
        event: crossterm::event::Event,
    ) -> crate::input_results::InputResult {
        match event {
            Event::Key(key) => match (key.code, key.modifiers) {
                (KeyCode::Esc | KeyCode::Char('q'), _)
                | (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
                    ReturnFromScreen
                }
                _ => NoAction,
            },
            _ => NoAction,
        }
    }
}

impl Editor {
    fn draw_internal(&self, state: &AppState, frame: &mut Frame) {
        frame.render_widget(Line::from("Editor Screen").yellow(), Rect::new(0, 0, 30, 1));
    }
}
