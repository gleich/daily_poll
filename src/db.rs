use std::env;

use anyhow::Result;
use diesel::prelude::*;

use crate::schema;

#[derive(Queryable)]
pub struct Poll {
	pub question: String,
	pub author: String,
	pub used: bool,
	pub add_options: bool,
	pub multiselect: bool,
}

#[derive(Queryable)]
pub struct PollOption {
	pub id: u32,
	pub question: String,
	pub option_name: String,
}

pub fn connect() -> Result<MysqlConnection> {
	let database_url = env::var("DATABASE_URL")?;
	Ok(MysqlConnection::establish(&database_url)?)
}

pub fn get_poll(database: &MysqlConnection) -> Result<Poll> {
	let results = schema::polls::table
		.filter(schema::polls::used.eq(false))
		.limit(1)
		.load::<Poll>(database)?;
	Ok(results.into_iter().nth(0).unwrap())
}
