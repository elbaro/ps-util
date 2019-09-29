use cookie::{Cookie, CookieJar};
use reqwest::{Client, Response};
// use selenium_rs::webdriver::{Browser, WebDriver};
use fantoccini::Client as Driver;
use serde::Serialize;
use std::error::Error;
use std::process::Child;
use std::process::{Command, Stdio};
// use time::{now_utc, Tm};
use std::future::Future;
use url::Url;

struct BrowserGuard(Child);

impl Drop for BrowserGuard {
	fn drop(&mut self) {
		println!("closing browser ..");
		self.0.kill().expect("browser refuse to die");
		self.0.wait().expect("cannot wait for browser");
	}
}

pub struct Session {
	browser: BrowserGuard,
	client: Client,
	// pub driver: Driver,
	pub jar: CookieJar,
	last_url: Option<String>,
}

impl Session {
	pub fn new() -> Result<Self, failure::Error> {
		let browser = BrowserGuard(
			// Command::new("chromedriver")
			Command::new("geckodriver")
				// .args(&["--port=4444"])
				// .args(&["--port=4444", "--url-base=wd/hub"])
				// Command::new("geckodriver")
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.spawn()?,
		);

		// let mut driver = WebDriver::new(Browser::Firefox);
		// let mut driver = WebDriver::new(Browser::Chrome);
		// driver.start_session()?;

		Ok(Session {
			browser,
			// driver,
			client: Client::new(),
			jar: CookieJar::new(),
			last_url: None,
		})
	}

	pub fn new_browsing(&mut self) {
		self.last_url = None;
	}

	pub fn get(&mut self, url: &Url) -> Result<Response, failure::Error> {
		let res = reqwest::get(url.as_str())?;
		for raw_cookie in res.headers().get_all(reqwest::header::SET_COOKIE) {
			let mut cookie = Cookie::parse(raw_cookie.to_str()?.to_string()).expect("burnt cookie");
			cookie.set_domain(url.host_str().unwrap().to_string());
			self.jar.add(cookie);
		}
		self.last_url = Some(url.to_string());
		Ok(res)
	}
	pub fn get_cookie(&self, url: &Url, name: &str) -> Option<String> {
		let host = url.host_str().unwrap();
		for cookie in self.jar.iter() {
			if cookie.domain() == Some(host) {
				return Some(cookie.value().to_owned());
			}
		}
		None
	}
	pub fn post<T: Serialize + ?Sized>(
		&mut self,
		url: &Url,
		form: Option<&T>,
	) -> Result<Response, failure::Error> {
		let mut req = self.client.post(url.as_str());
		let host = url.host_str().unwrap();
		for cookie in self.jar.iter() {
			if cookie.domain() == Some(host) {
				let raw = format!("{}={}", cookie.name(), cookie.value());
				req = req.header(reqwest::header::COOKIE, raw);
			}
		}
		if let Some(ref u) = self.last_url {
			req = req.header(reqwest::header::REFERER, u.to_owned());
		}
		if let Some(f) = form {
			req = req.form(f);
		}
		let res = req.send()?;
		self.last_url = Some(url.to_string());
		Ok(res)
	}
}
