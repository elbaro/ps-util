use crate::session::Session;
use std::error::Error;
use std::path::Path;

pub trait CollectionHandle {
	// fn overview();
	// fn register();
	fn download();
}

struct Problem {}

pub trait ProblemHandle {
	fn get_problem_url(&self) -> String;
	fn get_submit_url(&self) -> String;

	// fn text_description(&self);
	fn submit<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<Error>> {
		let path = path.as_ref();
		let url = self.get_submit_url();
		unimplemented!();
		// client.post(url).unwrap();
	}
	fn download<P: AsRef<Path>>(&self, dir: P) -> Result<(), Box<Error>>;
}

pub trait Site {
	fn login(id: &str, pw: &str) -> Result<(), Box<Error>>;
}
