use ratatui_explorer::FileExplorer;
use std::path::PathBuf;

use crate::screens::{
    editor::{EditorMode, EditorState},
    start::{Start, StartState},
};

/// application state initialized at the start and persisted/mutated throughout runtime
pub struct AppState {
    pub editor: EditorState,
    pub start: StartState,
    pub file_explorer: FileExplorer,
}

impl AppState {
    pub fn new(working_directory: PathBuf) -> Self {
        AppState {
            editor: EditorState::new(EditorMode::MatrixDefinition),
            start: StartState::new(),
            file_explorer: Start::new_file_explorer(&working_directory),
        }
    }
}
