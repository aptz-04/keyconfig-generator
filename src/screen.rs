use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame, layout::Rect, style::Stylize, text::Line};

use crate::input_results::InputResult;
use crate::state::AppState;

pub trait Screen {
    /// construct a new screen, passing the global application state.
    fn new() -> Box<Self>
    where
        Self: Sized;
    /// (re)-draw the screen on the given Terminal
    fn draw(&self, state: &mut AppState, terminal: &mut DefaultTerminal);
    /// handle an input event
    fn handle_crossterm_events(&mut self, state: &mut AppState, event: Event) -> InputResult;
}

pub struct ScreenStack {
    stack: Vec<Box<dyn Screen>>,
}

impl ScreenStack {
    /// Construct a new, empty stack of screens.
    pub fn new() -> Self {
        ScreenStack {
            stack: Vec::<Box<dyn Screen>>::new(),
        }
    }
    /// add a boxed screen onto the screen stack
    pub fn push(&mut self, screen: Box<dyn Screen>) {
        self.stack.push(screen)
    }
    /// remove the last added screen, returning the boxed screen.
    pub fn pop(&mut self) -> Option<Box<dyn Screen>> {
        self.stack.pop()
    }
}

pub fn debug_draw(frame: &mut Frame, text: &str) {
    frame.render_widget(Line::from(text).yellow(), Rect::new(0, 0, 30, 1));
}
