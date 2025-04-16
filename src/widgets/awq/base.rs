use crossterm::event::KeyCode;
use ratatui::widgets::Widget;
use ratatui::DefaultTerminal;
use reqwest::{Error, Response, Url};
use std::future::Future;

pub trait Component: Widget {
    fn new() -> Self;
    fn update(&mut self, new: Self) -> impl Future<Output = ()> + Send;
    fn mount(&mut self, t: &mut DefaultTerminal) -> impl Future<Output = ()> + Send;
    fn handle(&mut self, key: KeyCode) -> impl Future<Output = ()> + Send;
    fn get(&mut self, url: Url) -> impl Future<Output = Result<Response, Error>> + Send;
}
