#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    let values: Vec<i32> = input
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    for slice in values.chunks(2) {
        first_column.push(slice[0]);
        second_column.push(slice[1]);
    }

    let score: i32 = first_column
        .iter()
        .map(|number| *number * second_column.iter().filter(|x| number == *x).count() as i32)
        .sum();

    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
