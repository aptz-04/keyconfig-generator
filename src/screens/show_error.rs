use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, StatefulWidget, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{InputResult, Screen, state::AppState};

pub mod state;
use state::ErrorType;
pub use state::ShowErrorState;

struct ScrollableErrorWidget<'a> {
    errors: &'a [ErrorType],
}

impl<'a> StatefulWidget for ScrollableErrorWidget<'a> {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let content = self
            .errors
            .iter()
            .map(|err| err.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let content_paragraph = Paragraph::new(content).wrap(Wrap { trim: true });
        let content_size = Size::new(
            area.width,
            content_paragraph.line_count(area.width).try_into().unwrap(),
        );
        let mut scroll_view = ScrollView::new(content_size)
            .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

        let content_position = Rect::new(0, 0, content_size.width, content_size.height);
        scroll_view.render_widget(content_paragraph, content_position);
        scroll_view.render(area, buf, state);
    }
}

pub struct ShowError {}

impl Screen for ShowError {
    fn draw(&self, state: &mut AppState, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| self.draw_internal(state, frame));
    }

    fn handle_crossterm_events(&mut self, state: &mut AppState, event: Event) -> InputResult {
        let scroll_state = state.error_screen_state.get_scrollview_state();

        match event {
            Event::Key(key) if key.is_press() => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return InputResult::ReturnFromScreen,
                KeyCode::Char('j') | KeyCode::Down => scroll_state.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => scroll_state.scroll_up(),
                _ => (),
            },
            _ => (),
        }

        InputResult::NoAction
    }
}

impl ShowError {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn draw_internal(&self, state: &mut AppState, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Fill(1),
            Constraint::Percentage(20),
        ])
        .horizontal_margin(10);
        let [_, error_area, _] = frame.area().layout(&layout);
        let error_block = Block::bordered().title(
            Line::from(" Errors occurred! ")
                .left_aligned()
                .red()
                .on_white(),
        );
        let inner_area = error_block.inner(error_area);
        frame.render_widget(error_block, error_area);

        let (error_list, view_state) = state.error_screen_state.get_error_scroll_resources();

        let widget = ScrollableErrorWidget { errors: error_list };
        frame.render_stateful_widget(widget, inner_area, view_state);
    }
}
