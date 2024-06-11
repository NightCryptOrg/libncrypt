use crate::string::{NCString, NBString};
use crate::versionable::Versionable;
use std::collections::HashSet;

// Defined header versions
// Version field MUST match between EncryptionHeader and its child structs
pub const V0: u16 = 0;
pub const CURRENT_VERSION: u16 = V0;

#[repr(C)]
pub struct EncryptionHeader {
	pub version: u16,
	pub data_header: DataHeader,
	pub key_header: KeyHeader,
	/// Data-encryption key, encrypted using user-KEK
	key: Box<NBString>
}
impl Versionable for EncryptionHeader { const CURRENT_VERSION: u16 = CURRENT_VERSION; }

/// Metadata for encrypted data fields
/// (delivered encrypted with the same KEK as the data key)
#[repr(C)]
pub struct DataHeader {
	pub version: u16,
	/// Whether the data is empty/null.
	/// This field enables opaque nullability
	pub empty: bool,
	/// A set of fields in the data that are empty/null
	pub empty_fields: HashSet<Box<NCString>>,
	/// Data encryption algorithm ID
	pub algorithm: Box<NCString>
}
impl Versionable for DataHeader{ const CURRENT_VERSION: u16 = CURRENT_VERSION; }

/// Metadata for KEK-encrypted data-encryption keys
/// (delivered as plaintext)
#[repr(C)]
pub struct KeyHeader {
	pub version: u16,
	/// Key encryption algorithm ID
	pub algorithm: Box<NCString>
}
impl Versionable for KeyHeader{ const CURRENT_VERSION: u16 = CURRENT_VERSION; }
