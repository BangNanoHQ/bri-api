use chrono::{DateTime, Utc};
use rsa::{Pkcs1v15Sign, pkcs8::DecodePrivateKey, sha2::Sha256, RsaPrivateKey};
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use sha2::{Digest};
use rsa::pss::{BlindedSigningKey, Signature}; // Add this import

use super::{AccessTokenRequest, AccessTokenResponse, AuthError};

pub struct AuthClient {
    client: Client,
    base_url: String,
    client_id: String,
    private_key: RsaPrivateKey,
}

impl AuthClient {
    pub fn new(base_url: String, client_id: String, private_key_base64: &str) -> Result<Self, AuthError> {
        // Decode base64 to get PEM string
        let private_key_pem = String::from_utf8(
            STANDARD.decode(private_key_base64)
                .map_err(|e| AuthError::SignatureError(e.to_string()))?
        ).map_err(|e| AuthError::SignatureError(e.to_string()))?;

        // Parse PEM formatted private key
        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)
            .map_err(|e| AuthError::SignatureError(e.to_string()))?;

        Ok(Self {
            client: Client::new(),
            base_url,
            client_id,
            private_key,
        })
    }

    pub async fn get_access_token(&self) -> Result<AccessTokenResponse, AuthError> {
        // Match exact timestamp format from bash script
        let timestamp = format!("{}.{}Z",
            Utc::now().format("%Y-%m-%dT%H:%M:%S"),
            format!("{:03}", (Utc::now().timestamp_subsec_nanos() / 1_000_000))
        );

        let signature = self.generate_signature(&timestamp)?;

        let response = self.client
            .post(format!("{}/snap/v1.0/access-token/b2b", self.base_url))
            .header("Content-Type", "application/json")
            .header("X-CLIENT-KEY", &self.client_id)
            .header("X-TIMESTAMP", &timestamp)
            .header("X-SIGNATURE", signature)
            .json(&AccessTokenRequest {
                grant_type: "client_credentials".to_string(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AuthError::InvalidCredentials(error_text));
        }

        let token_response = response.json::<AccessTokenResponse>().await?;
        Ok(token_response)
    }

    fn generate_signature(&self, timestamp: &str) -> Result<String, AuthError> {
        let data = format!("{}|{}", self.client_id, timestamp);
        
        // Hash the data first with SHA256
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let hashed_data = hasher.finalize();
        
        // Sign the hashed data
        let signature = self.private_key
            .sign(
                Pkcs1v15Sign::new::<Sha256>(),
                &hashed_data
            )
            .map_err(|e| AuthError::SignatureError(e.to_string()))?;

        Ok(STANDARD.encode(signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_successful_authentication() {
        let mut server = mockito::Server::new();
        
        let mock = server.mock("POST", "/snap/v1.0/access-token/b2b")
            .match_header("Content-Type", "application/json")
            .match_header("X-CLIENT-KEY", "test_client")
            .with_status(200)
            .with_body(r#"{"access_token":"test_token","token_type":"BearerToken","expires_in":"899"}"#)
            .create();

        let test_key = include_str!("../../tests/test_private_key.pem");
        let client = AuthClient::new(
            server.url(),
            "test_client".to_string(),
            test_key,
        ).unwrap();

        let result = client.get_access_token().await;
        assert!(result.is_ok());
        
        let token = result.unwrap();
        assert_eq!(token.token_type, "BearerToken");
        assert_eq!(token.expires_in, "899");
        
        mock.assert();
    }
}
