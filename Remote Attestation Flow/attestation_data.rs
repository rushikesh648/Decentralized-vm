use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

// --- Core Data Structures ---

/// A challenge issued by the Verifier to ensure the report is fresh.
#[derive(Debug, Serialize, Deserialize)]
pub struct AttestationChallenge {
    pub nonce: String,
}

/// The hardware-signed cryptographic evidence from the Confidential VM.
#[derive(Debug, Serialize, Deserialize)]
pub struct AttestationReport {
    /// A cryptographic hash of the entire VM's boot state.
    pub measurement: [u8; 32],
    /// The nonce copied from the challenge to ensure freshness.
    pub report_data: [u8; 64],
    /// The raw bytes of the hardware signature (e.g., VCEK-signed signature).
    pub signature: Vec<u8>,
    /// Public key or certificate chain needed to verify the signature.
    pub cert_chain: Vec<u8>,
}

/// The result of the verification process.
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationResult {
    Trustworthy(String),
    Untrustworthy(String),
}
