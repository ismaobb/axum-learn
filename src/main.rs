use std::env;

use tracing_subscriber::EnvFilter;

fn main() {
	dotenvy::dotenv().ok();

	let level = env::var("RUST_LOG").unwrap_or(tracing::Level::INFO.to_string());
	env::set_var("RUST_LOG", level);

	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();
	api::bootstrap();
}

#[test]
fn test() {
	let x: Result<_, &str> = Ok("foo");
	assert_eq!(x.map_or(42, |v| v.len()), 3);

	let x: Result<&str, _> = Err("bar");
	assert_eq!(x.map_or(42, |v| v.len()), 42);
}
