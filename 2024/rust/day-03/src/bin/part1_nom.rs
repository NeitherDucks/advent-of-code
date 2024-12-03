use day_03::part1_nom::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1 nom")?;
    println!("{}", result);
    Ok(())
}