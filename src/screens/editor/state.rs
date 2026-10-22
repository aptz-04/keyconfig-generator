use super::EditorMode;

pub struct EditorState {
    current_mode: EditorMode,
}

impl EditorState {
    pub fn new(initial_mode: EditorMode) -> Self {
        EditorState {
            current_mode: initial_mode,
        }
    }
}
