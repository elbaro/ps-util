use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn sanitize_file<P: AsRef<Path>>(path: P, confirmed: bool) -> Result<bool, Box<Error>> {
	// 1. already good
	// 2. sanitized
	// 3. error

	// lowercase ext
	// - line ending to LF (remove any CR)
	// - make sure newline at EOF
	// - make sure ascii
	let path: &Path = path.as_ref();

	//    if let Some(ext) = path.extension() {
	//        if !ext.to_str()?.bytes().all(|b| b>=b'a' && b<=b'z') {
	//
	//            println!("  Renamed: {} => {}", path.display(),);
	//        }
	//    }

	// 1. check
	{
		let f = File::open(&path)?;
		let reader = BufReader::new(f);
		let mut it = reader.bytes();
		let mut cr = false;
		let mut last: u8 = 0;
		while let Some(b) = it.next() {
			let b = b?;
			// ascii range? 127=DEL
			// non-control? 32=SPACE, 9=TAB
			if !(b < 127 && (b >= 32 || b == 10 || b == 13)) {
				return Err(format!("non-ascii byte: {}", b).into());
			}
			if b == 13 {
				cr = true;
			}
			last = b;
		}

		if !cr && last == b'\n' {
			return Ok(false); // already good
		}
	}

	if confirmed {
		let f = File::open(&path)?;
		let reader = BufReader::new(f);
		let mut it = reader.bytes();

		let tmp_path = &path.with_extension("tmp");
		let tmp_file = File::create(&tmp_path)?;
		let mut bw = BufWriter::new(tmp_file);

		// let last_cr = false; // last_cr == skip_lf
		let mut last: u8 = 0;
		while let Some(b) = it.next() {
			let b = b?;
			// it == CR
			if last == b'\r' && b == b'\n' {
				// pass
			} else if b == b'\r' {
				bw.write(&[b'\n'])?;
			} else {
				bw.write(&[b])?;
			}
			last = b;
		}
		if last != b'\r' && last != b'\n' {
			bw.write(&[b'\n'])?;
		}
		std::fs::rename(tmp_path, path)?;
	}

	println!("Converted: {}", path.display());
	Ok(true)
}

pub fn sanitize<P: AsRef<Path>>(path: P, exts: Vec<&str>, confirmed: bool) {
	println!("   Exts: {:?}", exts);

	let mut good = 0;
	let mut changed = 0;
	let mut error = 0;

	for entry in WalkDir::new(path) {
		match entry {
			Ok(e) => {
				let path = e.path();
				if let Some(ext) = path.extension() {
					if exts.contains(&ext.to_str().unwrap()) {
						match sanitize_file(path, confirmed) {
							Ok(true) => {
								changed += 1;
							}
							Ok(false) => {
								good += 1;
							}
							Err(err) => {
								eprintln!("[Error] {}", err);
								eprintln!("\t=> {}", path.display());
							}
						}
					}
				}
			}
			Err(err) => {
				error += 1;
				println!("[File Error] {:?}", err);
			}
		}
	}
	println!("   {}: {}", "Good".green(), good);
	println!("{}: {}", "Changed".yellow(), changed);
	println!("  {}: {}", "Error".red(), error);
	if !confirmed && changed > 0 {
		eprintln!("\n{}", "Run with --confirmed to make actual change".red());
	}
}
