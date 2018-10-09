use libc::{rlimit, rlimit64, setrlimit, setrlimit64};
use std::error::Error;

#[derive(Clone)]
pub struct Limitation {
	pub time: Option<f32>,
	pub memory_mb: Option<u64>,
}

pub fn set_limits(limit: &Limitation) -> Result<(), Box<Error>> {
	// time
	let time = limit.time.unwrap_or(1.0).ceil() as u64;

	unsafe {
		// cpu time
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

		// wallclock

		// let mut action: libc::sigaction = std::mem::zeroed();
		// action.sa_sigaction = self_terminate as usize;
		// action.sa_flags = libc::SA_RESETHAND | libc::SA_RESTART;
		// action.sa_mask = {
		// 	let mut mask: libc::sigset_t = std::mem::uninitialized();
		// 	libc::sigemptyset(&mut mask);
		// 	if libc::sigaddset(&mut mask, libc::SIGALRM) != 0 {
		// 		return Err("error setting wallclock limit".into());
		// 	}
		// 	if libc::sigaddset(&mut mask, libc::SIGTERM) != 0 {
		// 		return Err("error setting wallclock limit".into());
		// 	}
		// 	mask
		// };
		// if libc::sigaction(libc::SIGALRM, &action, std::ptr::null_mut()) != 0 {
		// 	return Err("error setting wallclock limit".into());
		// }

		// single thread
		if setrlimit(
			libc::RLIMIT_NPROC,
			&rlimit {
				rlim_cur: 1,
				rlim_max: 1,
			},
		) < 0
		{
			return Err("error setting limit".into());
		}

		// memory
		if let Some(m) = limit.memory_mb {
			if setrlimit64(
				libc::RLIMIT_AS,
				&rlimit64 {
					rlim_cur: m * 1024 * 1024,
					rlim_max: m * 1024 * 1024,
				},
			) < 0
			{
				return Err("error setting limit".into());
			}
		}

		// opend file
		// problem: inherit judge's fd, shared libs, etc.
		// if setrlimit(
		// 	libc::RLIMIT_NOFILE,
		// 	&rlimit {
		// 		rlim_cur: 4,
		// 		rlim_max: 4,
		// 	},
		// ) < 0
		// {
		// 	return Err("error setting limit".into());
		// }
	}

	Ok(())
}

// extern "C" fn self_terminate(sig: libc::c_int) {
// 	if sig == libc::SIGALRM {
// 		eprintln!("[child process] self destroying");
// 		std::process::abort();
// 	}
// }
