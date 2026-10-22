use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Offset, Rect},
    style::{Color, Stylize},
    symbols::half_block,
    text::{self, Line, Text},
    widgets::{Block, Paragraph},
};

use tui_textarea::{Input, TextArea};

use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent,
        KeyEventKind, KeyModifiers,
    },
    execute,
};

use std::{
    cmp::{max, min},
    io::stdout,
    ops::Add,
};

use super::grid::{ConfigOption, EditorGrid};
use crate::screen::Screen;
use crate::state::AppState;
use crate::{
    editor::grid_position::{GridPosition, NavigationDirection},
    input_results::InputResult::{self, NoAction, QuitApplication},
};

#[derive(Debug)]
enum EditorMode {
    Navigate,
    Matrix,
    MatrixNaming,
    KeyPlacement,
    KeyMapping,
    Layer,
}

const HEADER_HEIGHT: u16 = 1;
const FOOTER_HEIGHT: u16 = 1;

pub struct EditorScreen<'a> {
    app_state: AppState,
    cursor_pos: GridPosition,
    area_start_pos: GridPosition,
    grid_height: u16,
    grid_width: u16,
    shift_pressed_previously: bool,
    grid: EditorGrid<'a>,
    current_mode: EditorMode,
    input_area: Option<TextArea<'a>>,
}

const COLS_PER_INCREMENT: u16 = 4;
const ROW_PER_INCREMENT: u16 = 2;

impl<'a> Screen for EditorScreen<'a> {
    fn new(state: AppState) -> Box<Self> {
        Box::new(EditorScreen {
            app_state: state,
            cursor_pos: GridPosition { row: 0, col: 0 },
            area_start_pos: GridPosition { row: 0, col: 0 },
            grid_height: 0,
            grid_width: 0,
            shift_pressed_previously: false,
            grid: EditorGrid::new(1, 1, ConfigOption::Default),
            current_mode: EditorMode::Navigate,
            input_area: None,
        })
    }

    fn setup(&mut self) {
        execute!(stdout(), EnableMouseCapture).unwrap();
    }

    fn draw(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        terminal.draw(|frame| self.draw_internal(frame));
    }

    fn handle_crossterm_events(&mut self, event: Event) -> InputResult {
        match event {
            Event::Mouse(_) => NoAction,
            Event::Key(key) => self.handle_key_event(key),
            Event::Resize(_, _) => NoAction,
            _ => NoAction,
        }
    }

    fn teardown(self) -> crate::state::AppState {
        execute!(stdout(), DisableMouseCapture).unwrap();
        self.app_state
    }
}

impl<'a> EditorScreen<'a> {
    fn handle_key_event(&mut self, key: KeyEvent) -> InputResult {
        let shift_pressed = (key.modifiers & KeyModifiers::SHIFT) == KeyModifiers::SHIFT;
        let result = match key.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                self.handle_key_pressed_repeated(key, shift_pressed)
            }
            KeyEventKind::Release => NoAction,
        };
        self.shift_pressed_previously = shift_pressed;
        return result;
    }

    fn handle_key_pressed_repeated(&mut self, key: KeyEvent, shift_pressed: bool) -> InputResult {
        let input_area_active = self.input_area.is_some();
        if input_area_active {
            match key.code {
                KeyCode::Enter => {
                    self.change_mode(EditorMode::KeyPlacement);
                    self.input_area = None
                }
                _ => {
                    if let Some(area) = &mut self.input_area {
                        area.input(Input::from(key.clone()));
                    }
                }
            }

            return NoAction;
        }

        match (key.code, key.modifiers) {
            (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down, _) => {
                self.move_cursor(key.code, shift_pressed);
                NoAction
            }
            (KeyCode::Esc | KeyCode::Char('q'), _)
            | (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => QuitApplication,
            (KeyCode::Char('m'), _) => {
                self.change_mode(EditorMode::Matrix);
                NoAction
            }
            (KeyCode::Enter, _) => match self.current_mode {
                EditorMode::Navigate => NoAction,
                EditorMode::Matrix => {
                    self.change_mode(EditorMode::MatrixNaming);
                    self.input_area = Some(TextArea::default());
                    NoAction
                }
                EditorMode::MatrixNaming => NoAction,
                EditorMode::KeyPlacement => NoAction,
                EditorMode::KeyMapping => NoAction,
                EditorMode::Layer => NoAction,
            },
            _ => NoAction,
        }
    }

    fn handle_key_released(&mut self, key: KeyEvent, shift_pressed: bool) -> InputResult {
        todo!()
    }

    fn draw_cursor_and_selection(&mut self, frame: &mut Frame, editor_area: Rect) {
        let selection_width =
            ((self.cursor_pos.col as i16 - self.area_start_pos.col as i16).abs() + 1) as u16;
        let selection_height =
            ((self.cursor_pos.row as i16 - self.area_start_pos.row as i16).abs() + 1) as u16;

        frame.render_widget(
            Block::new().red(),
            self.grid
                .get_area_between_positions(self.cursor_pos, self.area_start_pos)
                .offset(Offset::new(editor_area.x as i32, editor_area.y as i32)),
        );
    }

    fn draw_internal(&mut self, frame: &mut Frame) {
        // first get the newest terminal area
        let frame_area = frame.area();

        let layout = Layout::vertical([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Fill(1),
            Constraint::Length(FOOTER_HEIGHT),
        ]);

        let [header, editor, footer] = layout.areas(frame_area);

        let footer_layout =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [status, input] = footer_layout.areas(footer);

        self.grid.check_and_resize(editor);
        (self.grid_height, self.grid_width) = self.grid.max_row_col(editor);

        // write something to the header
        frame.render_widget(Paragraph::new("unsaved config"), header);
        match self.current_mode {
            EditorMode::Matrix => self.grid.draw(frame, editor),
            EditorMode::MatrixNaming => {
                if let Some(a) = &self.input_area {
                    frame.render_widget(a, input);
                }
            }
            _ => (), // EditorMode::Navigate => todo!(),
                     // EditorMode::KeyPlacement => todo!(),
                     // EditorMode::KeyMapping => todo!(),
                     // EditorMode::Layer => todo!(),
        }
        self.draw_cursor_and_selection(frame, editor);
        frame.render_widget(
            Paragraph::new(format!("editor mode: {:?}", self.current_mode)),
            status,
        );
        // frame.render_widget(, area);
    }

    fn move_cursor(&mut self, code: KeyCode, shift_pressed: bool) {
        self.cursor_pos.move_position(
            match code {
                KeyCode::Right => NavigationDirection::Right,
                KeyCode::Left => NavigationDirection::Left,
                KeyCode::Up => NavigationDirection::Up,
                KeyCode::Down => NavigationDirection::Down,
                _ => NavigationDirection::None,
            },
            GridPosition {
                row: self.grid_height as usize,
                col: self.grid_width as usize,
            },
        );

        if !shift_pressed {
            self.area_start_pos = self.cursor_pos;
        }
    }

    fn change_mode(&mut self, mode: EditorMode) {
        self.current_mode = mode;
    }
}
