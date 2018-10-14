use chrono::{DateTime, Utc};
use crate::judge;
use crate::judge::Site;
use crate::session::Session;
use select::{document::Document, predicate::Class};
// use selenium_rs::webdriver::{Browser, Selector, WebDriver};
use fantoccini::{error::CmdError, Client as Driver, Locator};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/**
 * Set:
 *
 * https://codeforces.com/contest/1059/problem/A
 * https://codeforces.com/problemset/problem/1060/G
 * https://codeforces.com/gym/101915/problem/B
 *
 * Problem:
 *
 * Set + A~Z
 */

enum TaskHandle {
	Collection(u32),
	Problem(u32),
}

enum ContestStatus {
	Scheduled,
	Running,
	Finished,
}

pub struct ContestHandle(pub u32);

impl judge::CollectionHandle for ContestHandle {
	fn download() {}
}
struct Contest {
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,
}

pub struct ProblemHandle(pub u32, pub char);
impl judge::ProblemHandle for ProblemHandle {
	fn get_submit_url(&self) -> String {
		"submit url".to_string()
	}
	fn get_problem_url(&self) -> String {
		if self.0 <= 100000 {
			// contest
			// https://codeforces.com/contest/1013/problem/F
			// https://codeforces.com/problemset/problem/1060/H
			format!(
				"https://codeforces.com/problemset/problem/{}/{}",
				self.0, self.1
			)
		} else {
			// gym
			// https://codeforces.com/gym/101917/problem/B
			format!("https://codeforces.com/gym/{}/problem/{}", self.0, self.1)
		}
	}
	fn download<P: AsRef<Path>>(&self, dir: P) -> Result<(), Box<Error>> {
		let dir = dir.as_ref();
		let url = self.get_problem_url();
		let html_read = reqwest::get(&url)?;
		let doc: Document = Document::from_read(html_read)?;
		let prob = doc.find(Class("problem-statement")).next().unwrap();

		{
			let mut f = File::create(dir.join("problem.html"))?;
			f.write_all(include_bytes!("codeforces.css"))?;
			f.write_all(prob.html().as_bytes())?;
		}

		let title = prob.find(Class("title")).next().unwrap().text();
		let time_limit = prob.find(Class("time-limit")).next().unwrap().text();
		let memory_limit = prob.find(Class("memory-limit")).next().unwrap().text();
		for (i, node) in prob.find(Class("input")).enumerate() {
			let data = node
				.last_child()
				.unwrap()
				.inner_html()
				.replace("<br>", "\n");
			let mut f = File::create(dir.join(format!("input{}.txt", i + 1)))?;
			f.write_all(data.as_bytes())?;
		}
		for (i, node) in prob.find(Class("output")).enumerate() {
			let data = node
				.last_child()
				.unwrap()
				.inner_html()
				.replace("<br>", "\n");
			let mut f = File::create(dir.join(format!("output{}.txt", i + 1)))?;
			f.write_all(data.as_bytes())?;
		}
		Ok(())
	}

	fn submit<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<Error>> {
		let mut session = Session::new()?;
		Codeforces::login(&mut session, "aa", "bb")?;

		// let path = path.as_ref();
		// let login = &Url::parse("https://algospot.com/accounts/login/")?;
		// let login = &Url::parse("https://codeforces.com/enter")?;
		// let mut session = Session::new();
		// session.get(login)?;

		// let csrf_token = session
		// 	.get_cookie(login, "csrftoken")
		// 	.expect("no csrf token received");
		// let form = {
		// 	let mut h = HashMap::new();
		// 	h.insert("username", "aa");
		// 	h.insert("password", "bb");
		// 	h.insert("csrfmiddlewaretoken", &csrf_token);
		// 	h
		// };

		// let form = {
		// 	let mut h = HashMap::new();
		// 	h.insert("handleOrEmail", "aa");
		// 	h.insert("password", "bb");
		// 	h.insert("action", "enter");
		// 	h.insert("csrfmiddlewaretoken", &csrf_token);
		// 	h
		// };

		// let mut res = session.post(login, Some(&form))?;
		// println!("{}", res.text()?);
		Ok(())
	}
}

async fn login_async<'a>(
	session: &'a mut Session,
	id: &'a str,
	pw: &'a str,
) -> Result<(), CmdError> {
	let mut driver: Driver = unsafe { std::mem::uninitialized() };
	std::mem::swap(&mut session.driver, &mut driver);

	let mut driver = await!(driver.goto("https://codeforces.com/enter"))?;
	let mut f = await!(driver.form(Locator::Css("#enterForm")))?;
	await!(f.set_by_name("handleOrEmail", "abc"))?;
	await!(f.set_by_name("password", "abcefg"))?;
	await!(f.submit())?;

	// driver.navigate("https://codeforces.com/enter")?;
	// // already logged in?
	// let handle_input = driver.query_element(Selector::CSS, "#handleOrEmail")?;
	// let pw_input = driver.query_element(Selector::CSS, "#password")?;
	// let submit = driver.query_element(Selector::CSS, ".submit")?;
	// handle_input.type_text(id)?;
	// pw_input.type_text(pw)?;
	// submit.click()?;

	std::mem::swap(&mut session.driver, &mut driver);
	Ok(())
}

async fn login_impl<'a>(session: &'a mut Session, id: &'a str, pw: &'a str) {
	match await!(login_async(session, id, pw)) {
		Ok(()) => {}
		Err(err) => {
			eprintln!("login error: {}", err);
		}
	};
}

pub struct Codeforces {}

impl judge::Site for Codeforces {
	fn login(
		session: &'static mut Session,
		id: &'static str,
		pw: &'static str,
	) -> Result<(), Box<Error + 'static>> {
		// run_async requires 'static future
		tokio::run_async(login_impl(session, id, pw));
		Ok(())
	}
}

// type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`

// expected enum `std::result::Result`, found ()

// note: expected type `std::result::Result<(), fantoccini::error::CmdError>`
//          found type `()`
// note: required by `tokio::run_async`
