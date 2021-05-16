use std::env;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

const API_URL: &str = "https://api.airtable.com/v0/appycGfYYgt3yM6ie/questions";

fn get_token() -> Result<String> {
	Ok(env::var("AIRTABLE_TOKEN").context("Failed to get airtable token")?)
}

#[derive(Deserialize)]
struct AirtableRecord<T> {
	fields: T,
	id: String,
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
	#[serde(default)]
	used: bool,
	#[serde(default)]
	add: bool,
	#[serde(default)]
	multiselect: bool,
}
#[derive(Debug)]
pub struct Poll {
	pub multiselect: bool,
	pub question: String,
	pub options: Vec<String>,
	pub author: String,
	pub used: bool,
	pub add: bool,
	pub id: String,
}

pub fn get_polls(client: &Client) -> Result<Vec<Poll>> {
	// Making request
	let response = client
		.get(API_URL)
		.bearer_auth(get_token()?)
		.send()
		.context("Failed to get list of polls")?;
	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	// Parsing response
	let polls_data: AirtableResponse<PollData> =
		serde_json::from_str(&response.text()?).context("Failed to parse response")?;
	let mut polls = Vec::new();
	for poll in polls_data.records.into_iter() {
		let fields = poll.fields;
		polls.push(Poll {
			multiselect: fields.multiselect,
			question: fields.question,
			options: fields.options.lines().map(|l| l.to_string()).collect(),
			author: fields.author,
			used: fields.used,
			add: fields.add,
			id: poll.id,
		});
	}

	Ok(polls)
}

pub fn set_as_used(client: &Client, poll: &Poll) -> Result<()> {
	let response = client
		.patch(API_URL)
		.json(&json!({"records": [{"id": poll.id, "fields": {"used": true}}]}))
		.bearer_auth(get_token()?)
		.send()
		.context("Failed to set poll used status to true")?;

	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	Ok(())
}
