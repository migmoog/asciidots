use crate::{dot::Dot, grid::Direction};

pub fn asterix(d: Dot, edges: Vec<Direction>) -> Vec<Dot> {
    let mut out = Vec::new();
    let Dot {
        pos,
        value,
        trajectory,
    } = d;
    for dir in edges {
        if dir == trajectory {
            continue;
        }

        let d = Dot {
            pos: pos.clone() + dir,
            value,
            trajectory: dir,
        };
        out.push(d);
    }
    out
}

#[cfg(test)]
mod receivers_tests {
    use crate::grid::Point;

    use super::*;

    fn d(p: (u32, u32), t: Direction) -> Dot {
        Dot {
            pos: Point::from(p),
            value: 1.0,
            trajectory: t,
        }
    }

    #[test]
    fn test_asterix() {
        let d = d((3, 3), Direction::East);
        let edges = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let resulting_points = asterix(d, edges)
            .into_iter()
            .map(|d| d.pos)
            .collect::<Vec<Point>>();
        assert_eq!(
            vec![
                Point::from((3, 2)),
                Point::from((3, 4)),
                Point::from((2, 3))
            ],
            resulting_points
        );
    }
}
