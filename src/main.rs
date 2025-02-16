use file::SaveFile;
use std::fs;

mod file;
mod names;
mod tui;

fn main() {
    let result = tui::run(ratatui::init(), {
        let bytes = fs::read("./a018.narc").expect("Should have been able to read the file");
        SaveFile::try_from(bytes.as_ref()).expect("Parsing to work")
    });
    ratatui::restore();
    save_to_file(result)
}

fn save_to_file(save_file: SaveFile) {
    std::io::Write::write_all(
        &mut fs::File::create_new("out.narc").unwrap(),
        &save_file.to_binary_format(),
    )
    .expect("Writing to file to work");
}
