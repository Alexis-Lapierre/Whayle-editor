use std::any::Any;

use crate::{
    file::{Move, SaveFile},
    names::POKE_NAMES,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, HighlightSpacing, Row, Table, TableState},
    Frame,
};

pub struct App {
    current_pokemon: usize,
    save_file: SaveFile,
    selection: Selection,
}

enum Selection {
    Pokemon,
    Move(usize),
}

impl App {
    pub fn from_save_file(save_file: SaveFile) -> Self {
        debug_assert!(save_file.pokemons.len() == POKE_NAMES.len());
        Self {
            current_pokemon: 0,
            selection: Selection::Pokemon,
            save_file,
        }
    }
    pub const fn next_pokemon(&mut self) {
        self.current_pokemon += 1;
        if self.current_pokemon >= POKE_NAMES.len() {
            self.current_pokemon = 0;
        }
    }
    pub const fn previous_pokemon(&mut self) {
        self.current_pokemon = match self.current_pokemon.checked_sub(1) {
            Some(value) => value,
            None => POKE_NAMES.len() - 1,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = self.layout().split(frame.area());
        frame.render_stateful_widget(self.pokemon_table(), layout[0], &mut self.pokemon_state());
        frame.render_stateful_widget(self.move_table(), layout[1], &mut self.move_state());
    }

    fn layout(&self) -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
    }

    fn pokemon_table(&self) -> Table<'_> {
        let rows = (1..)
            .zip(POKE_NAMES)
            .map(|(id, name)| Row::new([id.to_string(), name.to_string()]));
        let widths = [Constraint::Length(5), Constraint::Length(25)];
        Table::new(rows, widths)
            .header(Row::new(["ID", "Name"]))
            .block(Block::bordered().title("Pokemons"))
            .row_highlight_style(Style::new().reversed())
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
    }

    fn pokemon_state(&self) -> TableState {
        TableState::default().with_selected(self.current_pokemon)
    }

    fn move_table(&self) -> Table<'_> {
        let moves = self.get_moves().iter().map(|pmove| {
            Row::new([
                pmove.id.to_string(),
                pmove.name().to_string(),
                pmove.level.to_string(),
            ])
        });
        let widths = [
            Constraint::Length(5),
            Constraint::Length(20),
            Constraint::Length(5),
        ];
        Table::new(moves, widths)
            .header(Row::new(["ID", "Name", "Level"]))
            .block(Block::bordered().title("Moves"))
            .row_highlight_style(Style::new().reversed())
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn move_state(&self) -> TableState {
        match self.selection {
            Selection::Move(index) => TableState::default().with_selected(index),
            Selection::Pokemon => TableState::default(),
        }
    }

    fn get_moves(&self) -> &[Move] {
        debug_assert!(self.current_pokemon < self.save_file.pokemons.len());
        &self.save_file.pokemons[self.current_pokemon]
    }
}
