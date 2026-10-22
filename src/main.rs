use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent,
        KeyEventKind, KeyModifiers,
    },
    execute,
};
use futures::{FutureExt, StreamExt};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

mod screens;

mod error;
mod input_results;
mod screen;
mod state;

use crate::{
    input_results::InputResult,
    screen::{Screen, ScreenStack},
    screens::editor::Editor,
    screens::start::Start,
    state::AppState,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}

pub struct App {
    /// Is the application running?
    is_running: bool,
    // Event stream.
    event_stream: EventStream,
    screen_stack: ScreenStack,
    current_screen: Option<Box<dyn Screen>>,
    state: AppState,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        // setup screens, editor is the first one displayed
        App {
            is_running: false,
            event_stream: EventStream::default(),
            screen_stack: ScreenStack::new(),
            current_screen: None,
            state: AppState::new(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let initial_screen = Start::new();
        self.current_screen = Some(initial_screen);

        while let Some(ref mut screen) = self.current_screen {
            screen.draw(&self.state, &mut terminal);
            if let Some(event) = self.event_stream.next().await {
                match screen.handle_crossterm_events(&mut self.state, event.unwrap()) {
                    InputResult::QuitApplication => break,
                    InputResult::ReturnFromScreen => {
                        // pop the next screen from stack, if there's None, quit the application
                        self.current_screen = self.screen_stack.pop();
                    }
                    _ => (),
                };
            }
        }
        Ok(())
    }
}
