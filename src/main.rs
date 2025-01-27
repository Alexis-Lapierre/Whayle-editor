pub mod names;
use std::io::Write;

use file::Move;
use names::POKE_NAMES;

pub mod file;

fn main() {
    let bytes = std::fs::read("./a018.narc").expect("Should have been able to read the file");
    let parsed = file::SaveFile::try_from(bytes.as_ref()).expect("Parsing to work");

    let args: Vec<String> = std::env::args().collect();
    match &args[1..] {
        [poke_id, add, move_id, move_level] if add == "add" => {
            let mut save_file = parsed;
            let poke_id: u16 = poke_id.parse().expect("First argument to be pokemon ID");
            let poke_id = poke_id - 1;
            let pmove = {
                let id: u16 = move_id.parse().expect("Third argument to be move ID");
                let level: u16 = move_level.parse().expect("Forth argument to be level");
                Move::new(id, level)
            };
            println!(
                "Wrote {} for Pokemon {}",
                pmove,
                POKE_NAMES[usize::from(poke_id)]
            );
            save_file.pokemons[usize::from(poke_id)].push(pmove);
            std::fs::File::create_new("out.narc")
                .unwrap()
                .write_all(&save_file.to_binary_format())
                .expect("Writing to file to work");
        }
        [poke_id] => {
            let mut pokemons = POKE_NAMES.iter().zip(parsed.pokemons);
            let poke_id: u16 = poke_id.parse().expect("First argument to be pokemon ID");
            assert!(poke_id != 0);
            let (name, moves) = pokemons.nth(usize::from(poke_id - 1)).unwrap();
            println!("Pokemon ID {poke_id} - {name}");
            for m in moves {
                println!("{m}");
            }
        }
        [] => {
            for (name, moves) in POKE_NAMES.iter().zip(parsed.pokemons) {
                println!("{name}: {moves:?}");
            }
        }
        _ => todo!("Handle this branch"),
    }
}
