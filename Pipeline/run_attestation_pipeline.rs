use attester_flow::{
    attestation_data::{AttestationChallenge, AttestationReport},
    attester::attester,
    verifier::verifier,
};
use serde_json;
use std::error::Error;

// Define a simple custom error type for our pipeline
#[derive(Debug)]
enum PipelineError {
    KBSCommunicationError(String),
    VerificationFailed(String),
    SerializationError(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PipelineError::KBSCommunicationError(e) => write!(f, "KBS Communication Error: {}", e),
            PipelineError::VerificationFailed(e) => write!(f, "Verification Failed: {}", e),
            PipelineError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
        }
    }
}

impl Error for PipelineError {}


/// A simple struct to represent the Attestation Agent running inside the CVM.
struct AttestationAgent {
    kbs_endpoint: String,
}

impl AttestationAgent {
    /// Simulates the entire RCAR (Remote Challenge Attestation & Resource) pipeline.
    /// This is the core logic that an Attestation Agent would execute.
    pub fn run_attestation_pipeline(&self, resource_path: &str) -> Result<String, PipelineError> {
        println!("\n--- Attestation Pipeline Starting ---");
        println!("Key Broker Service Endpoint: {}", self.kbs_endpoint);

        // --- Step 1: Request Challenge from the KBS ---
        let challenge = self.request_challenge()?;
        
        // --- Step 2: Generate Attestation Evidence ---
        let report = attester::generate_evidence(&challenge);
        
        // --- Step 3: Submit Evidence to the KBS (Attestation Phase) ---
        let attestation_token = self.submit_evidence(&report)?;

        // NOTE: In a production scenario, the KBS would handle the verification
        // (Steps 1, 2, and 3 combined on the server side). 
        // We will run a client-side verification check here for demonstration, 
        // as if the Agent is checking a local policy before submitting.

        // --- DEMO Step: Client-Side Verification Check ---
        match verifier::verify_report(&challenge, &report) {
            attester_flow::attestation_data::VerificationResult::Trustworthy(_) => {
                println!("[Agent] Local integrity check passed.");
            },
            attester_flow::attestation_data::VerificationResult::Untrustworthy(e) => {
                return Err(PipelineError::VerificationFailed(format!(
                    "Local integrity check failed: {}", e
                )));
            },
        }
        // --- END DEMO Step ---

        // --- Step 4: Retrieve Resource (Secret) using the Token ---
        let secret = self.retrieve_resource(&attestation_token, resource_path)?;

        println!("--- Attestation Pipeline Complete ---");
        Ok(secret)
    }

    // --- Private Methods Simulating Network/API Calls ---

    /// Simulates an HTTP POST to the KBS /auth endpoint.
    fn request_challenge(&self) -> Result<AttestationChallenge, PipelineError> {
        println!("\n[KBS Agent] 1. Requesting Attestation Challenge...");
        
        // Simulate network call success
        let challenge_json = r#"{"nonce": "kbs-nonce-e5a9c1f2-7d3b-4e8c-9a0f-8b2d1c5e4a3"}"#;
        
        let challenge: AttestationChallenge = serde_json::from_str(challenge_json)
            .map_err(|e| PipelineError::SerializationError(e.to_string()))?;

        println!("[KBS Agent] Challenge received: {}", challenge.nonce);
        Ok(challenge)
    }

    /// Simulates an HTTP POST to the KBS /attest endpoint.
    fn submit_evidence(&self, report: &AttestationReport) -> Result<String, PipelineError> {
        println!("\n[KBS Agent] 2. Submitting Attestation Evidence...");

        // Serialize the report to send over the network
        let _evidence_payload = serde_json::to_string(report)
            .map_err(|e| PipelineError::SerializationError(e.to_string()))?;

        // Simulate network call success and receiving an attestation token
        let token = "kbs-auth-token-d4f7g8h1j2k3l4m5n6p7q8r9s0t1u2v3";
        println!("[KBS Agent] Evidence accepted. Received Attestation Token.");
        Ok(token.to_string())
    }

    /// Simulates an HTTP GET to the KBS /resource endpoint using the token.
    fn retrieve_resource(&self, token: &str, path: &str) -> Result<String, PipelineError> {
        println!("\n[KBS Agent] 3. Requesting Secret '{}' using Token...", path);

        // Simulate network call failure (if token is invalid, etc.)
        if token.is_empty() {
            return Err(PipelineError::KBSCommunicationError("No valid token provided.".to_string()));
        }

        // Simulate network call success: The KBS decrypts and returns the secret.
        let secret_payload = format!("Decrypted Secret for {}: API_KEY__{}", path, self.kbs_endpoint.len() * 100);
        
        println!("[KBS Agent] Successfully retrieved resource!");
        Ok(secret_payload)
    }
}


fn main() {
    // Instantiate the agent with the remote service endpoint.
    let agent = AttestationAgent {
        kbs_endpoint: "https://kbs.cloud.provider.com/api/v1".to_string(),
    };

    let secret_to_fetch = "/keys/database-cred";

    match agent.run_attestation_pipeline(secret_to_fetch) {
        Ok(secret) => {
            println!("\n==============================================");
            println!("✅ SUCCESS! Secret retrieved and application ready.");
            println!("Retrieved Secret: {}", secret);
            println!("==============================================");
        },
        Err(e) => {
            println!("\n==============================================");
            eprintln!("❌ FATAL ERROR in Attestation Pipeline: {}", e);
            println!("Secret access denied. Exiting.");
            println!("==============================================");
        }
    }
}

// NOTE: To run this, ensure the `attester_flow` module structure from the previous 
// response is correctly set up in your Rust project.
