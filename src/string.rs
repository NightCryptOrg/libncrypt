use std::ffi::c_char;
use serde::{Serialize, Deserialize};

/// An FFI-opaque + FFI-immutable string type with *optional* NUL-termination (See [TerminatedNString]).
pub trait NString<T> {
	/// Allocate the string over FFI. Len should **not** include optional NUL byte. Free using [NString::free]
	extern fn malloc(str: *const T, len: usize) -> *mut Self;
	/// Free the string allocated using [NString::malloc]
	extern fn free(v: *mut Self);

	/// Get the string's start address. If [Self] impl [TerminatedNString],
	/// then this pointer is a C string.
	///
	/// NULL is returned if an error occurs
	extern fn get(v: *mut Self) -> *mut T;


	/// Get the string's length, **not** including the NUL terminator if [Self] impl [TerminatedNString].
	extern fn get_len(v: *const Self) -> usize;
}

/// An NString that guarantees that get() will return either a valid C string
/// or a NULL ptr
pub trait TerminatedNString<T>: NString<T> {}

/// NCString - An NUL-terminated text string with FFI interop via [NString].
#[repr(transparent)]
#[derive(Serialize, Deserialize)]
pub struct NCString(String);

impl From<String> for NCString {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl TerminatedNString<c_char> for NCString {}
impl NString<c_char> for NCString {
	#[export_name = "NCString_malloc"]
	extern fn malloc(str: *const c_char, len: usize) -> *mut Self {
		let self_: Self = if !str.is_null() {

			// Copy string from str -> self_
			let mut chars: Vec<u8> = Vec::with_capacity(len + 1);
			for i in 0..len {
				chars.push(unsafe { *str.add(i) } as u8);
			}
			chars.push(0);

			// Validate as utf8
			match String::from_utf8(chars) {
				Ok(s) => Self(s),
				Err(err) => {
					if cfg!(debug) {
						eprintln!("Error creating NCString: {}", err);
					}
					return std::ptr::null_mut();
				}
			}
		} else {

			// Fill with repeating 'NC' as a zero value
			let mut chars: Vec<u8> = Vec::with_capacity(len + 1);
			for i in 0..len {
				chars.push(if i % 2 == 0 { b'N' } else { b'C' });
			}
			chars.push(0);

			Self(unsafe { String::from_utf8_unchecked(chars) })
		};

		// Box the NCString and remove RAII.
		// The caller is responsible for using NCString_free() to deallocate
		Box::into_raw(Box::new(self_))
	}

	#[export_name = "NCString_free"]
	extern fn free(v: *mut Self) {
		// Reconstruct the Box<NCString>, which is then dropped
		let _self = unsafe { Box::from_raw(v) };
	}

	#[export_name = "NCString_get"]
	extern fn get(v: *mut Self) -> *mut c_char {
		let mut self_ = unsafe { Box::from_raw(v) };
		let str = self_.0.as_mut_ptr() as *mut c_char;

		// Return ownership of v to the caller
		Box::leak(self_);

		// Return the str ptr
		str
	}

	#[export_name = "NCString_get_len"]
	extern fn get_len(v: *const Self) -> usize {
		// We need to cast to *mut for from_raw to work, but the only mutation we're doing is
		// re-leaking the Box. Therefore, despite looking wild, the following cast is sound
		let self_ = unsafe { Box::from_raw(v as *mut Self) };
		let len = self_.0.len();

		// Return ownership to caller
		Box::leak(self_);

		// Return the str len (excluding NUL terminator)
		len - 1
	}
}

/// NBString - A byte string with FFI interop via [NString]
#[repr(transparent)]
#[derive(Serialize, Deserialize)]
pub struct NBString(Vec<u8>);

impl From<Vec<u8>> for NBString {
	fn from(value: Vec<u8>) -> Self {
		Self(value)
	}
}

impl NString<u8> for NBString {
	#[export_name = "NBString_malloc"]
	extern fn malloc(data: *const u8, len: usize) -> *mut Self {
		let self_: Self = if !data.is_null() {

			// Copy string from data -> self_
			let mut bytes: Vec<u8> = Vec::with_capacity(len);
			for i in 0..len {
				bytes.push(unsafe { *data.add(i) });
			}

			Self(bytes)
		} else {
			Self(vec![0u8; len])
		};

		Box::into_raw(Box::new(self_))
	}

	#[export_name = "NBString_free"]
	extern fn free(v: *mut Self) {
		// Reconstruct and drop
		let _self = unsafe { Box::from_raw(v) };
	}

	#[export_name = "NBString_get"]
	extern fn get(v: *mut Self) -> *mut u8 {
		let mut self_ = unsafe { Box::from_raw(v) };
		let data = self_.0.as_mut_ptr();

		// Re-leak
		Box::leak(self_);

		data
	}

	#[export_name = "NBString_get_len"]
	extern fn get_len(v: *const Self) -> usize {
		// See [NCString::get_len] for notes  about the following cast to *mut.
		// TL;DR we're mutating the Box, not the value, so the cast is sound
		let self_ = unsafe { Box::from_raw(v as *mut Self) };
		let len = self_.0.len();

		Box::leak(self_);

		len
	}
}
