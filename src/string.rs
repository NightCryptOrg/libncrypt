use std::{ffi::{c_char, c_int, CString}, mem};
use std::string;
use serde::{Serialize, Deserialize};

/// NCString - An owned NUL-terminated text string with associated length.
///
/// SAFETY: An NCString used via FFI must be initialized with `NCString_init()` before use,
/// and deinitialized with `NCString_deinit()` after use
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct NCString {
	#[serde(skip, default = "std::ptr::null_mut")]
	pub str: *mut c_char,
	pub len: usize,
	#[serde(with = "serde_bytes")]
	inner: Option<Box<[u8]>> // [std::ffi::CString] is used internally for validation
}

impl NCString {
	/// Deinitialize an NCString (unecessary to call from Rust where we have RAII)
	#[export_name = "NCString_deinit"]
	pub extern fn deinit(&mut self) {
		if let Some(inner) = &mut self.inner {
			self.str = std::ptr::null_mut();
			self.len = 0;
			mem::drop(mem::take(inner));
		}
	}

	/// Initialize an NCString (copies the contents of str)
	/// (uneccessary to call from Rust, as NCString implements [From]<[String]>)
	#[export_name = "NCString_init"]
	pub extern fn init(&mut self, str: *mut c_char, len: usize) -> c_int {
		let mut raw: Vec<u8> = Vec::with_capacity(len);
		for i in 0..len {
			raw.push(unsafe { *str.add(i) as u8 });
		}

		match CString::new(raw) { // Validate that string doesn't contain internal NUL chars
			Ok(cstr) => {
				let mut inner = cstr.into_bytes_with_nul().into_boxed_slice();
				self.str = inner.as_mut_ptr() as *mut c_char;
				self.inner = Some(inner);
				self.len = len;
				0
			}
			Err(_) => 1
		}
	}
}

impl From<string::String> for NCString {
	fn from(value: string::String) -> Self {
		let len = value.len();
		let mut inner = value.into_bytes().into_boxed_slice();
			Self {
				str: inner.as_mut_ptr() as *mut c_char,
				inner: Some(inner),
				len
		}
	}
}

/// NBString - An owned binary string with associated length
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct NBString {
	#[serde(skip, default = "std::ptr::null_mut")]
	pub data: *mut u8,
	pub len: usize,
	#[serde(with = "serde_bytes")]
	inner: Option<Box<[u8]>>
}
