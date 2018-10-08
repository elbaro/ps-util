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
	fn submit<P: AsRef<Path>>(&self, path: P) {
		let path = path.as_ref();
		let url = self.get_submit_url();
		// client.post(url).unwrap();
	}
	fn download<P: AsRef<Path>>(&self, dir: P) -> Result<(), Box<Error>>;
}
