use crossterm::event::Event;
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;

use crate::input_results::InputResult;
use crate::state::AppState;

pub trait Screen {
    /// construct a new screen, passing the global application state.
    fn new(state: AppState) -> Box<Self>
    where
        Self: Sized;
    /// setup functionality of screen before rendering
    fn setup(&mut self);
    /// (re)-draw the screen on the given Terminal
    fn draw(&mut self, terminal: &mut DefaultTerminal);
    /// handle an input event
    fn handle_crossterm_events(&mut self, event: Event) -> InputResult;
    /// reverse necessary stuff from setup and give back global state
    fn teardown(self) -> AppState;
}
