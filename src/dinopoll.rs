use std::env;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::db::Poll;
use crate::slack;

#[derive(Deserialize)]
struct Response {
	pub ok: bool,
	pub message: String,
	pub poll: PollResponse,
}

#[derive(Deserialize)]
pub struct PollResponse {
	pub timestamp: String,
}

pub fn create_poll(client: &Client, poll: &Poll) -> Result<PollResponse> {
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
		"Response didn't have a status code of OK when trying to create the poll with dinopoll"
	);

	let response_data: Response =
		serde_json::from_str(&response.text()?).context("Failed to parse dinopoll response")?;
	anyhow::ensure!(
		response_data.ok,
		"Error given from daily poll: {}",
		response_data.message
	);

	Ok(response_data.poll)
}
