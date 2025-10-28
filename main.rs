use attester_flow::{
    attestation_data::*,
    attester::attester,
    verifier::verifier,
};

fn main() {
    // --- Setup: Define the trusted environment hash ---
    // This value is pre-calculated from the known-good VM image and should be
    // securely stored in the Verifier's policy database.
    let trusted_image_hash = "733dd8952b1b7027b4b12185a53907c03af5183424040954b071e67e335b3760";

    // 1. The remote Verifier initiates the request.
    println!("\n### Verifier Initiates Attestation ###");
    let challenge = AttestationChallenge {
        nonce: "unique-session-nonce-12345".to_string(), // Crucial for freshness
    };

    // 2. The Guest VM Attester generates the evidence.
    let attestation_report = attester::generate_evidence(&challenge);

    // 3. The Verifier receives the report and performs validation.
    println!("\n### Verifier Validates Report ###");
    let result = verifier::verify_report(&challenge, &attestation_report);

    // 4. The Verifier makes a trust decision.
    match result {
        VerificationResult::Trustworthy(msg) => {
            println!("\n✅ TRUST ESTABLISHED: {}", msg);
            // Securely provision secrets (e.g., decrypt application keys).
        }
        VerificationResult::Untrustworthy(msg) => {
            println!("\n❌ TRUST FAILED: {}", msg);
            // Abort the connection and refuse to provision secrets.
        }
    }

    // --- Simulating a Failure (Tampered VM) ---
    println!("\n--- Simulating a Tampered VM State ---");
    let mut tampered_report = attester::generate_evidence(&challenge);

    // Simulate a hypervisor or attacker changing the boot measurement.
    tampered_report.measurement = *b"TAMPERED_VM_BOOT_STATE_HASH_123456";

    let tampered_result = verifier::verify_report(&challenge, &tampered_report);

    match tampered_result {
        VerificationResult::Trustworthy(msg) => println!("\n✅ TRUST ESTABLISHED (Should not happen!): {}", msg),
        VerificationResult::Untrustworthy(msg) => {
            println!("\n❌ TRUST FAILED (Expected outcome): {}", msg);
        }
    }
}
