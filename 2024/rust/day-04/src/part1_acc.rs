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

    let r = words
        .iter()
        .filter(|v| *v == "XMAS")
        .collect::<Vec<&String>>();

    Ok(r.len().to_string())
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
