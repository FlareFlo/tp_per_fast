use std::{
	sync::atomic::{AtomicUsize, Ordering::Relaxed},
	thread,
	thread::sleep,
	time::Duration,
};

static REQS: AtomicUsize = AtomicUsize::new(0);

pub fn log_req() {
	REQS.fetch_add(1, Relaxed);
}

pub fn spawn_stattrack() {
	thread::spawn(|| loop {
		let last = REQS.load(Relaxed) / 1000;
		REQS.store(0, Relaxed);
		println!("{last}k/s");
		sleep(Duration::from_secs(1));
	});
}
