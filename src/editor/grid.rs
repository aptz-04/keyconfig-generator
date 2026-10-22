use std::cmp::{max, min};
use std::ops::Add;

use ratatui::{Frame, layout::Rect, style::Color, symbols::half_block, text::Text};

use super::grid_entry::GridEntry;
use super::grid_position::{GridPosition, NavigationDirection};

const DEFAULT_GRID_CONFIG: GridConfig = GridConfig {
    max_cols: 30,
    max_rows: 20,
    element_width: 6,
    element_height: 3,
    internal_column_margin: 0,
    internal_row_margin: 0,
    external_column_margin: 2,
    external_row_margin: 1,
};

#[derive(Copy, Clone)]
pub struct GridConfig {
    max_cols: u32,
    max_rows: u32,
    element_width: u32,
    element_height: u32,
    internal_column_margin: u32,
    internal_row_margin: u32,
    external_column_margin: u32,
    external_row_margin: u32,
}

pub enum ConfigOption {
    Default,
    Some(GridConfig),
}

pub struct EditorGrid<'a> {
    rows: Vec<Vec<GridEntry<'a>>>,
    config: GridConfig,
}

impl<'a> EditorGrid<'_> {
    pub fn new(n_rows: u16, n_cols: u16, config: ConfigOption) -> Self {
        EditorGrid {
            rows: vec![vec![GridEntry::new(); n_cols as usize]; n_rows as usize],
            config: match config {
                ConfigOption::Default => DEFAULT_GRID_CONFIG,
                ConfigOption::Some(c) => c,
            },
        }
    }

    /// Computes the maximum number of columns fitting into the given angle
    pub fn max_row_col(&self, area: Rect) -> (u16, u16) {
        let c = self.config;
        let n_rows = (area.height as u32 - 2 * c.external_row_margin + c.internal_row_margin)
            / (c.element_height + c.internal_row_margin);
        let n_cols = (area.width as u32 - 2 * c.external_column_margin + c.internal_column_margin)
            / (c.element_width + c.internal_column_margin);

        (n_rows as u16, n_cols as u16)
    }

    /// Checks whether the current given editor area is filled with grid
    /// entries and grows the EditorGrid if not. Never shrinks the area as
    /// data could be lost doing so.
    pub fn check_and_resize(&mut self, area: Rect) {
        // calc maximum number of grid entries that fit into the area
        let (n_rows, n_cols) = self.max_row_col(area);

        // only extend the vector if needed, do not truncate
        let target_rows = max(
            min(self.config.max_rows as u16, n_rows),
            self.rows.len() as u16,
        );
        let target_cols = max(
            min(self.config.max_cols as u16, n_cols),
            self.rows.first().unwrap().len() as u16,
        );

        for row in &mut self.rows {
            row.resize(target_cols as usize, GridEntry::default())
        }
        self.rows.resize(
            target_rows as usize,
            vec![GridEntry::default(); target_cols as usize],
        );
    }

    pub fn get_area_between_positions(&self, a: GridPosition, b: GridPosition) -> Rect {
        let config = self.config;

        let n_cols = ((a.col as i16 - b.col as i16).abs() + 1) as u32;
        let n_rows = ((a.row as i16 - b.row as i16).abs() + 1) as u32;

        let element_width_with_margin = config.element_width + config.internal_column_margin;
        let element_height_with_margin = config.element_height + config.internal_row_margin;

        let (left_upper_x, left_upper_y) = (
            min(a.col as u32, b.col as u32) * element_width_with_margin
                + config.external_column_margin,
            min(a.row as u32, b.row as u32) * element_height_with_margin
                + config.external_row_margin,
        );

        Rect {
            x: left_upper_x as u16,
            y: left_upper_y as u16,
            width: (element_width_with_margin * n_cols - config.internal_column_margin) as u16,
            height: (element_height_with_margin * n_rows - config.internal_row_margin) as u16,
        }
    }

    pub fn draw(&self, frame: &mut Frame, containing_area: Rect) {
        let config = self.config;
        let element_text = String::from(half_block::FULL)
            .repeat(config.element_width as usize)
            .add("\n")
            .repeat(config.element_height as usize);

        let (max_row, max_col) = self.max_row_col(containing_area);
        let (max_row, max_col) = (max_row as usize, max_col as usize);

        for (i, row) in (&self.rows)
            .iter()
            .enumerate()
            .filter(|(i, _)| *i < max_row)
        {
            for (j, element) in row.iter().enumerate().filter(|(j, _)| *j < max_col) {
                let mut area = Rect::new(
                    (config.external_column_margin
                        + (j as u32) * (config.element_width + config.internal_column_margin))
                        as u16,
                    (config.external_row_margin
                        + (i as u32) * (config.element_height + config.internal_row_margin))
                        as u16,
                    config.element_width as u16,
                    config.element_height as u16,
                );
                (area.x, area.y) = (area.x + containing_area.x, area.y + containing_area.y);

                let color = match element.in_use {
                    true => Color::LightYellow,
                    false => Color::DarkGray,
                };

                frame.render_widget(Text::from(element_text.as_str()).style(color), area);
            }
        }
    }
}
