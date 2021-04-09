use airtable::get_polls;
use reqwest::blocking::Client;
use tracing::info;

mod airtable;

fn main() {
	tracing_subscriber::fmt::init();
	let client = Client::new();
	info!("Created client");

	let polls = get_polls(&client).expect("Failed to get list of polls");
	println!("{:?}", polls);
}
