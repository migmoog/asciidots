use crate::dot::Point;
use std::cmp::{max, min};

type AsciiArt = Vec<Vec<char>>;

/// returns symbols in the order of (up, down, left, right)
pub fn surrounding_symbols(
    map: &AsciiArt,
    pos: Point,
) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
    let up = if pos.y <= 0 || map[pos.y - 1][pos.x] == ' ' {
        None
    } else {
        Some(map[pos.y - 1][pos.x])
    };

    let down = if pos.y >= map.len() - 1 || map[pos.y + 1][pos.x] == ' ' {
        None
    } else {
        Some(map[pos.y + 1][pos.x])
    };

    let left = if pos.x <= 0 || map[pos.y][pos.x - 1] == ' ' {
        None
    } else {
        Some(map[pos.y][pos.x - 1])
    };

    let right = if pos.x >= map[pos.y].len() - 1 || map[pos.y][pos.x + 1] == ' ' {
        None
    } else {
        Some(map[pos.y][pos.x + 1])
    };

    (up, down, left, right)
}

pub fn string_to_matrix(s: String) -> AsciiArt {
    let mut out = AsciiArt::new();
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

// TODO: send string instead of first bool
/// returns two bools: if the character is within quotes, and if so whether the direction is vertical or not
pub fn symbol_within_quote(map: &AsciiArt, pos: Point) -> (bool, Option<bool>) {
    let mut first_and_last: (Option<usize>, Option<usize>) = (None, None);

    // upward from character
    let top = max(pos.y as i32 - 1, 0) as usize;
    for y in top..=0 {
        if map[y][pos.x] == '"' {
            first_and_last.0 = Some(y);
            break;
        }
    }
    //downward from character
    let btm = min(pos.y + 1, map.len() - 1);
    for y in pos.y + 1..btm {
        if map[y][pos.x] == '"' {
            first_and_last.1 = Some(y);
            break;
        }
    }

    // is it vertical?
    if let (Some(top), Some(bottom)) = first_and_last {
        return (true, Some(true));
    }
    // if not then set it to nil
    first_and_last = (None, None);

    // leftward from character
    let left = max(pos.x as i32 - 1, 0) as usize;
    for x in left..=0 {
        if map[pos.y][x] == '"' {
            first_and_last.0 = Some(x);
            break;
        }
    }

    //rightward from character
    let right = min(pos.x + 1, map[pos.y].len() - 1);
    for x in right..map[pos.y].len() {
        if map[pos.y][x] == '"' {
            first_and_last.1 = Some(x);
            break;
        }
    }

    if let (Some(left), Some(right)) = first_and_last {
        return (true, Some(false));
    }

    (false, None)
}
