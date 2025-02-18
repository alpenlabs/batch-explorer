use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Batch Explorer",
    author = "Your Name",
    version = "1.0",
    about = "A Batch Explorer Application"
)]
pub struct Config {
    /// The URL of the Strata Fullnode
    #[arg(
        long,
        env = "STRATA_FULLNODE",
        default_value = "http://localhost:58000/",
        help = "Strata fullnode URL"
    )]
    pub strata_fullnode: String,

    /// The URL of the PostgreSQL database
    #[arg(
        long,
        env = "APP_DATABASE_URL",
        default_value = "postgres://postgres:password@localhost:5432/batch_explorer_db",
        help = "PostgreSQL database URL"
    )]
    pub database_url: String,

    /// The fetch interval in seconds
    #[arg(
        long,
        env = "APP_FETCH_INTERVAL",
        default_value_t = 100,
        help = "Fetch interval in seconds"
    )]
    pub fetch_interval: u64,

    #[arg(
        long,
        env = "STRATA_URL",
        default_value = "https://stratabtc.org",
        help = "Strata URL"
    )]
    pub strata_url: String,
}
