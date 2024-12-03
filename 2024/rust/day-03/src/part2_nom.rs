use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let mut can_do = true;

    let result = instructions.iter().fold(0, |acc, ins| {
        acc + match ins {
            Instruction::Mul(l, r) => l * r * can_do as u32,
            Instruction::Do => {
                can_do = true;
                0
            }
            Instruction::Dont => {
                can_do = false;
                0
            }
        }
    });

    Ok(result.to_string())
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn mul_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        complete::char('('),
        separated_pair(complete::u32, tag(","), complete::u32),
        complete::char(')'),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn do_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;

    Ok((input, Instruction::Do))
}

fn dont_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;

    Ok((input, Instruction::Dont))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
        many_till(
            anychar,
            alt((do_instruction, dont_instruction, mul_instruction)),
        )
        .map(|(_, instruction)| instruction),
    )(input)
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
