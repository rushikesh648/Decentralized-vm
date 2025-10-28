// This is the remote service running outside the CVM.
pub mod verifier {
    use super::*;

    /// Defines the expected boot state hash for a trusted VM image.
    const EXPECTED_MEASUREMENT_HASH: &str = "733dd8952b1b7027b4b12185a53907c03af5183424040954b071e67e335b3760"; // Mocked hash

    /// The main function for verifying the attestation evidence.
    ///
    /// In a real implementation, this would involve:
    /// 1. Cryptographic validation of the signature using the cert chain (PKI).
    /// 2. Policy lookup based on platform ID and TCB.
    /// 3. Comparison of reported measurements against known trusted values.
    pub fn verify_report(
        challenge: &AttestationChallenge,
        report: &AttestationReport,
    ) -> VerificationResult {
        println!("\n[Verifier] Starting verification process...");

        // --- Step 1: Verify Freshness (Nonce Binding) ---
        let mut nonce_hasher = Sha256::new();
        nonce_hasher.update(challenge.nonce.as_bytes());
        let expected_report_data: [u8; 32] = nonce_hasher.finalize().into();

        if report.report_data[0..32] != expected_report_data {
            return VerificationResult::Untrustworthy(
                "Freshness check failed: Report data does not match challenge nonce hash."
                    .to_string(),
            );
        }
        println!("[Verifier] Nonce check successful. Report is fresh.");

        // --- Step 2: Verify Signature (Hardware Authenticity) ---
        // Mocked check: In a real flow, this is where a complex PKI check happens.
        if report.signature != b"MOCKED_HARDWARE_SIGNATURE" {
            return VerificationResult::Untrustworthy(
                "Signature check failed: Could not verify hardware authenticity."
                    .to_string(),
            );
        }
        println!("[Verifier] Signature check successful. Report is authentic.");

        // --- Step 3: Verify Integrity (Measurement Policy) ---
        let actual_hash_hex = hex::encode(report.measurement);

        // Compare the reported boot state measurement against the trusted policy.
        if actual_hash_hex == EXPECTED_MEASUREMENT_HASH {
            VerificationResult::Trustworthy(
                "Attestation successful! VM is running the expected image."
                    .to_string(),
            )
        } else {
            VerificationResult::Untrustworthy(format!(
                "Integrity check failed. Actual measurement: {}, Expected: {}",
                actual_hash_hex, EXPECTED_MEASUREMENT_HASH
            ))
        }
    }
}
