const XMAS: [&str; 4] = ["X", "M", "A", "S"];

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut words: Vec<String> = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        for x in 0..line.len() {
            for i in 0..8 {
                words.push(
                    (0..4)
                        .map(|j| {
                            let dir = get_dir(i);
                            let y = y as i32 + (dir.0 * j);
                            let x = x as i32 + (dir.1 * j);

                            grid.get(y as usize)
                                .and_then(|l| l.get(x as usize))
                                .unwrap_or(&"".to_string())
                                .to_string()
                        })
                        .reduce(|acc, f| acc + f.as_str())
                        .unwrap_or(String::new()),
                );
            }
        }
    }

    let r = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            (0..line.len())
                .map(|x| {
                    (0..8)
                        .map(|i| is_next_letter_correct(0, (x, y), get_dir(i), &grid) as i32)
                        .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    Ok(r.to_string())
}

fn is_next_letter_correct(
    current: usize,
    position: (usize, usize),
    direction: (i32, i32),
    array: &[Vec<String>],
) -> bool {
    let y = position.0 as i32 + (direction.0 * current as i32);
    let x = position.1 as i32 + (direction.1 * current as i32);

    let Some(l) = array.get(y as usize).and_then(|f| f.get(x as usize)) else {
        return false;
    };

    let r = l.as_str() == XMAS[current];

    if r && current + 1 < 4 {
        r && is_next_letter_correct(current + 1, position, direction, array)
    } else {
        r
    }
}

fn get_dir(i: i32) -> (i32, i32) {
    match i {
        0 => (-1, 0),
        1 => (1, 0),
        2 => (0, -1),
        3 => (0, 1),
        4 => (-1, -1),
        5 => (-1, 1),
        6 => (1, -1),
        _ => (1, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
