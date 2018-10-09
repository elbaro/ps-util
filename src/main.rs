use piston_window::*;
use std::path::Path;

mod cli;
mod generate;
use self::generate::Range;
mod codeforces;
mod judge;
use self::judge::*;
mod runner;
mod sanitize;

mod sandbox;

use std::fs::File;
use std::io::Write;

/**
 * ProblemSet
 * Problem
 *
 * CF
 * 		Problem
 * 		ProblemSet
 * 		Content
 * 		Gym
 *
 * TC
 * 		Prob
 * 		ProbSet
 */

fn show() {
	let mut window: PistonWindow = WindowSettings::new("", [640, 480])
		.exit_on_esc(true)
		.build()
		.unwrap();
	while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			clear([1.0; 4], graphics);
			rectangle(
				[1.0, 0.0, 0.0, 1.0], // red
				[0.0, 0.0, 100.0, 100.0],
				context.transform,
				graphics,
			);
		});
	}
}

mod vendor {
	trait Judge {
		fn overview() {}
		fn read() {}
		fn submit() {}
	}
}

fn main() {
	let args = cli::build_cli().get_matches();

	// app - matches - subcommand(app) - matches - subcommand(tree)
	let sub = args.subcommand.unwrap();
	match sub.name.as_ref() {
		"generate" => {
			let sub = sub.matches.subcommand.unwrap();
			match sub.name.as_ref() {
				"tree" => {
					let n = sub.matches.value_of("n").unwrap().parse().unwrap();
					if let Some(mut w) = sub.matches.values_of("int-weight") {
						let low: i64 = w.next().unwrap().parse().unwrap();
						let high: i64 = w.next().unwrap().parse().unwrap();
						generate::generate_tree(n, Some(Range { low, high }));
					} else if let Some(mut w) = sub.matches.values_of("float-weight") {
						let low: f64 = w.next().unwrap().parse().unwrap();
						let high: f64 = w.next().unwrap().parse().unwrap();
						generate::generate_tree(n, Some(Range { low, high }));
					} else {
						generate::generate_tree::<i8>(n, None);
					}
				}
				"convex" => {
					let n = sub.matches.value_of("n").unwrap().parse().unwrap();
					if let Some(mut w) = sub.matches.values_of("int-range") {
						let low: i64 = w.next().unwrap().parse().unwrap();
						let high: i64 = w.next().unwrap().parse().unwrap();
						generate::generate_convex(n, Range { low, high });
					} else if let Some(mut w) = sub.matches.values_of("float-range") {
						let low: f64 = w.next().unwrap().parse().unwrap();
						let high: f64 = w.next().unwrap().parse().unwrap();
						generate::generate_convex(n, Range { low, high });
					}
				}
				_ => unreachable!(),
			}
		}
		"sanitize" => {
			let path = sub.matches.value_of("path").unwrap_or(".");
			let exts: Vec<&str> = sub.matches.values_of("ext").unwrap().collect();
			let confirmed = sub.matches.is_present("confirmed");
			sanitize::sanitize(path, exts, confirmed);
		}
		"validate" => {
			let path = sub.matches.value_of("validator").unwrap();
			let paths: Vec<&str> = sub.matches.values_of("paths").unwrap().collect();
			let filter = sub.matches.value_of("filter");
			runner::validate(path, paths, filter).unwrap();
		}
		"eval" => {
			let path = sub.matches.value_of("validator").unwrap();
			let data_dir = sub.matches.value_of("data").unwrap();

			let time_limit: Option<f32> = sub
				.matches
				.value_of("time-limit")
				.map(|s| s.parse().expect("cannot read time limit"));
			let memory_limit: Option<u64> = sub
				.matches
				.value_of("memory-limit")
				.map(|s| s.parse().expect("cannot read memory limit"));
			// runner::eval(path, data_dir).unwrap();
			unimplemented!();
		}
		"new" => {
			// psutil new dir/prob1 --python
			// psutil new ps --download cf 1060H

			let path = Path::new(sub.matches.value_of("path").unwrap());
			std::fs::create_dir_all(path).unwrap();

			{
				let mut f = File::create(path.join("code.cpp")).unwrap();
				f.write_all(include_bytes!("static/code.cpp")).unwrap();
			}

			if let Some(from) = sub.matches.values_of("from") {
				let from: Vec<&str> = from.collect();
				match from[0] {
					"cf" => {
						assert!(from.len() == 2, "cf [contest_id | gym_id | prob_id]");
						let ch = from[1].chars().last().unwrap();
						if ch >= 'A' && ch <= 'Z' {
							let num: u32 = (&from[1][..from[1].len() - 1]).parse().unwrap();
							let p = codeforces::ProblemHandle(num, ch);
							p.download(path).unwrap();
						} else {
							let num: u32 = from[1].parse().unwrap();
							let c = codeforces::ContestHandle(num);
							// c.download(path).unwrap();
						}
					}
					_ => {
						unimplemented!("not supported judge");
					}
				}
			}
		}
		"submit" => {
			sub.matches.value_of("vendor");
			sub.matches.value_of("prob");
			let code_path = sub.matches.value_of("code").unwrap();
			assert!(
				Path::new(code_path).exists(),
				"code does not exist:\n\t=>{}",
				&code_path
			);
			unimplemented!();
		}
		// "acmicpc" => {
		// 	unimplemented!();
		// }
		// "codeforces" => {
		// 	unimplemented!();
		// }
		// "topcoder" => {
		// 	unimplemented!();
		// }
		"visualize" => {
			unimplemented!();
		}
		_ => unreachable!(),
	};
}
