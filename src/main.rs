const END_OF_POKEMON: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn list_pokemons(bytes: &[u8]) -> Vec<Vec<Move>> {
    do_list_pokemon(bytes, Vec::new())
}
fn do_list_pokemon(bytes: &[u8], mut acc: Vec<Vec<Move>>) -> Vec<Vec<Move>> {
    match find_subsequence(bytes, &END_OF_POKEMON) {
        Some(index) => {
            let (found, remainder) = bytes.split_at(index);
            acc.push(
                found
                    .chunks(4)
                    .map(|chunk| Move::from(&chunk.try_into().unwrap()))
                    .collect(),
            );
            do_list_pokemon(&remainder[4..], acc)
        }
        None => acc,
    }
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
        let ignored = find_subsequence(&bytes, &END_OF_POKEMON)
            .expect("At least one pokemon delimited in file");
        &bytes[ignored..]
    };

    for (id, pokemon) in list_pokemons(poke_bytes).iter().enumerate() {
        println!("{id}: {pokemon:?}");
    }
}
