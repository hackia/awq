use crate::widgets::awq::base::Component;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Widget};
use ratatui::DefaultTerminal;
use reqwest::{get, Error, Response, Url};
use std::future::Future;

#[derive(Clone)]
pub struct SearchInput {
    label: String,
    value: String,
    is_active: bool,
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
        }
    }

    async fn update(&mut self, new: Self) {
        self.label = new.label;
        self.value = new.value;
        self.is_active = new.is_active;
    }

    async fn mount(&mut self, t: &mut DefaultTerminal) {
        assert!(t
            .draw(|frame| {
                let size = frame.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(size);
                frame.render_widget(self.clone(), chunks[0]);
            })
            .is_ok());
    }

    async fn handle(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter => {
                if self.is_active {
                    let url = format!(
                        "http://127.0.0.1:4213/breathes/search/{}",
                        self.value.replace(" ", "-").replace("_", "-")
                    );

                    let res = self
                        .get(Url::parse(url.as_str()).expect("invalid url"))
                        .await
                        .expect("no response");
                    println!("{res:?}");
                    self.value.clear();
                    self.is_active = false;
                }
            }
            KeyCode::Char('/') => {
                self.is_active = !self.is_active;
            }
            KeyCode::Char(c) if self.is_active => {
                self.value.push(c);
                self.update(self.clone()).await;
            }
            KeyCode::Backspace if self.is_active => {
                self.value.pop();
                self.update(self.clone()).await;
            }
            _ => {}
        }
    }

    fn get(&mut self, url: Url) -> impl Future<Output = Result<Response, Error>> + Send {
        get(url.clone())
    }
}
