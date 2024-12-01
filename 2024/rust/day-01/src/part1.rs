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

    // Sort the two lists, ascending.
    first_column.sort_unstable();
    second_column.sort_unstable();

    // Calculate the distance and sum the list.
    let sum: i32 = std::iter::zip(first_column, second_column)
        .map(|(left, right)| (left - right).abs())
        .sum();

    Ok(sum.to_string())
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
