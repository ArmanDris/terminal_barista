use crate::cup::scramble_cups;
use crate::cup::Cup;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Welcome,
    Main,
    Finished,
}

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub cups: Vec<Cup>,
    pub src_selection: Option<u32>,
    pub tooltip: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            cups: scramble_cups(),
            src_selection: None,
            tooltip: None,
        }
    }
}
