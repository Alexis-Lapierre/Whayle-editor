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
    pub save_file: SaveFile,
    gui_state: State,
}

struct State {
    current_pokemon: usize,
    selected: Selected,
}

#[derive(PartialEq, Eq)]
enum Selected {
    Pokemon,
    Move(usize),
}

impl App {
    pub fn from_save_file(save_file: SaveFile) -> Self {
        debug_assert!(save_file.pokemons.len() == POKE_NAMES.len());
        Self {
            gui_state: State {
                current_pokemon: 0,
                selected: Selected::Pokemon,
            },
            save_file,
        }
    }

    pub fn select_next(&mut self) {
        self.gui_state.next(self.get_moves().len());
    }

    pub fn select_previous(&mut self) {
        self.gui_state.previous(self.get_moves().len());
    }
    pub fn select_right(&mut self) {
        self.gui_state.select_moves();
    }
    pub fn select_left(&mut self) {
        self.gui_state.select_pokemon();
    }

    pub fn delete_move_selected(&mut self) {
        match self.gui_state.selected {
            Selected::Pokemon => todo!("Handle delete when no move is selected, AKA do nothing."),
            Selected::Move(index) => {
                self.get_moves_mut().remove(index);
                self.gui_state.selected = match self.get_moves().len() {
                    0 => Selected::Pokemon, // todo handle empty move set...
                    n if index >= n => Selected::Move(n - 1),
                    _ => Selected::Move(index),
                }
            }
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
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    }

    fn pokemon_table(&self) -> Table<'_> {
        let rows = (1..)
            .zip(POKE_NAMES)
            .map(|(id, name)| Row::new([id.to_string(), name.to_string()]));
        let widths = [Constraint::Length(5), Constraint::Length(25)];
        let table = Table::new(rows, widths)
            .header(Row::new(["ID", "Name"]))
            .block(Block::bordered().title("Pokemons"))
            .row_highlight_style(Style::new().reversed())
            .highlight_spacing(HighlightSpacing::Always);

        if self.gui_state.selected == Selected::Pokemon {
            table.highlight_symbol(">>")
        } else {
            table
        }
    }

    fn pokemon_state(&self) -> TableState {
        TableState::default().with_selected(self.gui_state.current_pokemon)
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
        let table = Table::new(moves, widths)
            .header(Row::new(["ID", "Name", "Level"]))
            .block(Block::bordered().title("Moves"))
            .row_highlight_style(Style::new().reversed())
            .highlight_spacing(HighlightSpacing::Always);

        if let Selected::Move(_) = self.gui_state.selected {
            table.highlight_symbol(">>")
        } else {
            table
        }
    }

    fn move_state(&self) -> TableState {
        match self.gui_state.selected {
            Selected::Move(index) => {
                debug_assert!(index < self.get_moves().len());
                TableState::default().with_selected(index)
            }
            Selected::Pokemon => TableState::default(),
        }
    }

    fn get_moves(&self) -> &[Move] {
        debug_assert!(self.gui_state.current_pokemon < self.save_file.pokemons.len());
        &self.save_file.pokemons[self.gui_state.current_pokemon]
    }

    fn get_moves_mut(&mut self) -> &mut Vec<Move> {
        debug_assert!(self.gui_state.current_pokemon < self.save_file.pokemons.len());
        &mut self.save_file.pokemons[self.gui_state.current_pokemon]
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_pokemon: 0,
            selected: Selected::Pokemon,
        }
    }
}

impl State {
    const fn next(&mut self, max_move_len: usize) {
        if let Selected::Move(index) = self.selected {
            let index = index + 1;
            self.selected = Selected::Move(if index >= max_move_len { 0 } else { index })
        } else {
            self.next_pokemon();
        }
    }
    fn previous(&mut self, max_move_len: usize) {
        if let Selected::Move(index) = self.selected {
            self.selected = Selected::Move(index.checked_sub(1).unwrap_or(max_move_len - 1));
        } else {
            self.previous_pokemon();
        }
    }
    const fn next_pokemon(&mut self) {
        self.current_pokemon += 1;
        if self.current_pokemon >= POKE_NAMES.len() {
            self.current_pokemon = 0;
        }
    }

    const fn previous_pokemon(&mut self) {
        self.current_pokemon = match self.current_pokemon.checked_sub(1) {
            Some(value) => value,
            None => POKE_NAMES.len() - 1,
        }
    }
    const fn select_pokemon(&mut self) {
        self.selected = Selected::Pokemon;
    }

    const fn select_moves(&mut self) {
        self.selected = Selected::Move(0);
    }
}
