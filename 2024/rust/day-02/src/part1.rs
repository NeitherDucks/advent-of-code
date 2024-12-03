pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;

    for line in input.lines() {
        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        let dir = (reports[0] - reports[1]).signum();
        let mut valid = true;

        for v in reports.windows(2) {
            let l = v[0];
            let r = v[1];

            let dif = l - r;
            let ldir = dif.signum();
            let dif = dif.abs();

            valid &= dif > 0 && dif < 4 && dir == ldir;
        }

        result += valid as i32;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
