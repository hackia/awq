use crate::widgets::awq::base::Component;
use crate::widgets::search_result::SearchResult;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Widget};
use ratatui::DefaultTerminal;

#[derive(Clone)]
pub struct SearchInput {
    label: String,
    value: String,
    is_active: bool,
    result: SearchResult,
}

impl Widget for SearchInput {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(self.label)
            .borders(Borders::ALL)
            .border_style(if self.is_active {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        block.render(area, buf);
        let input_area = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        buf.set_string(
            input_area.x,
            input_area.y,
            self.value.to_string(),
            Style::default().fg(Color::White),
        );
    }
}

impl Component for SearchInput {
    fn new() -> Self {
        Self {
            label: String::from(" Kōgnitara "),
            value: String::new(),
            is_active: false,
            result: SearchResult::new(),
        }
    }

    fn update(&mut self, new: Self) {
        self.label = new.label;
        self.value = new.value;
        self.is_active = new.is_active;
    }

    fn mount(&mut self, t: &mut DefaultTerminal) {
        assert!(t
            .draw(|frame| {
                let size = frame.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(size);
                frame.render_widget(self.clone(), chunks[0]);
                frame.render_widget(self.result.clone(), chunks[1]);
            })
            .is_ok());
    }

    fn handle(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter => {
                if self.is_active {
                    self.value.clear();
                    self.is_active = false;
                    self.result.active = true;
                }
            }
            KeyCode::Char('/') => {
                self.is_active = !self.is_active;
                self.result.active = !self.is_active;
            }
            KeyCode::Char(c) if self.is_active => {
                self.value.push(c);
            }
            KeyCode::Backspace if self.is_active => {
                self.value.pop();
            }
            _ => {}
        }
    }
}
