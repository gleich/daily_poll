use std::env;

use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;

use crate::airtable::Poll;

pub fn create_poll(client: &Client, poll: &Poll) -> Result<(), anyhow::Error> {
	// Creating title based off author
	let author_note = if poll.author != "UGTQ393RR" {
		// If the user is not Matthew Gleich
		format!(" -- thanks for the submission <@{}>!", &poll.question)
	} else {
		String::new()
	};
	let title = format!("{}{}", poll.question, author_note);

	let response = client
		.post("https://dinopoll.host.calebdenio.me/create")
		.json(
			&json!({"title": title, "channel": env::var("DINOPOLL_CHANNEL")?, "options": poll.options, "othersCanAdd": poll.add}),
		)
		.bearer_auth(env::var("DINOPOLL_TOKEN")?)
		.send()
		.with_context(|| format!("Failed to create poll with title of {}", title))?;

	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	Ok(())
}
