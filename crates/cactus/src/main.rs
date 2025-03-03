use anyhow::Result;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let private = std::env::args().any(|a| a == "--private" || a == "-p");

    cactus_ui::run(private)
}
