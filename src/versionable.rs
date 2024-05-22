/// Versionable - A struct is Versionable iff
/// its first field has the signature `version: u16`,
/// which must be updated whenever the struct definition is changed.
///
/// For structs this trait implements, the version field can ALWAYS be read as
/// *(u16 *)versionableStruct,
/// and must ALWAYS be set as CURRENT_VERSION
///
/// Versionable structs are intrinsically future-compatible
/// with the guarantee of never breaking logic
/// that depends on legacy layouts.
pub trait Versionable {
	const CURRENT_VERSION: u16;
}

pub fn version<T: Versionable>(s: &T) -> u16 {
	let ptr: *const T = s;
	let version: *const u16 = ptr as *const u16;

	unsafe { *version }
}
