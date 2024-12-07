use std::collections::HashMap;

use itertools::Itertools;
use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, line_ending},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

const OPERATORS: [char; 2] = ['*', '+'];

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, input) = parse(input).map_err(|e| miette!("Could not parse: {}", e))?;

    let r = input.iter().filter_map(|(result, items)| {
        //
        let count = items.len() - 1;
        (0..count)
            .map(|_| OPERATORS)
            .multi_cartesian_product()
            .any(|f| {
                let mut ops = f.iter();

                *result
                    == items
                        .iter()
                        .copied()
                        .reduce(|acc, r| {
                            let op = ops.next().unwrap();
                            match op {
                                '+' => acc + r,
                                '*' => acc * r,
                                _ => 0,
                            }
                        })
                        .unwrap_or(0)
            })
            .then_some(result)
    });

    Ok(r.sum::<u64>().to_string())
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(char(' '), complete::u64),
    )(input)
}

fn parse(input: &str) -> IResult<&str, HashMap<u64, Vec<u64>>> {
    many1(alt((terminated(parse_line, line_ending), parse_line)))(input)
        .map(|(i, v)| (i, v.into_iter().collect::<HashMap<u64, Vec<u64>>>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        // let input = "190: 10 19";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
