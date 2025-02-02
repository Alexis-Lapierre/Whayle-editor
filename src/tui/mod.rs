use app::App;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};

use crate::file::SaveFile;

mod app;

pub fn run(mut terminal: DefaultTerminal, mut save_file: SaveFile) {
    let mut app = App::from_save_file(save_file);
    loop {
        terminal.draw(|frame| app.render(frame)).unwrap();
        match event::read().unwrap() {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Up => {
                    app.previous_pokemon();
                }
                KeyCode::Down => {
                    app.next_pokemon();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
