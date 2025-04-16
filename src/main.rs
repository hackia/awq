use crate::widgets::awq::base::Component;
use crate::widgets::search::SearchInput;
use crossterm::event::{self, Event};
use ratatui::{init, restore};

pub mod widgets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = init();

    // Main loop
    let mut search = SearchInput::new();
    loop {
        search.mount(&mut terminal);

        if let Event::Key(key) = event::read()? {
            let code = key.code;
            search.handle(code);
            if key.code.is_esc() {
                break;
            }
        }
    }
    restore();
    Ok(())
}
