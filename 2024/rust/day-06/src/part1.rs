use std::fmt::Display;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Item {
    Guard(Direction),
    Obstacle,
    Ground,
    Visited,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Item::Ground => ".",
                Item::Guard(dir) => match dir {
                    Direction::North => "^",
                    Direction::South => "v",
                    Direction::East => ">",
                    Direction::West => "<",
                },
                Item::Obstacle => "#",
                Item::Visited => "X",
            }
        )
    }
}

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = parse_manually(input);
    let (mut y, mut x) = find_guard_pos(&grid);

    while move_guard(&mut x, &mut y, &mut grid) {}

    let r = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|cell| match cell {
                    Item::Visited => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    Ok(r.to_string())
}

fn move_guard(x: &mut i32, y: &mut i32, grid: &mut [Vec<Item>]) -> bool {
    let current_pos = (*y as usize, *x as usize);
    let Some(current_item) = grid
        .get(current_pos.0)
        .and_then(|line| line.get(current_pos.1))
    else {
        panic!("Could not get Item at specified position");
    };

    let dir = match current_item {
        Item::Guard(dir) => *dir,
        _ => panic!("Could not get Guard from specified position"),
    };

    let next_pos = match dir {
        Direction::North => ((*y - 1) as usize, *x as usize),
        Direction::South => ((*y + 1) as usize, *x as usize),
        Direction::East => (*y as usize, (*x + 1) as usize),
        Direction::West => (*y as usize, (*x - 1) as usize),
    };

    // Check if the guard exits the grid
    let Some(next_item) = grid.get(next_pos.0).and_then(|line| line.get(next_pos.1)) else {
        grid[current_pos.0][current_pos.1] = Item::Visited;

        return false;
    };

    // Otherwise, check if there is an Obstacle
    match next_item {
        // Rotate is so
        Item::Obstacle => grid[current_pos.0][current_pos.1] = Item::Guard(dir.rotate()),
        // Otherwise, move the guard to the next cell, and tag the current cell as visited
        _ => {
            grid[current_pos.0][current_pos.1] = Item::Visited;
            grid[next_pos.0][next_pos.1] = Item::Guard(dir);
            *x = next_pos.1 as i32;
            *y = next_pos.0 as i32;
        }
    }

    true
}

fn find_guard_pos(grid: &[Vec<Item>]) -> (i32, i32) {
    let y: usize = grid
        .iter()
        .enumerate()
        .find_map(|(index, value)| {
            value
                .contains(&Item::Guard(Direction::North))
                .then_some(index)
        })
        .expect("Could not find guard");

    let x = grid[y]
        .iter()
        .enumerate()
        .find_map(|(index, value)| (value == &Item::Guard(Direction::North)).then_some(index))
        .expect("Could not find guard");

    (y as i32, x as i32)
}

fn parse_manually(input: &str) -> Vec<Vec<Item>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Item::Obstacle,
                    '^' => Item::Guard(Direction::North),
                    _ => Item::Ground,
                })
                .collect::<Vec<Item>>()
        })
        .collect()
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
