pub mod names;
use names::{move_id_to_name, POKE_NAMES};

const DELIMITER_LEN: usize = 4;
const POKEMON_DELIMITER: [u8; DELIMITER_LEN] = [0xff, 0xff, 0xff, 0xff];

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn list_pokemons(bytes: &[u8]) -> impl Iterator<Item = impl Iterator<Item = Move> + use<'_>> {
    let mut current_remaining_data = bytes;
    std::iter::from_fn(move || {
        find_subsequence(current_remaining_data, &POKEMON_DELIMITER).map(|position| {
            let (pokemon_data, remaining) = current_remaining_data.split_at(position);
            current_remaining_data = &remaining[DELIMITER_LEN..];
            pokemon_data
                .chunks(4)
                .map(|chunk| Move::from(&chunk.try_into().unwrap()))
        })
    })
}

#[derive(Debug)]
struct Move {
    id: u16,
    level: u16,
}

impl From<&[u8; 4]> for Move {
    fn from(value: &[u8; 4]) -> Self {
        Self {
            id: u16::from_le_bytes([value[0], value[1]]),
            level: u16::from_le_bytes([value[2], value[3]]),
        }
    }
}

fn main() {
    let bytes = std::fs::read("./a018.narc").expect("Should have been able to read the file");
    let poke_bytes = {
        let ignored = find_subsequence(&bytes, &POKEMON_DELIMITER)
            .expect("At least one pokemon delimited in file");
        &bytes[(ignored + DELIMITER_LEN)..]
    };
    let pokemons = POKE_NAMES.iter().zip(list_pokemons(poke_bytes));

    let args: Vec<String> = std::env::args().collect();
    match &args[..] {
        [_name, poke_id, ..] => {
            let poke_id: u16 = poke_id.parse().expect("First argument to be pokemon ID");
            assert!(poke_id != 0);
            let (name, moves) = pokemons.skip(usize::from(poke_id - 1)).next().unwrap();
            println!("Pokemon: {name}");
            for m in moves {
                println!("  - {:15} at {:3}", move_id_to_name(m.id.into()), m.level);
            }
        }
        _ => {
            for (name, moves) in pokemons {
                let pokemon: Vec<String> = moves
                    .map(|e| format!("{} at {}", move_id_to_name(e.id.into()), e.level))
                    .collect();
                println!("{name}: {pokemon:?}");
            }
        }
    }
}
