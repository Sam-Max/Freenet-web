use freenet_stdlib::prelude::*;
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use ciborium::de::from_reader;

pub struct WebContract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebContainerMetadata {
    pub version: u32,
    pub signature: Signature,
}

const MAX_METADATA_SIZE: u64 = 1024; // 1KB
const MAX_WEB_SIZE: u64 = 1024 * 1024 * 100; // 100MB

#[contract]
impl ContractInterface for WebContract {
    fn validate_state(
        parameters: Parameters<'static>,
        state: State<'static>,
        _related: RelatedContracts<'static>,
    ) -> Result<ValidateResult, ContractError> {
        // 1. Get Public Key from Parameters
        if parameters.as_ref().len() != 32 {
            return Err(ContractError::Other("Invalid parameters length".into()));
        }
        let public_key = VerifyingKey::from_bytes(parameters.as_ref().try_into().unwrap())
            .map_err(|_| ContractError::Other("Invalid public key".into()))?;

        // 2. Parse State following River spec:
        // [metadata_length: u64][metadata: bytes][web_length: u64][web: bytes]
        let mut cursor = std::io::Cursor::new(state.as_ref());

        // Read metadata length
        let metadata_size = cursor.read_u64::<BigEndian>()
            .map_err(|_| ContractError::Other("Failed to read metadata size".into()))?;

        if metadata_size > MAX_METADATA_SIZE {
            return Err(ContractError::Other(format!("Metadata too large: {}", metadata_size)));
        }

        // Read metadata bytes
        let mut metadata_bytes = vec![0; metadata_size as usize];
        std::io::Read::read_exact(&mut cursor, &mut metadata_bytes)
            .map_err(|_| ContractError::Other("Failed to read metadata".into()))?;

        // Parse metadata as CBOR
        let metadata: WebContainerMetadata = from_reader(&metadata_bytes[..])
            .map_err(|e| ContractError::Deser(e.to_string()))?;

        if metadata.version == 0 {
            return Err(ContractError::Other("Invalid state version".into()));
        }

        // Read web content length
        let web_size = cursor.read_u64::<BigEndian>()
            .map_err(|_| ContractError::Other("Failed to read web size".into()))?;

        if web_size > MAX_WEB_SIZE {
            return Err(ContractError::Other(format!("Web content too large: {}", web_size)));
        }

        // Read web content
        let mut webapp_bytes = vec![0; web_size as usize];
        std::io::Read::read_exact(&mut cursor, &mut webapp_bytes)
            .map_err(|_| ContractError::Other("Failed to read web content".into()))?;

        // 3. Verify Signature
        // The message to verify is: [version (4 bytes BE)] + [webapp_bytes]
        // This matches River's signing tool construction
        let mut message = metadata.version.to_be_bytes().to_vec();
        message.extend_from_slice(&webapp_bytes);

        if public_key.verify(&message, &metadata.signature).is_ok() {
            Ok(ValidateResult::Valid)
        } else {
            Ok(ValidateResult::Invalid)
        }
    }

    fn update_state(
        _parameters: Parameters<'static>,
        _state: State<'static>,
        data: Vec<UpdateData<'static>>,
    ) -> Result<UpdateModification<'static>, ContractError> {
        // Simplified update: Just take the new state if valid
        // River does version checking here but we'll keep it simple for now
        if let Some(UpdateData::State(new_state)) = data.first() {
             Ok(UpdateModification::valid(new_state.clone()))
        } else {
            Err(ContractError::Other("Invalid update".into()))
        }
    }

    fn summarize_state(
        _parameters: Parameters<'static>,
        state: State<'static>,
    ) -> Result<StateSummary<'static>, ContractError> {
        // Return hash of the full state for simplicity
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&state);
        let hash = hasher.finalize();
        Ok(StateSummary::from(hash.to_vec()))
    }

    fn get_state_delta(
        _parameters: Parameters<'static>,
        state: State<'static>,
        _summary: StateSummary<'static>,
    ) -> Result<StateDelta<'static>, ContractError> {
        Ok(StateDelta::from(state.into_bytes()))
    }
}
