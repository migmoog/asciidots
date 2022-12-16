use crate::dot::{Direction, Point};
use std::cmp::max;

type AsciiArt = Vec<Vec<char>>;

/// returns symbols in the order of (up, down, left, right)
pub fn surrounding_symbols(map: &AsciiArt, pos: Point) -> [Option<(char, Point)>; 4] {
    let up = if pos.y <= 0 || map[pos.y - 1][pos.x] == ' ' {
        None
    } else {
        let p = Point {
            y: pos.y - 1,
            ..pos
        };
        Some((map[p.y][p.x], p))
    };

    let down = if pos.y >= map.len() - 1 || map[pos.y + 1][pos.x] == ' ' {
        None
    } else {
        let p = Point {
            y: pos.y + 1,
            ..pos
        };
        Some((map[p.y][p.x], p))
    };

    let left = if pos.x <= 0 || map[pos.y][pos.x - 1] == ' ' {
        None
    } else {
        let p = Point {
            x: pos.x - 1,
            ..pos
        };
        Some((map[p.y][p.x], p))
    };

    let right = if pos.x >= map[pos.y].len() - 1 || map[pos.y][pos.x + 1] == ' ' {
        None
    } else {
        let p = Point {
            x: pos.x + 1,
            ..pos
        };
        Some((map[p.y][p.x], p))
    };

    [up, down, left, right]
}

pub fn string_to_matrix(s: String) -> AsciiArt {
    let mut out = AsciiArt::new();
    let lns = s.lines();
    let mut grid_width = 0;

    for line in lns {
        grid_width = max(grid_width, line.len());

        let mut l_vec = Vec::<char>::new();
        for c in line.chars() {
            l_vec.push(c);
        }

        out.push(l_vec);
    }

    for l_vec in out.iter_mut() {
        while l_vec.len() < grid_width {
            l_vec.push(' ');
        }
    }

    out
}

pub fn symbol_within_quote(map: &AsciiArt, pos: Point) -> bool {
    let mut first_and_last: [Option<usize>; 2] = [None, None];

    // upward from character
    let mut upward = pos.y;
    while upward > 0 {
        if map[upward][pos.x] == '"' {
            first_and_last[0] = Some(upward);
            break;
        }

        upward -= 1;
    }

    //downward from character
    let mut downward = pos.y;
    while downward < map.len() {
        if map[downward][pos.x] == '"' {
            first_and_last[1] = Some(downward);
            break;
        }

        downward += 1;
    }

    // is it vertical?
    if let [Some(_), Some(_)] = first_and_last {
        println!("POO");
        println!("{:?})", pos);
        return true;
    }
    // if not then set it to nil
    first_and_last = [None, None];

    // leftward from character
    let mut leftward = pos.x;
    while leftward > 0 {
        if map[pos.y][leftward] == '"' {
            first_and_last[0] = Some(leftward);
            break;
        }

        leftward -= 1;
    }

    //rightward from character
    let mut rightward = pos.x;
    while rightward < map[pos.y].len() {
        if map[pos.y][rightward] == '"' {
            first_and_last[1] = Some(rightward);
            break;
        }

        rightward += 1;
    }

    if let [Some(_), Some(_)] = first_and_last {
        return true;
    }

    // (false, None)
    false
}

pub fn get_quote(map: &AsciiArt, mut quote_pos: Point, dir: Direction) -> String {
    let mut out = "".to_string();
    match dir {
        Direction::Up => {
            quote_pos.y -= 1;
            while map[quote_pos.y][quote_pos.x] != '"' {
                out.push(map[quote_pos.y][quote_pos.x]);
                quote_pos.y -= 1;
            }
        }
        Direction::Down => {
            quote_pos.y += 1;
            while map[quote_pos.y][quote_pos.x] != '"' {
                out.push(map[quote_pos.y][quote_pos.x]);
                quote_pos.y += 1;
            }
        }
        Direction::Left => {
            quote_pos.x -= 1;
            while map[quote_pos.y][quote_pos.x] != '"' {
                out.push(map[quote_pos.y][quote_pos.x]);
                quote_pos.x -= 1;
            }
        }
        Direction::Right => {
            quote_pos.x += 1;
            while map[quote_pos.y][quote_pos.x] != '"' {
                out.push(map[quote_pos.y][quote_pos.x]);
                quote_pos.x += 1;
            }
        }
    }

    out
}
