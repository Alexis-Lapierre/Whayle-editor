use std::fmt::Display;

use crate::names::move_id_to_name;

pub const POKEMON_DELIMITER: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

pub struct SaveFile {
    file_start: Box<[u8]>,
    pub pokemons: Vec<Vec<Move>>,
}

impl SaveFile {
    #[must_use]
    pub fn to_binary_format(self) -> Vec<u8> {
        let mut result = Vec::from(self.file_start);
        result.extend_from_slice(&POKEMON_DELIMITER);
        for pokemon in self.pokemons {
            for m in pokemon {
                let into: [u8; 4] = m.into();
                result.extend_from_slice(&into);
            }
            result.extend_from_slice(&POKEMON_DELIMITER);
        }

        result
    }
}

impl TryFrom<&[u8]> for SaveFile {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (file_start, pokemon) = split(value, &POKEMON_DELIMITER).ok_or(())?;
        let file_start = Box::from(file_start);

        let pokemons: Vec<Vec<Move>> = {
            list_pokemon_data(pokemon).map(|pokemon_data| {
                pokemon_data
                    .chunks(4)
                    .map(|chunk| Move::from(&chunk.try_into().unwrap()))
                    .collect()
            })
        }
        .collect();

        Ok(Self {
            file_start,
            pokemons,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub id: u16,
    pub level: u16,
}

impl Move {
    #[must_use]
    pub const fn new(id: u16, level: u16) -> Self {
        Self { id, level }
    }
}

impl From<&[u8; 4]> for Move {
    fn from(value: &[u8; 4]) -> Self {
        Self {
            id: u16::from_le_bytes([value[0], value[1]]),
            level: u16::from_le_bytes([value[2], value[3]]),
        }
    }
}

impl From<Move> for [u8; 4] {
    fn from(value: Move) -> Self {
        let id = value.id.to_le_bytes();
        let level = value.level.to_le_bytes();
        [id[0], id[1], level[0], level[1]]
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  - {:15} at level {:3}",
            move_id_to_name(self.id.into()),
            self.level
        )
    }
}

fn split<'a, T>(haystack: &'a [T], needle: &'_ [T]) -> Option<(&'a [T], &'a [T])>
where
    for<'b> &'b [T]: PartialEq,
{
    let needle_position = haystack
        .windows(needle.len())
        .position(|window| window == needle);

    needle_position.map(|position| haystack.split_at(position))
}

fn list_pokemon_data(data: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut current_remaining_data = &data[POKEMON_DELIMITER.len()..];
    std::iter::from_fn(move || {
        split(current_remaining_data, &POKEMON_DELIMITER).map(|(pokemon_data, remaining)| {
            current_remaining_data = &remaining[POKEMON_DELIMITER.len()..];
            pokemon_data
        })
    })
}
