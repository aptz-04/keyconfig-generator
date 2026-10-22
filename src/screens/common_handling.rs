use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::input_results::InputResult::{self, NoAction, QuitApplication};

pub fn common_event_handling(event: &Event) -> InputResult {
    match event {
        Event::Key(key) => match (key.code, key.modifiers) {
            (KeyCode::Esc | KeyCode::Char('q'), _)
            | (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => QuitApplication,
            _ => NoAction,
        },
        _ => NoAction,
    }
}
