use serde::{Deserialize, Serialize};

/// Supported Proof of Work mechanisms for verification.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ProofType {
    /// Basic hash-based verification (Weakest security guarantees).
    SimpleHash,
    /// Trusted Execution Environment (TEE) Attestation (e.g., Intel SGX / TDX).
    TEEAttestation { enclave_signature: String },
    /// Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (zk-SNARK).
    ZKSnark { proof_data: Vec<u8> },
}

/// Proof of Physical Work (PoPW) Interface.
/// Implementations must enforce hardware integrity checks and result certification.
#[allow(dead_code)]
pub trait ProofOfPhysicalWork {
    /// Generates a cryptographic proof for a computed result.
    fn generate_proof(&self, task_id: &str, result_hash: &[u8]) -> anyhow::Result<ProofType>;
    
    /// Verifies the integrity of the execution environment (e.g., Secure Boot, TEE status).
    fn verify_environment_integrity(&self) -> bool;
}

// Reference implementation for testing purposes.
#[allow(dead_code)]
pub struct MockProver;

impl ProofOfPhysicalWork for MockProver {
    fn generate_proof(&self, task_id: &str, _result_hash: &[u8]) -> anyhow::Result<ProofType> {
        tracing::warn!("MockProver active: Generating simulated TEE signature.");
        Ok(ProofType::TEEAttestation {
            enclave_signature: format!("mock_tee_sig_for_task_{}", task_id),
        })
    }

    fn verify_environment_integrity(&self) -> bool {
        true
    }
}