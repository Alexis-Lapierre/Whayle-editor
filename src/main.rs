use file::{Move, SaveFile};
use names::POKE_NAMES;
use std::fs;
use std::io::Write;

pub mod file;
pub mod names;

fn main() {
    let bytes = fs::read("./a018.narc").expect("Should have been able to read the file");
    let parsed = SaveFile::try_from(bytes.as_ref()).expect("Parsing to work");

    let args: Vec<String> = std::env::args().collect();
    match &args[1..] {
        [poke_id, add, move_id, move_level] if add == "add" => {
            add_move_to_pokemon(parsed, poke_id, move_id, move_level);
        }
        [poke_id, rem, move_id, move_level] if rem == "rem" => {
            remove_move_from_pokemon(parsed, poke_id, move_id, move_level);
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

fn add_move_to_pokemon(mut save_file: SaveFile, poke_id: &str, move_id: &str, move_level: &str) {
    let poke_id: u16 = poke_id.parse().expect("First argument to be Pokemon ID");
    let poke_id = poke_id - 1;
    let pmove = {
        let id: u16 = move_id.parse().expect("Third argument to be move ID");
        let level: u16 = move_level.parse().expect("Fourth argument to be level");
        Move::new(id, level)
    };

    println!(
        "Wrote {} for Pokemon {}",
        pmove,
        POKE_NAMES[usize::from(poke_id)]
    );

    save_file.pokemons[usize::from(poke_id)].push(pmove);
    save_to_file(save_file);
}

fn remove_move_from_pokemon(
    mut save_file: SaveFile,
    poke_id: &str,
    move_id: &str,
    move_level: &str,
) {
    let poke_id: u16 = poke_id.parse().expect("First argument to be Pokemon ID");
    let poke_id = poke_id - 1;
    let pmove = {
        let id: u16 = move_id.parse().expect("Third argument to be move ID");
        let level: u16 = move_level.parse().expect("Fourth argument to be level");
        Move::new(id, level)
    };

    let moves = &mut save_file.pokemons[usize::from(poke_id)];
    let original_length = moves.len();
    moves.retain(|m| *m != pmove);

    if moves.len() < original_length {
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
    fs::File::create_new("out.narc")
        .unwrap()
        .write_all(&save_file.to_binary_format())
        .expect("Writing to file to work");
}
