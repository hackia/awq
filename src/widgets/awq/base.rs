use crossterm::event::KeyCode;
use ratatui::widgets::Widget;
use ratatui::DefaultTerminal;

pub trait Component: Widget {
    fn new() -> Self;
    fn update(&mut self, new: Self);
    fn mount(&mut self, t: &mut DefaultTerminal);
    fn handle(&mut self, key: KeyCode);
}
