//! Rust implementation of the C standard library's `signal` related functions.
//!
//! Copyright (c) Gyungmin Myung <gmmyung@kaist.ac.kr>
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::{cell::RefCell, default};
use critical_section::Mutex;

// Signal handling is emulated by the `critical-section` crate.
static SIGNAL_HANDLERS: Mutex<RefCell<[Option<extern "C" fn(i32)>; 16]>> =
	Mutex::new(RefCell::new([None; 16]));

type SignalHandler = Option<extern "C" fn(i32)>;

const SIG_DFL: usize = 0;
const SIG_IGN: usize = 1;
const SIG_ERR: isize = -1;

// This is required because rust doesn't support 0, 1, -1 as a function pointer.
fn signal_handler(ptr: usize) -> SignalHandler {
	unsafe { core::mem::transmute(ptr) }
}

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

fn ignore_handler(_sig: i32) {}

fn default_handler(_sig: i32) {
	// TODO: This should call core::intrinsics::abort() but that's unstable.
	panic!("Aborted");
}

/// Rust implementation of the C standard library's `signal` function.
#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub unsafe extern "C" fn signal(sig: i32, handler: SignalHandler) -> SignalHandler {
	if SIGNALS.iter().all(|&s| s != sig) {
		return signal_handler(SIG_ERR as usize);
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
		if handler == signal_handler(SIG_DFL) {
			default_handler(sig);
		} else if handler == signal_handler(SIG_IGN) {
			ignore_handler(sig);
		} else {
			handler.unwrap()(sig);
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
		critical_section::with(|cs| unsafe {
			static mut COUNT: usize = 0;
			extern "C" fn count_handler(_sig: i32) {
				unsafe { COUNT += 1 };
			}
			dbg!(SIGNAL_HANDLERS.borrow(cs).borrow());
			let old_handler = signal(SIGTERM, Some(count_handler));
			assert_eq!(old_handler, signal_handler(SIG_DFL));
			(0..10).for_each(|_| {
				raise(SIGTERM);
			});
			let old_handler = signal(SIGTERM, core::mem::transmute(SIG_DFL));
			assert_eq!(COUNT, 10);
			assert_eq!(
				old_handler.unwrap() as usize,
				Some(count_handler).unwrap() as usize
			);
		});
	}

	#[test]
	fn test_abort() {
		let result = std::panic::catch_unwind(|| {
			abort();
		});
		assert!(result.is_err());
	}

	#[test]
	fn test_signal_error() {
		let err = unsafe { signal(1000, signal_handler(SIG_DFL)) };
		assert_eq!(err, signal_handler(SIG_ERR as usize));
	}

	#[test]
	fn test_raise() {
		let result = std::panic::catch_unwind(|| {
			unsafe { raise(SIGTERM) };
		});
		assert!(result.is_err());
	}

	#[test]
	fn test_ignore() {
		critical_section::with(|_cs| unsafe {
			let old_handler = signal(SIGTERM, core::mem::transmute(SIG_IGN));
			assert_eq!(old_handler, signal_handler(SIG_DFL));
			let result = std::panic::catch_unwind(|| {
				raise(SIGTERM);
			});
			assert!(result.is_ok());
			let old_handler = signal(SIGTERM, core::mem::transmute(SIG_DFL));
			assert_eq!(old_handler, signal_handler(SIG_IGN));
		});
	}

	#[test]
	fn test_raise_error() {
		let result = std::panic::catch_unwind(|| {
			assert!(unsafe { raise(1000) == -1 });
		});
		assert!(result.is_ok());
	}
}
