use ratatui_explorer::FileExplorer;
use std::path::PathBuf;

use crate::{
    Cli,
    error::CliArgumentError,
    screens::{
        editor::{EditorMode, EditorState},
        show_error::ShowErrorScreenState,
        start::{Start, StartState},
    },
};

/// application state initialized at the start and persisted/mutated throughout runtime
pub struct AppState {
    pub editor: EditorState,
    pub start: StartState,
    pub file_explorer: FileExplorer,
    pub initial_config_file: Option<PathBuf>,
    pub error_screen_state: ShowErrorScreenState,
}

impl AppState {
    pub fn new(args: Cli) -> Self {
        let mut error_state = ShowErrorScreenState::new();

        let working_directory = match check_working_directory_argument(args.working_directory) {
            Ok(path) => path,
            Err(error) => {
                error_state.add_error(error.to_string());
                PathBuf::from(".")
            }
        };

        let initial_config = match check_config_file_argument(args.config_file) {
            Ok(path) => path,
            Err(error) => {
                error_state.add_error(error.to_string());
                None
            }
        };

        AppState {
            editor: EditorState::new(EditorMode::MatrixDefinition),
            start: StartState::new(),
            file_explorer: Start::new_file_explorer(&working_directory),
            initial_config_file: initial_config,
            error_screen_state: error_state,
        }
    }
}

/// checks the existence of the supplied path by calling canonicalize
fn check_working_directory_argument(path: PathBuf) -> Result<PathBuf, CliArgumentError> {
    match path.canonicalize() {
        Ok(path) => Ok(path),
        Err(source) => Err(CliArgumentError::NonExistentWorkingDirectory { path, source }),
    }
}

/// checks the existence of the supplied path by calling canonicalize
fn check_config_file_argument(path: Option<PathBuf>) -> Result<Option<PathBuf>, CliArgumentError> {
    let Some(path) = path else { return Ok(None) };
    match path.canonicalize() {
        Ok(path) => Ok(Some(path)),
        Err(source) => Err(CliArgumentError::NonExistentConfigFile { path, source }),
    }
}
