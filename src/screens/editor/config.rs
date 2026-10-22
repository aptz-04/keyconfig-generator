use std::path::Path;

use crate::error::ConfigParserError;

/// The currently edited keyboard configuration data
pub struct Config {}

impl Config {
    /// Construct a new, empty config.
    pub fn _new() -> Self {
        Config {}
    }

    /// Try to construct a new config from an appropriate config file.
    pub fn try_from_file(path: &Path) -> Result<Self, ConfigParserError> {
        // todo!("Implement the actual input parsing");
        Err(ConfigParserError::InvalidFile(path.to_owned()))
        // Ok(Self {})
    }
}
