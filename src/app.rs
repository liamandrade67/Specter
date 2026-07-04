use crate::modules;
use crate::screens::Screen;

pub struct App {
    pub screen: Screen,
    pub selected_category: usize,
    pub selected_tool: usize,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Dashboard,
            selected_category: 0,
            selected_tool: 0,
            running: true,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                let len = modules::categories().len();
                self.selected_category = (self.selected_category + 1) % len;
            }
            Screen::Category => {
                let len = modules::tools_for_category(self.selected_category).len();
                self.selected_tool = (self.selected_tool + 1) % len;
            }
            Screen::Tool => {}
        }
    }

    pub fn previous(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                let len = modules::categories().len();
                self.selected_category = (self.selected_category + len - 1) % len;
            }
            Screen::Category => {
                let len = modules::tools_for_category(self.selected_category).len();
                self.selected_tool = (self.selected_tool + len - 1) % len;
            }
            Screen::Tool => {}
        }
    }

    pub fn select(&mut self) {
        match self.screen {
            Screen::Dashboard => {
                self.screen = Screen::Category;
                self.selected_tool = 0;
            }
            Screen::Category => {
                self.screen = Screen::Tool;
            }
            Screen::Tool => {}
        }
    }

    pub fn back(&mut self) {
        match self.screen {
            Screen::Dashboard => {}
            Screen::Category => self.screen = Screen::Dashboard,
            Screen::Tool => self.screen = Screen::Category,
        }
    }
}
