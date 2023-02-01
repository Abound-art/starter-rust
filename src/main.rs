mod abound;
mod algo;

fn main() -> Result<(), abound::Error> {
    let config: algo::Config = abound::load_config()?;
    let img = algo::run(config);
    abound::save_png(img)?;
    Ok(())
}
