use crate::error::MatrixError;

type KeyCode = u16;
const KEYCODE_INIT_VALUE: KeyCode = 0;

/// defines the name as well as dimensions and placement of a key matrix
#[derive(Debug, Clone)]
pub struct KeyMatrixSpecs {
    name: String,
    nrows: usize,
    ncols: usize,
    position_x: u32,
    position_y: u32,
}

impl KeyMatrixSpecs {
    pub fn new(name: &str, nrows: usize, ncols: usize, posx: u32, posy: u32) -> Self {
        KeyMatrixSpecs {
            name: String::from(name),
            nrows: nrows,
            ncols: ncols,
            position_x: posx,
            position_y: posy,
        }
    }
}

/// holds mapping and definition of a given key matrix
struct KeyMatrix {
    specs: KeyMatrixSpecs,
    mapping: KeyMapping,
}

/// holds the mapping of a given (row, col) tuple to the corresponding key code
#[derive(Debug, Clone)]

struct KeyMapping {
    rows: Vec<Vec<KeyCode>>,
}

impl KeyMapping {
    /// Initialize empty key mapping with the given dimensions
    fn new_with_dims(nrows: usize, ncols: usize) -> Self {
        KeyMapping {
            rows: vec![vec![KEYCODE_INIT_VALUE; ncols]; nrows],
        }
    }
}

/// holds key matrices specs as well as key associations
pub struct ConfigState {
    matrices: Vec<KeyMatrix>,
}

impl ConfigState {
    /// constructor for empty, uninitialized state
    pub fn new() -> Self {
        ConfigState {
            matrices: Vec::new(),
        }
    }

    /// helper to check whether a matrix with a given name exists in the config
    fn name_exists(&self, name: &str) -> bool {
        self.matrices
            .iter()
            .filter(|&m| m.specs.name == name)
            .count()
            > 0
    }

    /// add a new matrix to the configuration
    fn add_new_matrix(&mut self, mut specs: KeyMatrixSpecs) -> Result<(), MatrixError> {
        // check that name is not already in use
        if self.name_exists(specs.name.as_str()) {
            return Err(MatrixError::DuplicateMatrixName);
        }

        let mapping = KeyMapping::new_with_dims(specs.nrows, specs.ncols);
        self.matrices.push(KeyMatrix { specs, mapping });
        Ok(())
    }

    /// add a mapping from matrix coordinate to value
    fn add_mapping(
        &mut self,
        name: &str,
        row: usize,
        col: usize,
        value: KeyCode,
    ) -> Result<(), MatrixError> {
        // find matrix corresponding to name
        let mut matches = self.matrices.iter_mut().filter(|s| s.specs.name == name);
        if let Some(matrix) = matches.next() {
            if (row >= matrix.specs.nrows) || (col >= matrix.specs.ncols) {
                return Err(MatrixError::InvalidMatrixIndex);
            } else {
                // should be safe to unwrap since the mapping is initialized latest in add_new_matrix
                matrix.mapping.rows[row][col] = value;
                Ok(())
            }
        } else {
            return Err(MatrixError::UnknownMatrixName);
        }
    }
}
