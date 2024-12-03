use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String> {
    let mut result = 0;

    'line: for line in input.lines() {
        println!();

        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        let Some(err_pos) = get_error_position(&reports) else {
            result += 1;
            continue;
        };

        for i in 0..=2 {
            let mut test_reports = reports.clone();
            test_reports.remove(err_pos + i);

            if get_error_position(&test_reports).is_none() {
                result += 1;
                continue 'line;
            }
        }
    }

    Ok(result.to_string())
}

fn get_error_position(reports: &Vec<i32>) -> Option<usize> {
    println!("reports: {reports:?}");

    let steps = reports
        .iter()
        .tuple_windows()
        .map(|(l, r)| r - l)
        .collect::<Vec<i32>>();

    println!("steps: {steps:?}");

    let dir = steps.iter().map(|v| v.signum()).sum::<i32>().signum();

    println!("dir: {dir}");

    let errors = steps
        .iter()
        .map(|l| !(l.signum() == dir && (0..=3).contains(&l.abs())))
        .collect::<Vec<bool>>();

    println!("errors: {errors:?}");

    errors.iter().find_position(|v| **v).map(|(v, _)| v)
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
1 3 6 7 9
1 2 3 4 11
11 1 2 3 4 5
1 2 3 11 4";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
