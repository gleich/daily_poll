use std::thread;
use std::time::Duration;

use chrono::{Timelike, Utc};
use reqwest::blocking::Client;
use tracing::info;

mod airtable;
mod dinopoll;
mod slack;

fn main() {
	tracing_subscriber::fmt::init();

	let client = Client::new();
	info!("Created client");

	let sleep_time = Duration::from_secs(60);
	loop {
		let now = Utc::now();
		match (now.hour(), now.minute()) {
			(0, 0) => send_reminder(&client),
			(12, 0) => post_poll(&client),
			_ => (),
		}
		thread::sleep(sleep_time);
	}
}

fn post_poll(client: &Client) {
	println!("\n");
	info!("Posting poll");

	let poll_data = airtable::get_polls(client).expect("Failed to get polls from airtable");
	info!("Got polls from airtable");

	for poll in poll_data {
		if !poll.used {
			dinopoll::create_poll(client, &poll).expect("Failed to create poll with dinopoll");
			info!("Posted poll with question of \"{}\"", poll.question);
			airtable::set_as_used(client, &poll).expect("Failed to set poll to used");
			info!(
				"Set poll of question \"{}\" to used in airtable",
				poll.question
			);
			break;
		}
	}
}

fn send_reminder(client: &Client) {
	println!("\n");
	info!("Sending reminder message");
	slack::send_reminder(client).expect("Failed to send reminder message");
	info!("Sent reminder message");
}
