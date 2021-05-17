use std::env;

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;

use crate::airtable::{self, Poll};

pub const MATT_GLEICH_SLACK_ID: &str = "UGTQ393RR";

pub fn send_reminder(client: &Client) -> Result<()> {
	let response = client
		.post(env::var("SLACK_WEBHOOK_URL").context("Failed to find slack webhook URL env var")?)
		.json(&json!({
			"text":
				format!(
					"Hey everyone! Just a little reminder that you can DM <@{}> if you have an \
					 idea for a poll. We currently have {} polls in the queue. Have a good day \
					 everyone :)",
					MATT_GLEICH_SLACK_ID,
					airtable::get_polls(client)?
						.into_iter()
						.filter(|p| !p.used)
						.collect::<Vec<Poll>>()
						.len()
				)
		}))
		.send()
		.context("Failed to send reminder message")?;

	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	Ok(())
}
