//! Three-pillar BLAKE3 attestation (`theory/THEORY.md` §V.3, Pillar 10):
//!
//!   `artifact_hash ⊕ control_hash ⊕ intent_hash  →  BLAKE3 root`
//!
//! - **artifact** = the rendered bytes (the thing being attested).
//! - **control** = the canonical declaration that produced them.
//! - **intent** = the user-level intent context (authorship trail).
//!
//! v0 implements the PRIMITIVE (the deterministic three-leaf Merkle) using
//! the same `blake3` crate `tameshi` uses internally. The upgrade path at
//! workspace absorption is `tameshi::certification_artifact::Certification
//! Artifact` + Ed25519 signing + the persistent `HeartbeatChain` for the
//! Phase 1 → Phase 2 attestation lifecycle (`THEORY.md` §V.4). The hashes
//! produced here remain valid leaves in that chain.

/// A three-pillar attestation: three BLAKE3 leaves + a BLAKE3 Merkle root.
/// All values are byte-deterministic functions of their inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderAttestation {
    pub artifact_hash: [u8; 32],
    pub control_hash: [u8; 32],
    pub intent_hash: [u8; 32],
    pub root: [u8; 32],
}

impl RenderAttestation {
    /// Compute the three leaves + the Merkle root.
    #[must_use]
    pub fn new(artifact: &[u8], control: &[u8], intent: &[u8]) -> Self {
        let artifact_hash = *blake3::hash(artifact).as_bytes();
        let control_hash = *blake3::hash(control).as_bytes();
        let intent_hash = *blake3::hash(intent).as_bytes();
        // Three-leaf canonical concatenation, then BLAKE3 once more for the
        // root. Same shape tameshi's three-pillar Merkle uses.
        let mut combined = [0u8; 96];
        combined[..32].copy_from_slice(&artifact_hash);
        combined[32..64].copy_from_slice(&control_hash);
        combined[64..96].copy_from_slice(&intent_hash);
        let root = *blake3::hash(&combined).as_bytes();
        Self { artifact_hash, control_hash, intent_hash, root }
    }

    /// Lowercase hex of the Merkle root — the form embedded in
    /// `.typescape.yaml` provenance manifests.
    #[must_use]
    pub fn root_hex(&self) -> String {
        hex32(&self.root)
    }
}

fn hex32(b: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(64);
    for byte in b {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_pillar_is_deterministic() {
        let a = RenderAttestation::new(b"artifact-bytes", b"control-bytes", b"intent-bytes");
        let b = RenderAttestation::new(b"artifact-bytes", b"control-bytes", b"intent-bytes");
        assert_eq!(a, b);
        assert_eq!(a.root_hex().len(), 64);
    }

    #[test]
    fn root_changes_when_any_pillar_changes() {
        let base = RenderAttestation::new(b"A", b"B", b"C").root;
        assert_ne!(base, RenderAttestation::new(b"A'", b"B", b"C").root);
        assert_ne!(base, RenderAttestation::new(b"A", b"B'", b"C").root);
        assert_ne!(base, RenderAttestation::new(b"A", b"B", b"C'").root);
    }

    #[test]
    fn leaves_are_blake3_of_inputs() {
        let a = RenderAttestation::new(b"hello", b"x", b"y");
        assert_eq!(a.artifact_hash, *blake3::hash(b"hello").as_bytes());
        assert_eq!(a.control_hash, *blake3::hash(b"x").as_bytes());
        assert_eq!(a.intent_hash, *blake3::hash(b"y").as_bytes());
    }
}
