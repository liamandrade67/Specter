use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;
use std::time::Duration;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Enter => app.select(),
                    KeyCode::Esc => app.back(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
