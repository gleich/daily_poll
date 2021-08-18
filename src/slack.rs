use std::env;

use anyhow::{Context, Result};
use diesel::MysqlConnection;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;

use crate::db::unused_polls;

pub const MATT_GLEICH_SLACK_ID: &str = "UGTQ393RR";

pub fn send_reminder(client: &Client, database: &MysqlConnection) -> Result<()> {
	let polls_left = unused_polls(database)?.len();

	let response = client
		.post(env::var("SLACK_WEBHOOK_URL").context("Failed to find slack webhook URL env var")?)
		.json(&json!({
			"text":
				format!(
					"Hey everyone! Just a little reminder that you can DM <@{}> if you have an \
					 idea for a poll. {} Have a good rest of your day everyone :)",
					MATT_GLEICH_SLACK_ID,
					if polls_left <= 2 {
						format!(
							"Gadˈzo͞oks we {}have {} {} left in the queue!!",
							if polls_left > 0 { "only " } else { "" },
							polls_left,
							match polls_left {
								1 => "poll",
								_ => "polls",
							}
						)
					} else {
						format!("We currently have {} polls left in the queue.", polls_left)
					}
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
