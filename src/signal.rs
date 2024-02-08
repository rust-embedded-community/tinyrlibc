//! Rust implementation of the C standard library's `signal` related functions.
//!
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::cell::RefCell;
use critical_section::Mutex;

// Signal hanling is emulated by the `critical-section` crate.
static SIGNAL_HANDLERS: Mutex<RefCell<[extern "C" fn(i32); 16]>> =
	Mutex::new(RefCell::new([default_handler; 16]));

const SIG_DFL: usize = 0;
const SIG_IGN: usize = 1;
const SIG_ERR: isize = -1;

// Only ANSI C signals are now supported.
// SIGSEGV, SIGILL, SIGFPE are not supported on bare metal, but handlers are invoked when raise() is called.
// TODO: Support SIGSEGV, SIGILL, SIGFPE by using the `cortex-m-rt` or `riscv-rt` crate.
const SIGTERM: i32 = 15;
const SIGSEGV: i32 = 11;
const SIGINT: i32 = 2;
const SIGILL: i32 = 4;
const SIGABRT: i32 = 6;
const SIGFPE: i32 = 8;

const SIGNALS: [i32; 6] = [SIGTERM, SIGSEGV, SIGINT, SIGILL, SIGABRT, SIGFPE];

extern "C" fn ignore_handler(_sig: i32) {}

extern "C" fn default_handler(_sig: i32) {
	// TODO: This should call core::intrinsics::abort() but that's unstable.
	panic!("Aborted");
}

/// Rust implementation of the C standard library's `signal` function.
#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub unsafe extern "C" fn signal(sig: i32, handler: extern "C" fn(i32)) -> extern "C" fn(i32) {
	if SIGNALS.iter().all(|&s| s != sig) {
		return core::mem::transmute(SIG_ERR);
	}
	critical_section::with(|cs| {
		let mut handlers = SIGNAL_HANDLERS.borrow(cs).borrow_mut();
		let old_handler = handlers[sig as usize];
		handlers[sig as usize] = handler;
		old_handler
	})
}

/// Rust implementation of the C standard library's `raise` function.
#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub unsafe extern "C" fn raise(sig: i32) -> i32 {
	if SIGNALS.iter().all(|&s| s != sig) {
		return -1;
	}
	critical_section::with(|cs| {
		let handlers = SIGNAL_HANDLERS.borrow(cs).borrow();
		let handler = handlers[sig as usize];
		match handler as usize {
			SIG_DFL => default_handler(sig),
			SIG_IGN => ignore_handler(sig),
			_ => handler(sig),
		}
		0
	})
}

#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub extern "C" fn abort() {
	unsafe {
		raise(SIGABRT);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_signal() {
		unsafe {
			static COUNT: Mutex<RefCell<i32>> = Mutex::new(RefCell::new(0));
			extern "C" fn count_handler(_sig: i32) {
				critical_section::with(|cs| {
					let mut count = COUNT.borrow(cs).borrow_mut();
					*count += 1;
				});
			}
			println!("default_handler: {}", default_handler as usize);
			println!("count_handler: {}", count_handler as usize);
			let old_handler = signal(SIGTERM, count_handler);
			println!("old_handler: {}", old_handler as usize);
			println!("count_handler: {}", count_handler as usize);
			println!("default_handler: {}", default_handler as usize);
			assert_eq!(old_handler as usize, default_handler as usize);
			(0..10).for_each(|_| {
				raise(SIGTERM);
			});
			let old_handler = signal(SIGTERM, default_handler);
			critical_section::with(|cs| {
				let count = COUNT.borrow(cs).borrow();
				assert_eq!(*count, 10);
			});
			assert_eq!(old_handler as usize, count_handler as usize);
		}
	}

	#[test]
	fn test_abort() {
		let result = std::panic::catch_unwind(|| {
			abort();
		});
		assert!(result.is_err());
	}

	#[test]
	fn test_abort_signal() {
		static TRIGGER: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
		extern "C" fn trigger_handler(_sig: i32) {
			critical_section::with(|cs| {
				let mut trigger = TRIGGER.borrow(cs).borrow_mut();
				*trigger = true;
			});
		}
		unsafe { signal(SIGABRT, trigger_handler) };
		let result = std::panic::catch_unwind(|| {
			abort();
		});
		assert!(result.is_ok());
		critical_section::with(|cs| {
			let trigger = TRIGGER.borrow(cs).borrow();
			assert!(*trigger);
		});
	}
}
