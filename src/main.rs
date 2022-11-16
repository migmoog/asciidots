use clap::Parser;
use dot_receivers::Receivers;
use std::{fs::read_to_string, path::PathBuf};

use grid::Grid;

mod dot;
mod dot_receivers;
mod grid;
mod map;

/**
TODO
Arguments I want to add:
* --d/--draw: draw the dots on a map with an amount of ticks to sleep between each step
*/
#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let dots_content = match read_to_string(args.path) {
        Ok(content) => content,
        Err(e) => return Err(e.into()),
    };

    // These two variables are what make up asciidots, they work in tandem
    let mut grid = Grid::parse(dots_content);
    let mut recs = Receivers::new(&grid.ascii_art);
    'asciidots: loop {
        if !grid.running {
            break 'asciidots;
        }

        grid.tick();

        for (p, op) in recs.0.iter_mut() {
            match grid.receiver_check(op, *p) {
                Ok(_) => {}
                Err(_) => {
                    continue;
                }
            }
        }
        for (p, amp) in recs.1.iter_mut() {
            match grid.receiver_check(amp, *p) {
                Ok(_) => {
                    break 'asciidots;
                }
                Err(_) => {
                    continue;
                }
            }
        }
        for (p, dollar) in recs.2.iter_mut() {
            match grid.receiver_check(dollar, *p) {
                Ok(_) => {}
                Err(_) => {
                    continue;
                }
            }
        }
    }

    Ok(())
}
