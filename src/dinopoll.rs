use std::env;

use anyhow::{bail, Context};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;

use crate::airtable::Poll;

pub fn create_poll(client: &Client, poll: &Poll) -> Result<(), anyhow::Error> {
	// Sending request
	let response = client
		.post("https://dinopoll.host.calebdenio.me/create")
		.json(
			&json!({"title": poll.question, "channel": env::var("DINOPOLL_CHANNEL")?, "options": poll.options}),
		)
		.bearer_auth(env::var("DINOPOLL_TOKEN")?)
		.send()
		.context(format!(
			"Failed to create poll with question of {}",
			poll.question
		))?;

	// Checking status
	let status = response.status();
	if status != StatusCode::OK {
		bail!(
			"Request to create poll via dinopoll failed with status code of {}.\n{:?}",
			status,
			poll
		)
	}

	Ok(())
}
