use crate::widgets::awq::base::Component;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::{Color, Modifier, Style, Widget};
use ratatui::widgets::{Block, Borders};
use ratatui::DefaultTerminal;

#[derive(Clone)]
pub struct SearchResult {
    title: Vec<String>,
    description: Vec<String>,
    y: usize,
    pub active: bool,
}

impl Widget for SearchResult {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(" Breathes ")
            .borders(Borders::ALL)
            .border_style(if self.active {
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
            self.get_record(self.y).to_string(),
            Style::default().fg(Color::White),
        );
    }
}

impl Component for SearchResult {
    fn new() -> Self {
        Self {
            title: Vec::new(),
            description: Vec::new(),
            y: 1,
            active: false,
        }
    }

    fn update(&mut self, new: Self) {
        self.title = new.title;
        self.description = new.description;
    }

    fn mount(&mut self, t: &mut DefaultTerminal) {
        assert!(t
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(f.area());
                f.render_widget(self.clone(), chunks[1]);
            })
            .is_ok());
    }

    fn handle(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('/') => {
                self.active = false;
            }
            KeyCode::Down => {
                self.y += 1;
            }
            KeyCode::Up => {
                if self.y.gt(&0) {
                    self.y -= 1;
                }
            }
            KeyCode::Enter => {
                self.active = true;
            }
            _ => {
                self.active = false;
            }
        }
    }
}

impl SearchResult {
    pub fn get_record(&self, pos: usize) -> String {
        let t = self.title.get(pos).unwrap_or(&String::new()).to_string();
        let d = self
            .description
            .get(pos)
            .unwrap_or(&String::new())
            .to_string();
        format!("{t}\n\n{d}")
    }
}
