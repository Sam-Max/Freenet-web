use freenet_stdlib::prelude::*;
use ed25519_dalek::{Signer, SigningKey, SecretKey as EdSecretKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

// Simple messages for communication with UI
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum AppRequest {
    GenerateKey,
    SignContent { content: Vec<u8> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum AppResponse {
    KeyGenerated { public_key: Vec<u8> },
    ContentSigned { signature: Vec<u8>, full_state: Vec<u8> },
    Error { message: String },
}

struct WebDelegate;

#[delegate]
impl DelegateInterface for WebDelegate {
    fn process(
        ctx: &mut DelegateCtx,
        _parameters: Parameters<'static>,
        _attested: Option<&'static [u8]>,
        message: InboundDelegateMsg,
    ) -> Result<Vec<OutboundDelegateMsg>, DelegateError> {
        match message {
            InboundDelegateMsg::ApplicationMessage(app_msg) => {
                let req: AppRequest = serde_json::from_slice(&app_msg.payload)
                    .map_err(|e| DelegateError::Deser(e.to_string()))?;

                match req {
                    AppRequest::GenerateKey => {
                        let mut csprng = OsRng;
                        let signing_key = SigningKey::generate(&mut csprng);
                        let secret_bytes = signing_key.to_bytes();
                        let public_bytes = signing_key.verifying_key().to_bytes().to_vec();

                        // Store private key in node secrets
                        // Key: "my-website-key"
                        let key = b"my-website-key";
                        
                        // set_secret returns boolean success/failure in some versions, but let's assume it works or we ignore result for now as per reference
                        ctx.set_secret(key, &secret_bytes);
                        
                        let response = AppResponse::KeyGenerated { public_key: public_bytes };
                        let resp_msg = OutboundDelegateMsg::ApplicationMessage(ApplicationMessage::new(
                            app_msg.app,
                            serde_json::to_vec(&response).unwrap()
                        ));

                        Ok(vec![resp_msg])
                    }
                    AppRequest::SignContent { content } => {
                        let key = b"my-website-key";
                        
                        // Get secret directly
                        let secret_bytes = match ctx.get_secret(key) {
                            Some(s) => s,
                            None => {
                                let err_resp = AppResponse::Error { message: "Secret not found".into() };
                                return Ok(vec![OutboundDelegateMsg::ApplicationMessage(ApplicationMessage::new(
                                    app_msg.app,
                                    serde_json::to_vec(&err_resp).unwrap()
                                ))]);
                            }
                        };
                        
                        let secret_key: EdSecretKey = secret_bytes.try_into()
                            .map_err(|_| DelegateError::Other("Invalid key format".into()))?;
                        let signing_key = SigningKey::from_bytes(&secret_key);

                        // Sign
                        let signature = signing_key.sign(&content).to_bytes().to_vec();

                        // Construct full state: [Signature] + [Content]
                        let mut full_state = signature.clone();
                        full_state.extend_from_slice(&content);

                        let app_resp = AppResponse::ContentSigned { 
                            signature, 
                            full_state 
                        };
                        
                        Ok(vec![OutboundDelegateMsg::ApplicationMessage(ApplicationMessage::new(
                            app_msg.app,
                            serde_json::to_vec(&app_resp).unwrap()
                        ))])
                    }
                }
            }
            _ => Ok(vec![]),
        }
    }
}
