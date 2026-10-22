use std::ops::RangeFrom;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, StatefulWidget, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::{InputResult, Screen, state::AppState};

struct ScrollableErrorWidget {
    error_list: Vec<String>,
}

impl StatefulWidget for ScrollableErrorWidget {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let content = self.error_list.join("\n");

        let content_paragraph = Paragraph::new(content).wrap(Wrap { trim: true });
        let content_size = Size::new(
            area.width - 1,
            content_paragraph
                .line_count(area.width - 1)
                .try_into()
                .unwrap(),
        );
        let mut scroll_view = ScrollView::new(content_size);

        let content_position = Rect::new(0, 0, area.width, area.height);
        scroll_view.render_widget(content_paragraph, content_position);
        scroll_view.render(area, buf, state);
    }
}

pub struct ShowErrorScreen {}

pub struct ShowErrorScreenState {
    scroll_view_state: ScrollViewState,
    errors: Vec<String>,
    new_errors: RangeFrom<usize>,
    currently_shown_errors: Vec<String>,
}

impl ShowErrorScreenState {
    pub fn new() -> Self {
        ShowErrorScreenState {
            scroll_view_state: ScrollViewState::new(),
            errors: Vec::<String>::new(),
            new_errors: RangeFrom { start: 0 },
            currently_shown_errors: Vec::<String>::new(),
        }
    }

    pub fn has_unseen_errors(&self) -> bool {
        self.errors.len() > self.new_errors.start
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    /// sets the currently shown errors to the range of unseen values
    pub fn set_currently_shown_errors(&mut self) {
        let unseen_range = &self.errors[self.new_errors.clone()];
        self.new_errors = self.errors.len()..;

        self.currently_shown_errors = unseen_range.into()
    }

    pub fn view_state(&mut self) -> &mut ScrollViewState {
        &mut self.scroll_view_state
    }
}

impl Screen for ShowErrorScreen {
    fn draw(&self, state: &mut AppState, terminal: &mut DefaultTerminal) {
        let _ = terminal.draw(|frame| self.draw_internal(state, frame));
    }

    fn handle_crossterm_events(&mut self, state: &mut AppState, event: Event) -> InputResult {
        let scroll_state = &mut state.error_screen_state.scroll_view_state;

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

impl ShowErrorScreen {
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

        let widget = ScrollableErrorWidget {
            error_list: state.error_screen_state.currently_shown_errors.clone(),
        };
        frame.render_stateful_widget(widget, inner_area, state.error_screen_state.view_state());
    }
}
