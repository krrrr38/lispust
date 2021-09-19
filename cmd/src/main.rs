extern crate core;

use anyhow::Result;

fn main() -> Result<()> {
    print_run("(+ 2 3)")?;
    print_run("(- 10 3)")?;
    print_run("(if true 100 200)")?;
    Ok(())
}

fn print_run(input: &str) -> Result<()> {
    let res = core::run(input)?;
    println!("{}", res);
    Ok(())
}
