use std::env;

use anyhow::Result;
use diesel::prelude::*;

use crate::schema;

#[derive(Queryable, Debug)]
pub struct PollData {
	pub question: String,
	pub author: String,
	pub used: bool,
	pub add_options: bool,
	pub multiselect: bool,
}

#[derive(Queryable, Debug)]
pub struct PollOption {
	pub id: i32,
	pub question: String,
	pub option_name: String,
}

pub struct Poll {
	pub question: String,
	pub author: String,
	pub used: bool,
	pub add_options: bool,
	pub multiselect: bool,
	pub options: Vec<String>,
}

pub fn connect() -> Result<MysqlConnection> {
	let database_url = env::var("DATABASE_URL")?;
	Ok(MysqlConnection::establish(&database_url)?)
}

/// Get a list of unused polls
/// Returns the raw poll data with no options included.
pub fn unused_polls(database: &MysqlConnection) -> Result<Vec<PollData>> {
	let poll_results = schema::polls::table
		.filter(schema::polls::used.eq(false))
		.load::<PollData>(database)?;
	Ok(poll_results)
}

/// Get the poll to post for the day with the options
pub fn poll_to_post(database: &MysqlConnection) -> Result<Poll> {
	let poll = unused_polls(database)?.into_iter().nth(0).unwrap();
	let options_results = schema::poll_options::table
		.filter(schema::poll_options::question.eq(&poll.question))
		.load::<PollOption>(database)?;

	let mut options: Vec<String> = Vec::new();
	for option in options_results {
		options.push(option.option_name)
	}

	Ok(Poll {
		question: poll.question,
		author: poll.author,
		used: poll.used,
		add_options: poll.add_options,
		multiselect: poll.add_options,
		options,
	})
}

/// Set a poll as used
pub fn set_as_used(database: &MysqlConnection, question: &String) -> Result<()> {
	diesel::update(schema::polls::table.find(question))
		.set(schema::polls::used.eq(true))
		.execute(database)?;
	Ok(())
}
