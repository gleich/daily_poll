use std::env;

use anyhow::{bail, Context};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;

fn get_token() -> Result<String, anyhow::Error> { Ok(env::var("AIRTABLE_TOKEN")?) }

#[derive(Deserialize)]
struct AirtableRecord<T> {
	fields: T,
}

#[derive(Deserialize)]
struct AirtableResponse<T> {
	records: Vec<AirtableRecord<T>>,
}

#[derive(Deserialize, Debug)]
struct PollData {
	question: String,
	options: String,
	author: String,
	#[serde(default = "bool::default")]
	done: bool,
}
#[derive(Debug)]
pub struct Poll {
	pub question: String,
	pub options: Vec<String>,
	pub author: String,
	pub done: bool,
}

pub fn get_polls(client: &Client) -> Result<Vec<Poll>, anyhow::Error> {
	let token = get_token()?;
	// Making request
	let response = client
		.get("https://api.airtable.com/v0/appycGfYYgt3yM6ie/Table%201?view=Grid%20view")
		.bearer_auth(token)
		.send()
		.context("Failed to get list of polls")?;

	// Checking status
	let status = response.status();
	if status != StatusCode::OK {
		bail!(
			"Request to get list of polls failed with status code of {}",
			status
		);
	}

	// Parsing response
	let polls_data: AirtableResponse<PollData> =
		serde_json::from_str(&response.text()?).context("Failed to parse response")?;
	let mut polls = Vec::new();
	for poll in polls_data.records.into_iter() {
		let fields = poll.fields;
		polls.push(Poll {
			question: fields.question,
			options: fields.options.lines().map(|l| l.to_string()).collect(),
			author: fields.author,
			done: fields.done,
		});
	}

	Ok(polls)
}
