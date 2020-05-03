use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let values_1: Vec<u32> = (0..200).chain(0..50).collect();
    let values_2: Vec<u32> = (100..300).chain(100..220).chain(150..250).collect();

    pre::Density::new(values_1)
        .set_title("My legend")
        .to_owned()
        .compare_with(vec![pre::Density::new(values_2)])
        .set_title("My main title")
        .plot("identifier")?;

    Ok(())
}
