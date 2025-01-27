pub mod names;
use names::POKE_NAMES;

pub mod file;

fn main() {
    let bytes = std::fs::read("./a018.narc").expect("Should have been able to read the file");
    let parsed = file::SaveFile::try_from(bytes.as_ref()).expect("Parsing to work");
    let mut pokemons = POKE_NAMES.iter().zip(parsed.pokemons);

    let args: Vec<String> = std::env::args().collect();
    match &args[..] {
        [_name, poke_id, ..] => {
            let poke_id: u16 = poke_id.parse().expect("First argument to be pokemon ID");
            assert!(poke_id != 0);
            let (name, moves) = pokemons.nth(usize::from(poke_id - 1)).unwrap();
            println!("Pokemon ID {poke_id} - {name}");
            for m in moves {
                println!("{m}");
            }
        }
        _ => {
            for (name, moves) in pokemons {
                println!("{name}: {moves:?}");
            }
        }
    }
}
