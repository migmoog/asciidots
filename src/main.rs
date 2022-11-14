use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

use grid::Grid;

mod dot;
mod dot_receivers;
mod grid;
mod tokens;

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
    let dots_content = read_to_string(args.path).expect("Couldn't find file");

    let mut grid_width = 0;
    let content_lines = dots_content.lines();
    content_lines.clone().for_each(|line| {
        grid_width = std::cmp::max(grid_width, line.len());
    });

    let mut dots = String::new();
    for line in content_lines {
        let mut add_to_dots = line.to_string();
        println!("{add_to_dots}");

        while add_to_dots.len() < grid_width {
            add_to_dots.push(' ');
        }

        add_to_dots.push('\n');
        dots.push_str(add_to_dots.as_str());
    }
    let mut dots_grid = Grid::parse(dots);

    Ok(())
}
