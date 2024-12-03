use day_03::part2_nom::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2 nom")?;
    println!("{}", result);
    Ok(())
}
