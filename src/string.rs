use std::{ffi::{c_char, c_uchar, c_int, CString}, mem};
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
	buf: Option<Box<[u8]>> // [std::ffi::CString] is used internally for validation
}

impl NCString {
	/// Initialize an NCString (copies the contents of str)
	/// (uneccessary to call from Rust - NCString implements [From]<[String]>)
	///
	/// Returns 0 on success or 1 if *str contains inner NUL characters
	#[export_name = "NCString_init"]
	pub extern fn init(&mut self, str: *mut c_char, len: usize) -> c_int {
		let mut raw: Vec<u8> = Vec::with_capacity(len);
		for i in 0..len {
			raw.push(unsafe { *str.add(i) as u8 });
		}

		match CString::new(raw) { // Validate that string doesn't contain internal NUL chars
			Ok(cstr) => {
				let mut buf = cstr.into_bytes_with_nul().into_boxed_slice();
				self.str = buf.as_mut_ptr() as *mut c_char;
				self.buf = Some(buf);
				self.len = len;
				0
			}
			Err(_) => 1
		}
	}

	/// Deinitialize an NCString (unecessary to call from Rust - RAII accomplishes deinitialization)
	#[export_name = "NCString_deinit"]
	pub extern fn deinit(&mut self) {
		if let Some(buf) = &mut self.buf {
			self.str = std::ptr::null_mut();
			self.len = 0;
			mem::drop(mem::take(buf));
		}
	}
}

impl From<string::String> for NCString {
	fn from(value: string::String) -> Self {
		let len = value.len();
		let mut buf = value.into_bytes().into_boxed_slice();
			Self {
				str: buf.as_mut_ptr() as *mut c_char,
				buf: Some(buf),
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
	buf: Option<Box<[u8]>>
}


impl NBString {
	/// Initialize an NBString (copies to the contents of data)
	/// (uneccessary to call from Rust - NBString implements [From]<[Vec<u8>]> and [From]<[\[u8\]]>)
	#[export_name = "NBString_init"]
	pub extern fn init(&mut self, data: *mut c_uchar, len: usize) {
		let mut raw: Vec<u8> = Vec::with_capacity(len);
		for i in 0..len {
			raw.push(unsafe { *data.add(i) });
		}

		let mut buf = raw.into_boxed_slice();
		self.data = buf.as_mut_ptr();
		self.buf = Some(buf);
		self.len = len;
	}

	/// Deinitialize an NBString (unneccessary to call from Rust - RAII accomplishes deinitialization)
	#[export_name = "NBString_deinit"]
	pub extern fn deinit(&mut self) {
		if let Some(buf) = &mut self.buf {
			self.data = std::ptr::null_mut();
			self.len = 0;
			mem::drop(mem::take(buf));
			self.buf = None;
		}
	}
}
