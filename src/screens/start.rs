use crate::{
    input_results::InputResult::{NoAction, ReturnFromScreen},
    screen::{Screen, debug_draw},
    state::AppState,
};

pub struct Start {}

impl Screen for Start {
    fn new() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Start {})
    }

    fn draw(&self, state: &AppState, terminal: &mut ratatui::DefaultTerminal) {
        let _ = terminal.draw(|frame| debug_draw(frame, "Start Screen"));
    }

    fn handle_crossterm_events(
        &mut self,
        state: &mut AppState,
        event: crossterm::event::Event,
    ) -> crate::input_results::InputResult {
        todo!()
    }
}
