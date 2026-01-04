//! Rust implementation of the C standard library's `signal` related functions.
//!
//! Copyright (c) Gyungmin Myung <gmmyung@kaist.ac.kr>
//! Licensed under the Blue Oak Model Licence 1.0.0

use core::{cell::RefCell, default};
use portable_atomic::{AtomicUsize, Ordering};

/// An initialiser for our array.
///
/// We turn off the clippy warning because it's wrong - and there's no other
/// way to initialise an array of atomics.
#[allow(clippy::declare_interior_mutable_const)]
const SIG_DFL_ATOMIC: AtomicUsize = AtomicUsize::new(SIG_DFL);

/// Our array of registered signal handlers.
///
/// Signals in C are either 0, 1, -1, or a function pointer.
///
/// We cast function pointers into `usize` so they can be stored in this array.
static SIGNAL_HANDLERS: [AtomicUsize; 16] = [SIG_DFL_ATOMIC; 16];

/// A signal handler - either a function pointer or a magic integer.
pub type SignalHandler = usize;

/// Indicates we should use the default signal handler
const SIG_DFL: usize = 0;

/// Indicates we should use the default signal handler
const SIG_IGN: usize = 1;

/// Indicates we should use the default signal handler
const SIG_ERR: usize = usize::MAX;

/// The TERM signal
const SIGTERM: i32 = 15;

/// The SEGV signal
const SIGSEGV: i32 = 11;

/// The INT signal
const SIGINT: i32 = 2;

/// The ILL signal
const SIGILL: i32 = 4;

/// The ABRT signal
const SIGABRT: i32 = 6;

/// The FPE signal
const SIGFPE: i32 = 8;

/// The list of support signals.
///
/// Only ANSI C signals are now supported.
///
/// SIGSEGV, SIGILL, SIGFPE are not supported on bare metal, but handlers are
/// invoked when raise() is called.
///
/// We will index `SIGNAL_HANDLERS` by any integer in this list, so ensure that
/// the array is made larger if required.
///
/// TODO: Support SIGSEGV, SIGILL, SIGFPE by using the `cortex-m-rt` or
/// `riscv-rt` crate.
const SIGNALS: [i32; 6] = [SIGTERM, SIGSEGV, SIGINT, SIGILL, SIGABRT, SIGFPE];

/// An empty handler function that does nothing
fn ignore_handler(_sig: i32) {}

/// The default handler functions
///
/// Performs a panic.
fn default_handler(_sig: i32) {
	// TODO: This should call core::intrinsics::abort() but that's unstable.
	panic!("Aborted");
}

/// Rust implementation of the C standard library's `signal` function.
///
/// Using `not(test)` ensures we don't replace the actual OS `signal` function
/// when running tests!
#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub unsafe extern "C" fn signal(sig: i32, handler: SignalHandler) -> SignalHandler {
	if !SIGNALS.contains(&sig) {
		return SIG_ERR;
	}
	SIGNAL_HANDLERS[sig as usize].swap(handler, Ordering::Relaxed)
}

/// Rust implementation of the C standard library's `raise` function.
///
/// Using `not(test)` ensures we don't replace the actual OS `raise` function
/// when running tests!
#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub extern "C" fn raise(sig: i32) -> i32 {
	if !SIGNALS.contains(&sig) {
		return -1;
	}
	let handler = SIGNAL_HANDLERS[sig as usize].load(Ordering::Relaxed);
	match handler {
		SIG_DFL => {
			default_handler(sig);
		}
		SIG_IGN => {
			ignore_handler(sig);
		}
		_ => unsafe {
			let handler_fn: unsafe extern "C" fn(core::ffi::c_int) = core::mem::transmute(handler);
			handler_fn(sig);
		},
	}
	0
}

#[cfg_attr(all(not(test), feature = "signal"), no_mangle)]
pub extern "C" fn abort() {
	raise(SIGABRT);
}

#[cfg(test)]
mod tests {
	use super::*;

	struct State {
		inner: std::sync::Mutex<()>,
	}

	impl State {
		fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
			// Ensure we have exclusive access
			let guard = self.inner.lock().unwrap();
			// Reset the global signal handler list to defaults
			for sig in SIGNAL_HANDLERS.iter() {
				sig.store(SIG_DFL, Ordering::SeqCst);
			}
			guard
		}
	}

	/// Used to ensure we don't run multiple signal test concurrently, because
	/// they share some global state.
	///
	/// If a test fails, the lock will be poisoned and all subsequent tests will
	/// fail.
	static TEST_LOCK: State = State {
		inner: std::sync::Mutex::new(()),
	};

	#[test]
	fn test_signal() {
		let _guard = TEST_LOCK.lock();
		static COUNT: AtomicUsize = AtomicUsize::new(0);
		extern "C" fn count_handler(_sig: i32) {
			COUNT.fetch_add(1, Ordering::Relaxed);
		}
		let count_handler_ptr = count_handler as *const fn(i32) as usize;
		let old_handler = unsafe { signal(SIGTERM, count_handler_ptr) };
		assert_eq!(old_handler, SIG_DFL);
		(0..10).for_each(|_| {
			raise(SIGTERM);
		});
		let old_handler = unsafe { signal(SIGTERM, SIG_DFL) };
		assert_eq!(COUNT.load(Ordering::Relaxed), 10);
		assert_eq!(old_handler, count_handler_ptr);
	}

	#[test]
	fn test_abort() {
		let _guard = TEST_LOCK.lock();
		let result = std::panic::catch_unwind(|| {
			abort();
		});
		assert!(result.is_err());
	}

	#[test]
	fn test_signal_error() {
		let _guard = TEST_LOCK.lock();
		let err = unsafe { signal(1000, SIG_DFL) };
		assert_eq!(err, SIG_ERR);
	}

	#[test]
	fn test_raise() {
		let result = std::panic::catch_unwind(|| raise(SIGTERM));
		assert!(result.is_err());
	}

	#[test]
	fn test_ignore() {
		let _guard = TEST_LOCK.lock();
		let old_handler = unsafe { signal(SIGTERM, SIG_IGN) };
		assert_eq!(old_handler, SIG_DFL);
		// Shouldn't cause a panic
		raise(SIGTERM);
		let old_handler = unsafe { signal(SIGTERM, SIG_DFL) };
		assert_eq!(old_handler, SIG_IGN);
	}

	#[test]
	fn test_raise_error() {
		assert!(raise(1000) == -1);
	}
}
