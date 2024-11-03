mod config;

use crate::config::settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environment = "dev";

    let settings = Settings::new(environment)?;

    println!("{:?}", settings);
    Ok(())
}
