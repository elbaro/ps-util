use libc::{rlimit, rlimit64, setrlimit, setrlimit64};
use std::error::Error;

pub fn set_limits(time: Option<f32>, memory: Option<u64>) -> Result<(), Box<Error>> {
	// time
	let time = time.unwrap_or(1.0).ceil() as u64;

	unsafe {
		if setrlimit(
			libc::RLIMIT_CPU,
			&rlimit {
				rlim_cur: time,
				rlim_max: time + 1,
			},
		) < 0
		{
			return Err("error setting limit".into());
		}

		// core
		if setrlimit(
			libc::RLIMIT_CORE,
			&rlimit {
				rlim_cur: 1,
				rlim_max: 1,
			},
		) < 0
		{
			return Err("error setting limit".into());
		}

		// memory
		if let Some(m) = memory {
			if setrlimit64(
				libc::RLIMIT_AS,
				&rlimit64 {
					rlim_cur: m,
					rlim_max: m,
				},
			) < 0
			{
				return Err("error setting limit".into());
			}
		}

		// opend file
		if setrlimit(
			libc::RLIMIT_NOFILE,
			&rlimit {
				rlim_cur: 4,
				rlim_max: 4,
			},
		) < 0
		{
			return Err("error setting limit".into());
		}
	}

	Ok(())
}
