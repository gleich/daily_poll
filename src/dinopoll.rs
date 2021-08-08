use std::env;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;

use crate::db::Poll;
use crate::slack;

pub fn create_poll(client: &Client, poll: &Poll) -> Result<()> {
	// Creating title based off author
	let author_note = if poll.author != slack::MATT_GLEICH_SLACK_ID {
		// If the user is not Matthew Gleich
		format!(" -- thanks for the submission <@{}>!", &poll.author)
	} else {
		String::new()
	};
	let title = format!("{}{}", poll.question, author_note);

	let response = client
		.post("https://dinopoll.host.calebdenio.me/create")
		.json(&json!({
			"title": title, "channel": env::var("DINOPOLL_CHANNEL")?, "options":
			poll.options, "othersCanAdd": poll.add_options, "multipleVotes": poll.multiselect
		}))
		.bearer_auth(env::var("DINOPOLL_TOKEN")?)
		.send()
		.context(format!("Failed to create poll with title of {}", title))?;

	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	Ok(())
}
