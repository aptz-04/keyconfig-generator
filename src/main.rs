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
    // Event stream.
    event_stream: EventStream,
    state: AppState,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        // setup screens, editor is the first one displayed
        App {
            event_stream: EventStream::default(),
            state: AppState::new(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let mut current_screen: Box<dyn Screen> = Start::new();
        let mut screen_stack = ScreenStack::new();

        loop {
            current_screen.draw(&mut self.state, &mut terminal);

            if let Some(event) = self.event_stream.next().await {
                match current_screen.handle_crossterm_events(&mut self.state, event.unwrap()) {
                    InputResult::QuitApplication => break,
                    InputResult::ReturnFromScreen => {
                        // pop the next screen from stack, if there's None, quit the application
                        if let Some(new_screen) = screen_stack.pop() {
                            current_screen = new_screen
                        } else {
                            break;
                        };
                    }
                    InputResult::GoToScreen(new_screen) => {
                        screen_stack.push(current_screen);
                        current_screen = new_screen;
                    }
                    _ => (),
                };
            }
        }
        Ok(())
    }
}
