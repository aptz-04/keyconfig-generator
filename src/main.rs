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

mod editor;
mod error;
mod input_results;
mod screen;
mod state;

use crate::{
    editor::screen::EditorScreen, input_results::InputResult, screen::Screen, state::AppState,
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
    current_screen: Box<dyn Screen>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        App {
            is_running: false,
            event_stream: EventStream::default(),
            current_screen: EditorScreen::new(AppState::new()),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.is_running = true;
        while self.is_running {
            self.current_screen.draw(&mut terminal);

            if let Some(event) = self.event_stream.next().await {
                let result = self.current_screen.handle_crossterm_events(event.unwrap());
                match result {
                    InputResult::QuitApplication => {
                        self.is_running = false;
                    }
                    _ => (),
                };
            }
        }
        Ok(())
    }
}
