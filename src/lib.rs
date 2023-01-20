#![no_std]
#![cfg_attr(
	feature = "const",
	feature(const_option, const_nonnull_new, const_ptr_as_ref, const_mut_refs)
)]

mod ptr;
pub use ptr::*;