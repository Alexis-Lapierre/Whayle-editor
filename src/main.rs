use file::{Move, SaveFile};
use names::POKE_NAMES;
use std::fs;

pub mod file;
pub mod names;

fn main() {
    let bytes = fs::read("./a018.narc").expect("Should have been able to read the file");
    let parsed = SaveFile::try_from(bytes.as_ref()).expect("Parsing to work");

    let args: Vec<String> = std::env::args().collect();
    match &args[1..] {
        [poke_id, add_rem, move_id, move_level] => {
            let (pid, pmove) = parse_args(poke_id, move_id, move_level).unwrap();
            match add_rem.as_str() {
                "add" => add_move_to_pokemon(parsed, pid, pmove),

                "del" | "rem" => remove_move_from_pokemon(parsed, pid, pmove),
                _ => panic!(
                    "Unexpected second argument: expected “add” or “del” but got “{add_rem}”"
                ),
            }
        }
        [poke_id] => {
            show_all_moves_for_pokemon(&parsed, poke_id);
        }
        [] => {
            show_all(&parsed);
        }
        _ => todo!("Handle this branch"),
    }
}

fn parse_args(poke_id: &str, move_id: &str, move_level: &str) -> Result<(u16, Move), &'static str> {
    let poke_id: u16 = poke_id
        .parse()
        .map_err(|_| "First argument must be a valid Pokemon ID")?;

    let poke_id = poke_id
        .checked_sub(1)
        .ok_or("First argument must be greater than 0")?;

    let id: u16 = move_id
        .parse()
        .map_err(|_| "Third argument must be a valid move ID")?;

    let level: u16 = move_level
        .parse()
        .map_err(|_| "Fourth argument must be a valid level")?;

    Ok((poke_id, Move::new(id, level)))
}

fn insert_move_sorted(moves: &mut Vec<Move>, pmove: Move) {
    let insert_pos = moves
        .iter()
        .position(|m| m.level > pmove.level)
        .unwrap_or(moves.len());

    moves.insert(insert_pos, pmove);
}

fn add_move_to_pokemon(mut save_file: SaveFile, poke_id: u16, pmove: Move) {
    let moves = &mut save_file.pokemons[usize::from(poke_id)];

    insert_move_sorted(moves, pmove);

    println!(
        "Inserted {} for Pokemon {}",
        pmove,
        POKE_NAMES[usize::from(poke_id)]
    );

    save_to_file(save_file);
}

fn remove_move_if_exists(moves: &mut Vec<Move>, pmove: Move) -> bool {
    let original_length = moves.len();
    moves.retain(|m| *m != pmove);
    moves.len() < original_length
}

fn remove_move_from_pokemon(mut save_file: SaveFile, poke_id: u16, pmove: Move) {
    let moves = &mut save_file.pokemons[usize::from(poke_id)];

    if remove_move_if_exists(moves, pmove) {
        println!(
            "Removed {} from Pokemon {}",
            pmove,
            POKE_NAMES[usize::from(poke_id)]
        );
        save_to_file(save_file);
    } else {
        eprintln!(
            "{} was not found for Pokemon {}",
            pmove,
            POKE_NAMES[usize::from(poke_id)]
        );
    }
}

fn show_all_moves_for_pokemon(parsed: &SaveFile, poke_id: &str) {
    let mut pokemons = POKE_NAMES.iter().zip(parsed.pokemons.iter());
    let poke_id: u16 = poke_id.parse().expect("First argument to be Pokemon ID");
    assert!(poke_id != 0);

    let (name, moves) = pokemons.nth(usize::from(poke_id - 1)).unwrap();
    println!("Pokemon ID {poke_id} - {name}");
    for m in moves {
        println!("{m}");
    }
}

fn show_all(parsed: &SaveFile) {
    for (name, moves) in POKE_NAMES.iter().zip(parsed.pokemons.iter()) {
        println!("{name}: {moves:?}");
    }
}

fn save_to_file(save_file: SaveFile) {
    std::io::Write::write_all(
        &mut fs::File::create_new("out.narc").unwrap(),
        &save_file.to_binary_format(),
    )
    .expect("Writing to file to work");
}
