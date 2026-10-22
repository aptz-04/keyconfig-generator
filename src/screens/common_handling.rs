use crossterm::event::{Event, KeyCode, KeyModifiers};

/// Returns a given InputResult in the event of a common escape key/combination being input.
pub fn is_escape_etc(event: &Event) -> bool {
    match event {
        Event::Key(key) => match (key.code, key.modifiers) {
            (KeyCode::Esc | KeyCode::Char('q'), _)
            | (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => true,
            _ => false,
        },
        _ => false,
    }
}
