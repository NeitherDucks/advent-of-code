use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (obstacles, mut player, width, height) = parse_manually(input);
    let mut moves: HashSet<(i32, i32)> = HashSet::new();

    let mut current_dir = Direction::North;

    loop {
        let next_position = get_next_position(&player, current_dir);

        // Reached an edge
        if !((0..width).contains(&player.1) && (0..height).contains(&player.0)) {
            // moves.insert(player);

            break;
        }

        // Reached an obstacle
        if obstacles.contains(&next_position) {
            current_dir = current_dir.rotate();
        // Otherwise move
        } else {
            moves.insert(player);
            player = next_position;
        }
    }

    Ok(moves.len().to_string())
}

fn get_next_position(pos: &(i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (pos.0 - 1, pos.1),
        Direction::South => (pos.0 + 1, pos.1),
        Direction::East => (pos.0, pos.1 + 1),
        Direction::West => (pos.0, pos.1 - 1),
    }
}

type ParseResult = (HashSet<(i32, i32)>, (i32, i32), i32, i32);

fn parse_manually(input: &str) -> ParseResult {
    let mut walls = HashSet::new();
    let mut player = (0, 0);
    let mut width = 0;
    let height = input.lines().count() as i32;

    for (y, line) in input.lines().enumerate() {
        width = line.len() as i32;
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((y as i32, x as i32));
                }
                '^' => {
                    player = (y as i32, x as i32);
                }
                _ => {}
            }
        }
    }

    (walls, player, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
