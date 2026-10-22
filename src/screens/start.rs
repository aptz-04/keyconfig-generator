use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Modifier,
    widgets::List,
};
use ratatui_explorer::{FileExplorerBuilder, Theme};

use crate::{
    App,
    input_results::InputResult::{self, GoToScreen, NoAction},
    screen::Screen,
    screens::editor::Editor,
    state::AppState,
};

use super::common_handling::common_event_handling;

mod mode;
mod state;

pub use mode::StartMode;
pub use state::StartState;

pub struct Start {}

impl Screen for Start {
    fn new() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Start {})
    }

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
        // state/screen agnostic handling of common input events.
        match common_event_handling(&event) {
            NoAction => (),
            result => return result,
        }

        // mode dependent handling
        match state.start.current_mode() {
            StartMode::OptionSelection => Self::handle_list_navigation(&event, state),
            StartMode::FileSelection => todo!(),
        }
    }
}

impl Start {
    /// Draw the start screen according to current app state. The state needs to be mutable due to the stateful widget signature.
    fn draw_list_selection(&self, state: &mut AppState, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .margin(5);

        let [top, center, bottom] = frame.area().layout(&layout);

        let options = ["New Configuration", "Load from file"];
        let list = List::new(options)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, center, state.start.get_list_state());
    }

    fn draw_file_selection(&self, state: &mut AppState, frame: &mut Frame) {}

    /// Handle eventual list navigation upon key input
    fn handle_list_navigation(event: &Event, state: &mut AppState) -> InputResult {
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
}
