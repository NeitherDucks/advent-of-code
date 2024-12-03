use regex::Regex;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let result: i32 = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [l, r])| l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap())
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
