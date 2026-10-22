use crate::screen::Screen;

/// result of an input event to be handled directly by the App struct
pub enum InputResult {
    NoAction,
    QuitApplication,
    ReturnFromScreen,
    GoToScreen(Box<dyn Screen>),
}
