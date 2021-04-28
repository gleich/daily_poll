use std::env;

use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;

use crate::airtable::Poll;

pub fn create_poll(client: &Client, poll: &Poll) -> Result<(), anyhow::Error> {
	let response = client
		.post("https://dinopoll.host.calebdenio.me/create")
		.json(
			&json!({"title": format!("{} (<@{}>)", poll.question, poll.author), "channel": env::var("DINOPOLL_CHANNEL")?, "options": poll.options, "othersCanAdd": poll.add}),
		)
		.bearer_auth(env::var("DINOPOLL_TOKEN")?)
		.send()
		.with_context(|| format!("Failed to create poll with question of {}", poll.question))?;

	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	Ok(())
}
