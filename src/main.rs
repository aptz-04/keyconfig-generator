use clap::Parser;
use crossterm::event::EventStream;
use futures::StreamExt;
use ratatui::DefaultTerminal;
use std::path::PathBuf;

mod error;
mod input_results;
mod screen;
mod screens;
mod state;

use crate::{
    input_results::InputResult,
    screen::{Screen, ScreenStack},
    screens::{editor::Editor, show_error::ShowError, start::Start},
    state::AppState,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Directory to operate on (defaults to the current directory)
    #[arg(short, long, default_value = ".")]
    working_directory: PathBuf,
    /// Config file to be directly opened in the Editor (defaults to None)
    #[arg(short, long, default_value = None)]
    config_file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(args).run(terminal).await;
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
    pub fn new(args: Cli) -> Self {
        // setup screens, editor is the first one displayed
        App {
            event_stream: EventStream::default(),
            state: AppState::new(args),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // go directly to editor if valid config file is given, otherwise go to start screen.
        let mut current_screen: Box<dyn Screen> = match &self.state.initial_config_file.clone() {
            None => Start::new(),
            Some(file) => match Editor::try_from_file(&mut self.state, file) {
                Ok(editor) => editor,
                Err(err) => {
                    self.state.error_screen_state.add_error(format!("{err}"));
                    Start::new()
                }
            },
        };

        let mut screen_stack = ScreenStack::new();

        loop {
            // check for error(s) and display eventually
            if self.state.error_screen_state.has_unseen_errors() {
                self.state.error_screen_state.set_currently_shown_errors();
                screen_stack.push(current_screen);
                current_screen = ShowError::new();
            }

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
