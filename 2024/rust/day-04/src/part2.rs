#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    let r = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, c)| match c == &"A".to_string() {
                    true => x_at((y as i32, x as i32), &grid) as i32,
                    false => 0,
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    Ok(r.to_string())
}

fn x_at(pos: (i32, i32), array: &[Vec<String>]) -> bool {
    (0..4)
        .map(|i| {
            let dir = get_dir_x(i);

            let y = pos.0 + dir.0;
            let x = pos.1 + dir.1;

            let o_y = pos.0 - dir.0;
            let o_x = pos.1 - dir.1;

            let a_y = pos.0 - dir.0;
            let a_x = pos.1 + dir.1;

            let ao_y = pos.0 + dir.0;
            let ao_x = pos.1 - dir.1;

            letter_at((y, x), "M", array)
                && letter_at((o_y, o_x), "S", array)
                && ((letter_at((a_y, a_x), "M", array) && letter_at((ao_y, ao_x), "S", array))
                    || (letter_at((a_y, a_x), "S", array) && letter_at((ao_y, ao_x), "M", array)))
        })
        .any(|f| f)
}

fn letter_at(pos: (i32, i32), letter: &str, array: &[Vec<String>]) -> bool {
    array
        .get(pos.0 as usize)
        .and_then(|line| line.get(pos.1 as usize))
        .is_some_and(|v| v == letter)
}

fn get_dir_x(i: i32) -> (i32, i32) {
    match i {
        1 => (-1, -1),
        2 => (-1, 1),
        3 => (1, -1),
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
