use std::path::{Path, PathBuf};

use super::EditorMode;
use super::config::Config;

pub struct EditorState {
    _current_mode: EditorMode,
    current_file: Option<PathBuf>,
    config: Option<Config>,
}

impl EditorState {
    pub fn new(initial_mode: EditorMode) -> Self {
        EditorState {
            _current_mode: initial_mode,
            current_file: None,
            config: None,
        }
    }

    pub fn set_imported_config(&mut self, current_file: &Path, current_config: Config) {
        self.current_file = Some(current_file.to_owned());
        self.config = Some(current_config);
    }
}
