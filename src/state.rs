use crate::screens::{
    editor::{EditorMode, EditorState},
    start::StartState,
};

/// application state initialized at the start and persisted/mutated throughout runtime
pub struct AppState {
    pub editor: EditorState,
    pub start: StartState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            editor: EditorState::new(EditorMode::MatrixDefinition),
            start: StartState::new(),
        }
    }
}
