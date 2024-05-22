use crate::string::{NCString, NBString};

#[repr(C)]
pub struct EncryptionHeader {
	pub data_header: DataHeader,
	pub key_header: KeyHeader,
	/// Data-encryption key, encrypted using user-KEK
	key: Box<NBString>
}

/// Metadata for encrypted data fields
/// (delivered encrypted with the same KEK as the data key)
#[repr(C)]
pub struct DataHeader {
	/// Whether the data is empty/null.
	/// This field enables opaque nullability
	pub empty: bool,
	/// Data encryption algorithm ID
	pub algorithm: Box<NCString>
}

/// Metadata for KEK-encrypted data-encryption keys
/// (delivered as plaintext)
#[repr(C)]
pub struct KeyHeader {
	/// Key encryption algorithm ID
	pub algorithm: Box<NCString>
}
