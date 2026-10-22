use std::cmp::{max, min};

type PositionType = usize;

/// Position in grid as (row, col)
#[derive(Copy, Clone)]
pub struct GridPosition {
    pub row: PositionType,
    pub col: PositionType,
}

pub enum NavigationDirection {
    Up,
    Down,
    Right,
    Left,
    None,
}

impl GridPosition {
    /// Construct a new grid with current position at (0,0).
    pub fn new(n_rows: PositionType, n_cols: PositionType) -> Self {
        GridPosition { row: 0, col: 0 }
    }

    /// Move into given direction, stopping at any eventual border and returning the GridPosition after the move.
    pub fn move_position(&mut self, dir: NavigationDirection, maximum: GridPosition) {
        let (r, c) = (self.row as i32, self.col as i32);

        let (r_new, c_new) = match dir {
            NavigationDirection::Left => (r, c - 1),
            NavigationDirection::Right => (r, c + 1),
            NavigationDirection::Up => (r - 1, c),
            NavigationDirection::Down => (r + 1, c),
            NavigationDirection::None => (r, c),
        };

        self.row = min(max(r_new, 0), (maximum.row as i32) - 1) as PositionType;
        self.col = min(max(c_new, 0), (maximum.col as i32) - 1) as PositionType;
    }

    /// Get the position as (row, column) tuple
    pub fn get_components(&self) -> (PositionType, PositionType) {
        (self.row, self.col)
    }
}
