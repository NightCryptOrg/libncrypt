// Asymmetric encryption algorithms
#[allow(non_camel_case_types)]
pub enum AsymmetricAlgo {
	RSA_3072,
	RSA_4096,
	X25519
}

// Symmetric encryption algorithms
#[allow(non_camel_case_types)]
pub enum SymmetricAlgo {
	AES_GCM_256,
	CHACHA20_POLY1305
}
