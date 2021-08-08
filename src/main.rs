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
			(0, 0) => send_reminder(&client, &database),
			(12, 0) => post_poll(&client, &database),
			_ => (),
		}
		thread::sleep(sleep_time);
	}
}

fn post_poll(client: &Client, database: &MysqlConnection) {
	println!("\n");
	info!("Posting poll");

	let poll = db::poll_to_post(database).expect("Failed to get poll from MySQL");
	info!("Got poll from MySQL");

	dinopoll::create_poll(client, &poll).expect("Failed to create poll with dinopoll");
	info!("Posted poll with question of \"{}\"", poll.question);
	db::set_as_used(database, &poll.question).expect("Failed to set poll as used");
	info!(
		"Set poll of question \"{}\" to used in airtable",
		poll.question
	);
}

fn send_reminder(client: &Client, database: &MysqlConnection) {
	println!("\n");
	info!("Sending reminder message");
	slack::send_reminder(client, database).expect("Failed to send reminder message");
	info!("Sent reminder message");
}
