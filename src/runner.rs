use colored::*;
use std::error::Error;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

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

pub fn eval<P: AsRef<Path>>(solution: P, paths: Vec<&str>, filter: Option<&str>) {
	// crate::sandbox::set_limits(Some(1.0), Some(256_u64 * 1024 * 1024));
}
