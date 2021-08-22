use std::thread;
use std::time::Duration;

use chrono::{Timelike, Utc};
use diesel::prelude::*;
use reqwest::blocking::Client;
use tracing::info;

#[macro_use]
extern crate diesel;

mod db;
mod dinopoll;
mod schema;
mod slack;

fn main() {
	tracing_subscriber::fmt::init();

	let client = Client::new();
	let database = db::connect().expect("Failed to connect to MySQL database");
	info!("Created client and connected to database");

	let sleep_time = Duration::from_secs(60);
	loop {
		let now = Utc::now();
		match (now.hour(), now.minute()) {
			(23, 59) => send_reminder(&client, &database),
			(11, 59) => post_poll(&client, &database),
			_ => (),
		}
		thread::sleep(sleep_time);
	}
}

fn post_poll(client: &Client, database: &MysqlConnection) {
	println!("\n");
	info!("Posting poll in one minute");
	thread::sleep(Duration::from_secs(60));

	let poll = db::poll_to_post(database).expect("Failed to get poll from MySQL");
	info!("Got poll from MySQL");

	let poll_response =
		dinopoll::create_poll(client, &poll).expect("Failed to create poll with dinopoll");
	info!("Posted poll with question of \"{}\"", poll.question);
	slack::pin_msg(&client, poll_response.timestamp).expect("Failed to pin poll");
	info!("Pinned slack message containing poll");
	db::set_as_used(database, &poll.question).expect("Failed to set poll as used");
	info!(
		"Set poll of question \"{}\" to used in database",
		poll.question
	);
}

fn send_reminder(client: &Client, database: &MysqlConnection) {
	println!("\n");
	info!("Sending reminder message in one minute");
	thread::sleep(Duration::from_secs(60));
	slack::send_reminder(client, database).expect("Failed to send reminder message");
	info!("Sent reminder message");
}
