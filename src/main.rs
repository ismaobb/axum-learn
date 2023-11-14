use std::env;

use tracing_subscriber::EnvFilter;

fn main() {
	dotenvy::dotenv().ok();

	let level = env::var("RUST_LOG").unwrap_or(tracing::Level::INFO.to_string());
	env::set_var("RUST_LOG", level);

	tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
	api::bootstrap();
}
