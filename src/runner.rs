use colored::*;
use crate::sandbox;
use crate::sandbox::Limitation;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use strum_macros::Display;
use wait_timeout::ChildExt;
use walkdir::WalkDir;

#[derive(Display)]
pub enum JudgeResult {
	Correct,
	WrongAnswer,
	TimeOver,
	MemoryOver,
	RuntimeError,
}

pub fn validate_data<P1: AsRef<Path>, P2: AsRef<Path>>(
	exe_path: P1,
	data_path: P2,
) -> Result<(), Box<Error>> {
	let data_path = data_path.as_ref();
	let f: File = File::open(&data_path).expect("fail to open file");

	let status = Command::new(exe_path.as_ref())
		.stdin(f)
		.before_exec(|| Ok(()))
		.status()?;

	if status.success() {
		Ok(())
	} else {
		Err("validation failed".into())
	}
}

pub fn validate<P: AsRef<Path>>(
	exe_path: P,
	paths: Vec<&str>,
	filter: Option<&str>,
) -> Result<(), Box<Error>> {
	let exe_path = exe_path.as_ref();
	println!("{}", "Validating ..".green());

	let filter = regex::Regex::new(filter.unwrap_or(".*")).expect("wrong filter format");
	let mut good = 0;
	let mut error = 0;

	for path in paths {
		let path: &Path = path.as_ref();
		if path.is_file() {
			if !filter.is_match(path.file_name().unwrap().to_str().unwrap()) {
				continue;
			}

			match validate_data(exe_path, path) {
				Ok(()) => {
					good += 1;
				}
				Err(err) => {
					error += 1;
					eprintln!("[Error] {}", err);
					eprintln!("\t=> {}", path.display());
				}
			}
		} else {
			let dir = path;
			for entry in WalkDir::new(dir).into_iter() {
				let entry = entry.expect("fail to list dir");
				let path = entry.path();
				if !filter.is_match(path.file_name().unwrap().to_str().unwrap()) {
					continue;
				}

				match validate_data(exe_path, path) {
					Ok(()) => {
						good += 1;
					}
					Err(err) => {
						error += 1;

						let relative = path.strip_prefix(dir).unwrap_or(path);
						eprintln!("{} {}", "[Error]".red(), err);
						eprintln!("\t=> {}", relative.display());
					}
				}
			}
		}
	}
	println!("   {}: {}", "Good".green(), good);
	println!("  {}: {}", "Error".red(), error);
	Ok(())
}

pub fn eval_case<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
	solution: P1,
	input: P2,
	answer: P3,
	limit: &Limitation,
) -> Result<JudgeResult, Box<Error>> {
	let input = input.as_ref();
	let f: File = File::open(&input).expect("fail to open file");

	let answer = File::open(&answer)?;
	let child_limit = limit.clone();

	let mut child = Command::new(solution.as_ref())
		.stdin(f)
		.stdout(Stdio::piped())
		.before_exec(move || {
			sandbox::set_limits(&child_limit).unwrap();
			Ok(())
		})
		.spawn()?; // judge error

	let time = Duration::from_float_secs(limit.time.unwrap_or(1.0) as f64);
	let success = match child.wait_timeout(time)? {
		Some(status) => status.success(),
		None => {
			child.kill()?;
			child.wait()?;
			return Ok(JudgeResult::TimeOver);
		}
	};

	if success {
		// AC / WA
		let judge: bool = {
			let stdout = child.stdout.take();
			let user_it = stdout.unwrap().bytes().map(|e| e.unwrap());
			let ans_it = answer.bytes().map(|e| e.unwrap());
			user_it.eq(ans_it)
		};
		if judge {
			Ok(JudgeResult::Correct)
		} else {
			Ok(JudgeResult::WrongAnswer)
		}
	} else {
		// runtime error
		Ok(JudgeResult::RuntimeError)
	}
}

pub fn eval<P1: AsRef<Path>, P2: AsRef<Path>>(
	solution: P1,
	data_dir: P2,
	in_filter: &str,
	out_filter: &str,
	limit: &Limitation,
) -> Result<(), Box<Error>> {
	println!("{}\n", "Evaluating ..".green());
	let solution = solution.as_ref();
	let data_dir = data_dir.as_ref();
	let mut correct = 0;
	let mut incorrect = 0;
	let mut judge_error = 0;

	let inputs: Vec<_> = {
		let mut v: Vec<_> = WalkDir::new(data_dir)
			.into_iter()
			.map(|e| e.expect("fail to list dir").into_path())
			.filter(|p| p.file_name().unwrap().to_str().unwrap().contains(in_filter))
			.collect();
		v.sort();
		v
	};
	let outputs: Vec<_> = {
		let mut v: Vec<_> = WalkDir::new(data_dir)
			.into_iter()
			.map(|e| e.expect("fail to list dir").into_path())
			.filter(|p| {
				p.file_name()
					.unwrap()
					.to_str()
					.unwrap()
					.contains(out_filter)
			})
			.collect();
		v.sort();
		v
	};

	assert!(
		inputs.len() == outputs.len(),
		"{} inputs != {} outputs",
		inputs.len(),
		outputs.len()
	);

	for (input, output) in inputs.iter().zip(outputs.iter()) {
		let relative = input.strip_prefix(data_dir).unwrap_or(input);
		match eval_case(solution, input, output, limit) {
			Ok(JudgeResult::Correct) => {
				correct += 1;
				println!("{:>15} {}", "[Correct]".green(), relative.display());
			}
			Ok(reason) => {
				incorrect += 1;
				println!(
					"{:>15} {}",
					format!("[{}]", reason).red(),
					relative.display()
				);
			}
			Err(err) => {
				judge_error += 1;
				println!("{:>15} {}", "[Judge Error]".purple(), err);
				println!("\t=> {}", relative.display());
			}
		}
	}
	println!();
	println!("       {}: {}", "Good".green(), correct);
	println!("  {}: {}", "Incorrect".red(), incorrect);
	println!("{}: {}", "Judge Error".purple(), judge_error);
	Ok(())
}
