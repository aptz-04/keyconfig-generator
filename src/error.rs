use std::path::PathBuf;
use thiserror::Error;

pub enum MatrixError {
    InvalidMatrixIndex,
    UnknownMatrixName,
    DuplicateMatrixName,
}

#[derive(Debug, Error)]
pub enum ConfigParserError {
    #[error("File \"{0}\" could not be parsed")]
    InvalidFile(PathBuf),
}

/// Errors related to the initial cli argument check
#[derive(Debug, Error)]
pub enum CliArgumentError {
    // errors related to --working-directory/-w argument
    #[error("Working directory path \"{path}\" does not exist or is invalid: {source}")]
    NonExistentWorkingDirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // errors related to --config-file/-c argument
    #[error("Invalid config file path \"{path}\": {source}")]
    NonExistentConfigFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
