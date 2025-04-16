use crate::widgets::awq::base::Component;
use crate::widgets::search::SearchInput;
use crossterm::event::{self, Event};
use ratatui::{init, restore};
use std::time::Duration;

pub mod widgets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = init();

    // Main loop
    let mut search = SearchInput::new();
    loop {
        search.mount(&mut terminal).await;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                search.handle(key.code).await;
                if key.code.is_esc() {
                    break;
                }
            }
        }
    }
    restore();
    Ok(())
}
