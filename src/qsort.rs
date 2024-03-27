//! Rust implementation of C library function `qsort`
//!
//! This code snippet is copied from relibc.
//! MIT license COPYRIGHT (c) 2018 Redox OS

use core::num::NonZeroUsize;

use crate::{CChar, CInt, CSizeT, CVoid};

/// Rust implementation of C library function `qsort`
#[cfg_attr(feature = "qsort", no_mangle)]
pub unsafe extern "C" fn qsort(
	base: *mut CVoid,
	nel: CSizeT,
	width: CSizeT,
	compar: Option<extern "C" fn(*const CVoid, *const CVoid) -> CInt>,
) {
	if let Some(comp) = compar {
		if nel > 0 {
			introsort(base as *mut CChar, nel, width, comp);
		}
	}
}

fn introsort(
	base: *mut CChar,
	nel: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) {
	let maxdepth = 2 * nel.ilog2();
	introsort_helper(base, nel, width, maxdepth as CSizeT, comp);
}

fn introsort_helper(
	mut base: *mut CChar,
	mut nel: CSizeT,
	width: CSizeT,
	mut maxdepth: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) {
	const THRESHOLD: CSizeT = 16;

	// this loop is a trick to save stack space because TCO is not a thing in Rustland
	// basically, we just change the arguments and loop rather than recursing for the second call
	// to introsort_helper()
	loop {
		if nel < THRESHOLD {
			insertion_sort(base, nel, width, comp);
			break;
		} else if nel > 1 {
			if maxdepth == 0 {
				heapsort(base, nel, width, comp);
				break;
			} else {
				let (left, right) = partition(base, nel, width, comp);
				let right_base = unsafe { base.add((right + 1) * width) };
				let right_nel = nel - (right + 1);
				maxdepth -= 1;
				if left < nel - right {
					introsort_helper(base, left, width, maxdepth, comp);
					base = right_base;
					nel = right_nel;
				} else {
					introsort_helper(right_base, right_nel, width, maxdepth, comp);
					nel = left;
				}
			}
		}
	}
}

fn insertion_sort(
	base: *mut CChar,
	nel: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) {
	for i in 0..nel {
		for j in (0..i).rev() {
			let current = unsafe { base.add(j * width) };
			let prev = unsafe { base.add((j + 1) * width) };
			if comp(current as *const CVoid, prev as *const CVoid) > 0 {
				swap(current, prev, width);
			} else {
				break;
			}
		}
	}
}

fn heapsort(
	base: *mut CChar,
	nel: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) {
	heapify(base, nel, width, comp);

	let mut end = nel - 1;
	while end > 0 {
		let end_ptr = unsafe { base.add(end * width) };
		swap(end_ptr, base, width);
		end -= 1;
		heap_sift_down(base, 0, end, width, comp);
	}
}

fn heapify(
	base: *mut CChar,
	nel: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) {
	// we start at the last parent in the heap (the parent of the last child)
	let last_parent = (nel - 2) / 2;

	for start in (0..=last_parent).rev() {
		heap_sift_down(base, start, nel - 1, width, comp);
	}
}

fn heap_sift_down(
	base: *mut CChar,
	start: CSizeT,
	end: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) {
	// get the left child of the node at the given index
	let left_child = |idx| 2 * idx + 1;

	let mut root = start;

	while left_child(root) <= end {
		let child = left_child(root);
		let mut swap_idx = root;

		let root_ptr = unsafe { base.add(root * width) };
		let mut swap_ptr = unsafe { base.add(swap_idx * width) };
		let first_child_ptr = unsafe { base.add(child * width) };
		let second_child_ptr = unsafe { base.add((child + 1) * width) };

		if comp(swap_ptr as *const CVoid, first_child_ptr as *const CVoid) < 0 {
			swap_idx = child;
			swap_ptr = first_child_ptr;
		}
		if child < end && comp(swap_ptr as *const CVoid, second_child_ptr as *const CVoid) < 0 {
			swap_idx = child + 1;
			swap_ptr = second_child_ptr;
		}

		if swap_idx == root {
			break;
		} else {
			swap(root_ptr, swap_ptr, width);
			root = swap_idx;
		}
	}
}

#[inline]
fn partition(
	base: *mut CChar,
	nel: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) -> (CSizeT, CSizeT) {
	// calculate the median of the first, middle, and last elements and use it as the pivot
	// to do fewer comparisons, also swap the elements into their correct positions
	let mut pivot = median_of_three(base, nel, width, comp);

	let mut i = 1;
	let mut j = 1;
	let mut n = nel - 2;

	// use this to deal with the Dutch national flag problem
	while j <= n {
		let i_ptr = unsafe { base.add(i * width) };
		let j_ptr = unsafe { base.add(j * width) };
		let n_ptr = unsafe { base.add(n * width) };
		let pivot_ptr = unsafe { base.add(pivot * width) };

		let comparison = comp(j_ptr as *const CVoid, pivot_ptr as *const CVoid);
		match comparison.cmp(&0) {
			core::cmp::Ordering::Less => {
				swap(i_ptr, j_ptr, width);
				if i == pivot {
					pivot = j;
				}
				i += 1;
				j += 1;
			}
			core::cmp::Ordering::Greater => {
				swap(j_ptr, n_ptr, width);
				if n == pivot {
					pivot = j;
				}
				n -= 1;
			}
			core::cmp::Ordering::Equal => {
				j += 1;
			}
		}
	}

	(i, n)
}

fn median_of_three(
	base: *mut CChar,
	nel: CSizeT,
	width: CSizeT,
	comp: extern "C" fn(*const CVoid, *const CVoid) -> CInt,
) -> CSizeT {
	let pivot = nel / 2;

	let mid = unsafe { base.add(pivot * width) };
	let last = unsafe { base.add((nel - 1) * width) };
	if comp(mid as *const CVoid, base as *const CVoid) < 0 {
		swap(mid, base, width);
	}
	if comp(last as *const CVoid, mid as *const CVoid) < 0 {
		swap(mid, last, width);
		if comp(mid as *const CVoid, base as *const CVoid) < 0 {
			swap(mid, base, width);
		}
	}

	pivot
}

#[inline]
fn swap(mut ptr1: *mut CChar, mut ptr2: *mut CChar, mut width: CSizeT) {
	use core::mem;

	if ptr1 == ptr2 {
		return;
	}
	const BUFSIZE: usize = 128;

	let mut buffer = mem::MaybeUninit::<[CChar; BUFSIZE]>::uninit();
	while width > 0 {
		let copy_size = BUFSIZE.min(width);
		let buf = buffer.as_mut_ptr() as *mut CChar;

		unsafe {
			buf.copy_from_nonoverlapping(ptr1, copy_size);
			ptr1.copy_from_nonoverlapping(ptr2, copy_size);
			ptr2.copy_from_nonoverlapping(buf, copy_size);

			ptr1 = ptr1.add(copy_size);
			ptr2 = ptr2.add(copy_size);
		}
		width -= copy_size as CSizeT;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	extern "C" fn comp(a: *const CVoid, b: *const CVoid) -> CInt {
		unsafe { *(a as *const i32) - *(b as *const i32) }
	}

	#[test]
	fn identity() {
		let mut array: Vec<_> = (0..1000).collect();
		let orig = array.clone();

		unsafe {
			qsort(
				array.as_mut_ptr() as *mut CVoid,
				array.len() as CSizeT,
				std::mem::size_of::<i32>() as CSizeT,
				Some(comp),
			);
		}

		assert_eq!(array, orig);
	}

	#[test]
	fn identity_heapsort() {
		let mut array: Vec<_> = (0..1000).collect();
		let orig = array.clone();

		heapsort(
			array.as_mut_ptr() as *mut CChar,
			array.len() as CSizeT,
			std::mem::size_of::<i32>() as CSizeT,
			comp,
		);

		assert_eq!(array, orig);
	}

	#[test]
	fn reverse() {
		let mut array: Vec<_> = (0..1000).collect();
		array.reverse();
		let orig: Vec<_> = (0..1000).collect();

		unsafe {
			qsort(
				array.as_mut_ptr() as *mut CVoid,
				array.len() as CSizeT,
				std::mem::size_of::<i32>() as CSizeT,
				Some(comp),
			)
		}

		assert_eq!(array, orig);
	}

	#[test]
	fn reverse_heapsort() {
		let mut array: Vec<_> = (0..1000).collect();
		array.reverse();
		let orig: Vec<_> = (0..1000).collect();

		heapsort(
			array.as_mut_ptr() as *mut CChar,
			array.len() as CSizeT,
			std::mem::size_of::<i32>() as CSizeT,
			comp,
		);

		assert_eq!(array, orig);
	}
}
