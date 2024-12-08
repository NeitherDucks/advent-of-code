use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (input, width, height) = parse(input);

    let mut results: HashSet<(i32, i32)> = HashSet::new();
    for (_, values) in input.iter() {
        for ((y0, x0), (y1, x1)) in values.iter().copied().tuple_combinations() {
            let y_offset = y0 - y1;
            let x_offset = x0 - x1;

            for i in 0..width.max(height) {
                results.insert((y0 + (y_offset * i), x0 + (x_offset * i)));
                results.insert((y1 - (y_offset * i), x1 - (x_offset * i)));
            }
        }
    }

    let r = results
        .iter()
        .filter(|(y, x)| (0..=width).contains(y) && (0..height).contains(x))
        .count();

    Ok(r.to_string())
}

type ParseOutput = (HashMap<char, Vec<(i32, i32)>>, i32, i32);

fn parse(input: &str) -> ParseOutput {
    let mut results: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let height = input.lines().count() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            width = width.max(x as i32);
            if c != '.' {
                let Some(value) = results.get_mut(&c) else {
                    results.insert(c, vec![(y as i32, x as i32)]);
                    continue;
                };

                value.push((y as i32, x as i32));
            }
        }
    }

    (results, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
