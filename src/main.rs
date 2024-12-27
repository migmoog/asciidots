use std::{fs::File, io::Read};

mod grid;
mod receivers;
mod dot;
mod context;

#[derive(Debug)]
enum StartError {
    NonexistentFile,
    CouldntRead,
}

fn lex_grid(grid: Vec<u8>, width: usize, height: usize) {

}

fn main() -> Result<(), StartError> {
    let args: Vec<String> = std::env::args().collect();
    let mut src_file = match File::open(&args[0]) {
        Ok(f) => f,
        Err(_) => return Err(StartError::NonexistentFile)
    };

    let mut grid: Vec<u8> = Vec::new();
    if let Err(_) = src_file.read_to_end(&mut grid) {
        return Err(StartError::CouldntRead);
    }

    let width = grid.iter()
        .position(|&r| r == '\n' as u8)
        .unwrap_or_else(|| grid.len());
    grid = grid.into_iter().filter(|&r| r != '\n' as u8).collect();
    let height = grid.len() / width;

    Ok(())
}
