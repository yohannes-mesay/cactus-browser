use anyhow::Result;
use cactus_core::LaunchArgs;

fn main() -> Result<()> {
    let args = LaunchArgs::parse(std::env::args());
    let filter = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter)).init();

    cactus_ui::run(args)
}
