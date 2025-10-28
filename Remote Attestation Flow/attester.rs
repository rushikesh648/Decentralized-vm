// In a real CVM, this would interact with /dev/sev-guest or a vTPM.
pub mod attester {
    use super::*;

    /// Simulates calling the TEE hardware to generate a signed report.
    ///
    /// In a real implementation:
    /// 1. The `report_data` field (which includes the nonce hash) would be
    ///    passed to the hardware instruction (e.g., SNP_GET_REPORT).
    /// 2. The hardware returns the report, signed by the TEE key (e.g., VCEK).
    pub fn generate_evidence(challenge: &AttestationChallenge) -> AttestationReport {
        println!("\n[Attester] Received Challenge: {}", challenge.nonce);

        // 1. Calculate a deterministic measurement (hash of the running image)
        //    (In a real scenario, this would be retrieved from the TEE hardware).
        let mut hasher = Sha256::new();
        hasher.update(b"VM_BOOT_MEASUREMENT_HASH"); // Mocked hash
        let measurement: [u8; 32] = hasher.finalize().into();

        // 2. Prepare the REPORT_DATA (must contain a hash of the nonce for binding)
        let mut report_data: [u8; 64] = [0; 64];
        let mut nonce_hasher = Sha256::new();
        nonce_hasher.update(challenge.nonce.as_bytes());
        report_data[0..32].copy_from_slice(&nonce_hasher.finalize());
        // The rest of the report_data is often used for other claims (like a vTPM AK)

        // 3. Simulate hardware signing (Mocked signature and cert chain)
        let signature = b"MOCKED_HARDWARE_SIGNATURE".to_vec();
        let cert_chain = b"MOCKED_VCEK_CERTIFICATE_CHAIN".to_vec();

        println!("[Attester] Generated Report with Measurement: {:?}", &measurement[..8]);

        AttestationReport {
            measurement,
            report_data,
            signature,
            cert_chain,
        }
    }
}
