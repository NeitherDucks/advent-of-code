#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // find the sum of the first and last digit in the line, add all digits together
    let r = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("should be number");
            let last = it.last();

            match last {
                None => first * 10 + first,
                Some(last) => first * 10 + last,
            }
        })
        .sum::<u32>();

    Ok(r.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
