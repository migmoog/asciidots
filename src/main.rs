use clap::Parser;
use dot::Point;
use dot_receivers::Receivers;
use std::{fs::read_to_string, path::PathBuf};

use grid::Grid;

mod dot;
mod dot_receivers;
mod grid;

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
    /* while asciidots.running {
        asciidots.tick();
    } */

    Ok(())
}

pub fn string_to_matrix(s: String) -> Vec<Vec<char>> {
    let mut out = Vec::<Vec<char>>::new();
    let lns = s.lines();
    let mut grid_width = 0;
    lns.clone()
        .for_each(|line| grid_width = std::cmp::max(grid_width, line.len()));

    for line in lns {
        let mut l_vec = Vec::<char>::new();
        for c in line.chars() {
            l_vec.push(c);
        }

        while l_vec.len() < grid_width {
            l_vec.push(' ');
        }

        out.push(l_vec);
    }

    out
}
