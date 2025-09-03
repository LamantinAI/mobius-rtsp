use mobius_rtsp::{config::MobiusConfig, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config = MobiusConfig::new()?;
    run(config)
}
