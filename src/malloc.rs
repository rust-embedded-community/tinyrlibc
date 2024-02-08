//! Rust implementation of C library function `malloc`, `calloc`, `realloc`, and `free`.
//!
//! Copyright (c) Gyungmin Myung <gmmyung@kaist.ac.kr>
//! This file is licensed under the Blue Oak Model Licence 1.0.0

extern crate alloc;
use crate::CSizeT;

// The maximum alignment of any fundamental type. Equivalent to max_align_t
const MAX_ALIGN: usize = 16;

/// Rust implementation of C library function `malloc`
///
/// See [malloc](https://linux.die.net/man/3/malloc) for alignment details.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn malloc(size: CSizeT) -> *mut u8 {
	// size + MAX_ALIGN for to store the size of the allocated memory.
	let layout = alloc::alloc::Layout::from_size_align(size + MAX_ALIGN, MAX_ALIGN).unwrap();
	let ptr = unsafe { alloc::alloc::alloc(layout) };
	if ptr.is_null() {
		return ptr;
	}
	unsafe {
		*(ptr as *mut CSizeT) = size;
	}
	unsafe { ptr.add(MAX_ALIGN) }
}

/// Rust implementation of C library function `calloc`
///
/// See [calloc](https://linux.die.net/man/3/calloc) for alignment details.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn calloc(nmemb: CSizeT, size: CSizeT) -> *mut u8 {
	let total_size = nmemb * size;
	let layout = alloc::alloc::Layout::from_size_align(total_size + MAX_ALIGN, MAX_ALIGN).unwrap();
	let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
	if ptr.is_null() {
		return ptr;
	}
	unsafe {
		*(ptr as *mut CSizeT) = total_size;
	}
	unsafe { ptr.add(MAX_ALIGN) }
}

/// Rust implementation of C library function `realloc`
///
/// See [realloc](https://linux.die.net/man/3/realloc) for alignment details.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut u8, size: CSizeT) -> *mut u8 {
	if ptr.is_null() {
		return malloc(size);
	}
	let old_size = unsafe { *(ptr.sub(MAX_ALIGN) as *mut CSizeT) };
	let layout = alloc::alloc::Layout::from_size_align(old_size + MAX_ALIGN, MAX_ALIGN).unwrap();
	let new_ptr = unsafe { alloc::alloc::realloc(ptr.sub(MAX_ALIGN), layout, size + MAX_ALIGN) };
	if new_ptr.is_null() {
		return new_ptr;
	}
	unsafe {
		*(new_ptr as *mut CSizeT) = size;
	}
	unsafe { new_ptr.add(MAX_ALIGN) }
}

/// Rust implementation of C library function `free`
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
	if ptr.is_null() {
		return;
	}
	let old_size = unsafe { *(ptr.sub(MAX_ALIGN) as *mut CSizeT) };
	let layout = alloc::alloc::Layout::from_size_align(old_size + MAX_ALIGN, MAX_ALIGN).unwrap();
	unsafe { alloc::alloc::dealloc(ptr.sub(MAX_ALIGN), layout) };
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_malloc() {
		let ptr = unsafe { malloc(10) };
		assert!(!ptr.is_null());
		unsafe {
			assert_eq!(*(ptr.sub(MAX_ALIGN) as *mut CSizeT), 10);
			(0..10).for_each(|i| {
				*ptr.add(i) = i as u8;
			});
			(0..10).for_each(|i| {
				assert_eq!(*ptr.add(i), i as u8);
			});
		}
		unsafe { free(ptr) };
	}

	#[test]
	fn test_calloc() {
		let ptr = unsafe { calloc(10, 10) };
		assert!(!ptr.is_null());
		unsafe {
			assert_eq!(*(ptr.sub(MAX_ALIGN) as *mut CSizeT), 100);
			(0..100).for_each(|i| {
				assert_eq!(*ptr.add(i), 0);
			});
			(0..100).for_each(|i| {
				*ptr.add(i) = i as u8;
			});
			(0..100).for_each(|i| {
				assert_eq!(*ptr.add(i), i as u8);
			});
		}
		unsafe { free(ptr) };
	}

	#[test]
	fn test_realloc() {
		let ptr = unsafe { malloc(10) };
		assert!(!ptr.is_null());
		unsafe {
			assert_eq!(*(ptr.sub(MAX_ALIGN) as *mut CSizeT), 10);
			(0..10).for_each(|i| {
				*ptr.add(i) = i as u8;
			});
			(0..10).for_each(|i| {
				assert_eq!(*ptr.add(i), i as u8);
			});
		}
		let ptr = unsafe { realloc(ptr, 20) };
		assert!(!ptr.is_null());
		unsafe {
			assert_eq!(*(ptr.sub(MAX_ALIGN) as *mut CSizeT), 20);
			(0..10).for_each(|i| {
				assert_eq!(*ptr.add(i), i as u8);
			});
			(10..20).for_each(|i| {
				*ptr.add(i) = i as u8;
			});
			(10..20).for_each(|i| {
				assert_eq!(*ptr.add(i), i as u8);
			});
		}
		unsafe { free(ptr) };
	}
}
