use std::path::PathBuf;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Modifier,
    widgets::{FrameExt, List},
};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};

use crate::{
    input_results::InputResult::{self, GoToScreen, NoAction, QuitApplication},
    screen::Screen,
    screens::{common_handling::is_escape_etc, editor::Editor},
    state::AppState,
};

mod mode;
mod state;

pub use mode::StartMode;
pub use state::StartState;

pub struct Start {}

impl Screen for Start {
    fn draw(&self, state: &mut AppState, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| match state.start.current_mode() {
            StartMode::OptionSelection => self.draw_list_selection(state, frame),
            StartMode::FileSelection => self.draw_file_selection(state, frame),
        });
    }

    fn handle_crossterm_events(
        &mut self,
        state: &mut AppState,
        event: Event,
    ) -> crate::input_results::InputResult {
        // mode dependent handling
        match state.start.current_mode() {
            StartMode::OptionSelection => Self::handle_list_navigation(&event, state),
            StartMode::FileSelection => Self::handle_explorer_navigation(&event, state),
        }
    }
}

impl Start {
    /// Construct new instance of the start screen.
    pub fn new() -> Box<Self> {
        Box::new(Start {})
    }

    /// Draw the start screen according to current app state. The state needs to be mutable due to the stateful widget signature.
    fn draw_list_selection(&self, state: &mut AppState, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .margin(5);

        let [_, center, _] = frame.area().layout(&layout);

        let options = ["New Configuration", "Load from file"];
        let list = List::new(options)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, center, state.start.get_list_state());
    }

    fn draw_file_selection(&self, state: &mut AppState, frame: &mut Frame) {
        frame.render_widget_ref(state.file_explorer.widget(), frame.area());
    }

    /// Handle eventual list navigation upon key input
    fn handle_list_navigation(event: &Event, state: &mut AppState) -> InputResult {
        if is_escape_etc(event) {
            return QuitApplication;
        }

        let list_state = state.start.get_list_state();
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Enter => {
                    if let Some(index) = list_state.selected() {
                        match index {
                            0 => return GoToScreen(Editor::new()),
                            1 => state.start.set_current_mode(StartMode::FileSelection),
                            _ => (),
                        }
                    }
                }
                KeyCode::Down => list_state.select_next(),
                KeyCode::Up => list_state.select_previous(),
                _ => (),
            },
            _ => (),
        }
        NoAction
    }

    fn handle_explorer_navigation(event: &Event, state: &mut AppState) -> InputResult {
        if is_escape_etc(event) {
            state.start.set_current_mode(StartMode::OptionSelection);
        }

        if let Event::Key(key) = event {
            // catch enter if file is chosen, otherwise pass to explorer to handle.
            if (key.code == KeyCode::Enter) && state.file_explorer.current().is_file() {
                if let Ok(screen) = Editor::try_from_file(state) {
                    return GoToScreen(screen);
                } else {
                    todo!("Display parser error and do nothing");
                }
            }
        }

        let _ = state.file_explorer.handle(event);
        NoAction
    }

    pub fn new_file_explorer(initial_directory: &PathBuf) -> FileExplorer {
        let theme = Theme::default().add_default_title();
        let mut explorer = FileExplorerBuilder::build_with_theme(theme)
            .expect("Building the file explorer failed!");
        explorer.set_cwd(initial_directory).expect(
            format!(
                "Could not set file explorer to directory \"{:?}\"",
                initial_directory
            )
            .as_str(),
        );
        explorer
    }
}
