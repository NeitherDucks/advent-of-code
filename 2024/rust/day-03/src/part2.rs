use regex::Regex;

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re = Regex::new(r"(do\(\)|don't\(\))|mul\([0-9]+,[0-9]+\)").unwrap();
    let re2 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let captures = re.captures_iter(input);

    let mut can_do = true;
    let mut results = 0;

    for capture in captures {
        for value in capture.iter() {
            let Some(value) = value else {
                continue;
            };

            let result = match value.as_str() {
                "don't()" => {
                    can_do = false;
                    0
                }
                "do()" => {
                    can_do = true;
                    0
                }
                _ => re2
                    .captures_iter(value.as_str())
                    .map(|c| c.extract())
                    .map(|(_, [l, r])| l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap())
                    .sum(),
            };

            if can_do {
                results += result;
            }
        }
    }

    Ok(results.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
