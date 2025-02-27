use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::to_string;

use crate::auth::AuthClient;

type HmacSha512 = Hmac<Sha512>;

pub struct BifastV2Client {
    client: Client,
    base_url: String,
    auth_client: AuthClient,
    client_secret: String,
    partner_id: String,
    channel_id: String,
    external_id: String,
}

impl BifastV2Client {
    pub fn new(
        base_url: String,
        auth_client: AuthClient,
        client_secret: String,
        partner_id: String,
        channel_id: String,
        external_id: String,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url,
            auth_client,
            client_secret,
            partner_id,
            channel_id,
            external_id,
        }
    }

    pub async fn inquiry(&self, request: InquiryRequest) -> Result<InquiryResponse, Box<dyn std::error::Error>> {
        let access_token = self.auth_client.get_access_token().await?.access_token;
        
        let api_path = "/v2.0/transfer-bifast/inquiry-bifast";
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string() + &format!(".{:03}Z", chrono::Utc::now().timestamp_subsec_millis());

        // Create request body and calculate body hash
        let request_body = to_string(&request)?;
        let body_hash = {
            let mut hasher = Sha256::new();
            hasher.update(request_body.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        // Generate signature
        let string_to_sign = format!(
            "POST:{}:{}:{}:{}",
            api_path, access_token, body_hash, timestamp
        );
        
        let signature = {
            let mut mac = HmacSha512::new_from_slice(self.client_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(string_to_sign.as_bytes());
            format!("{:x}", mac.finalize().into_bytes())
        };

        // Make the API request
        let response = self.client
            .post(format!("{}{}", self.base_url, api_path))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("X-TIMESTAMP", &timestamp)
            .header("X-SIGNATURE", signature)
            .header("X-PARTNER-ID", &self.partner_id)
            .header("CHANNEL-ID", &self.channel_id)
            .header("X-EXTERNAL-ID", &self.external_id)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        Ok(response.json::<InquiryResponse>().await?)
    }

    pub async fn transfer(&self, mut request: TransferRequest) -> Result<TransferResponse, Box<dyn std::error::Error>> {
        let access_token = self.auth_client.get_access_token().await?.access_token;

        let api_path = "/v2.0/transfer-bifast/payment-bifast";
        let timestamp = chrono::Utc::now()
            .with_timezone(&chrono::FixedOffset::east(7 * 3600))
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string() + &format!(".{:03}+07:00", chrono::Utc::now().timestamp_subsec_millis());

        // request.transaction_date = timestamp.clone();

        // Create request body and calculate body hash
        let request_body = to_string(&request)?;
        
        let body_hash = {
            let mut hasher = Sha256::new();
            hasher.update(request_body.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        // Generate signature
        let string_to_sign = format!(
            "POST:{}:{}:{}:{}",
            api_path, access_token, body_hash, timestamp
        );

        let signature = {
            let mut mac = HmacSha512::new_from_slice(self.client_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(string_to_sign.as_bytes());
            format!("{:x}", mac.finalize().into_bytes())
        };

        // Make the API request
        let response = self.client
            .post(format!("{}{}", self.base_url, api_path))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("X-TIMESTAMP", &timestamp)
            .header("X-SIGNATURE", signature)
            .header("X-PARTNER-ID", &self.partner_id)
            .header("CHANNEL-ID", &self.channel_id)
            .header("X-EXTERNAL-ID", &self.external_id)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        Ok(response.json::<TransferResponse>().await?)
    }

    pub async fn check_status(&self, request: CheckStatusRequest) -> Result<CheckStatusResponse, Box<dyn std::error::Error>> {
        let access_token = self.auth_client.get_access_token().await?.access_token;

        let api_path = "/v2.0/transfer-bifast/check-status-bifast";
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string() + &format!(".{:03}Z", chrono::Utc::now().timestamp_subsec_millis());

        // Create request body and calculate body hash
        let request_body = to_string(&request)?;
        println!("Check Status Request body: {}", request_body);
        let body_hash = {
            let mut hasher = Sha256::new();
            hasher.update(request_body.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        // Generate signature
        let string_to_sign = format!(
            "POST:{}:{}:{}:{}",
            api_path, access_token, body_hash, timestamp
        );

        let signature = {
            let mut mac = HmacSha512::new_from_slice(self.client_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(string_to_sign.as_bytes());
            format!("{:x}", mac.finalize().into_bytes())
        };

        // Make the API request
        let response = self.client
            .post(format!("{}{}", self.base_url, api_path))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("X-TIMESTAMP", &timestamp)
            .header("X-SIGNATURE", signature)
            .header("X-PARTNER-ID", &self.partner_id)
            .header("CHANNEL-ID", &self.channel_id)
            .header("X-EXTERNAL-ID", &self.external_id)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }

        Ok(response.json::<CheckStatusResponse>().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthClient;

    #[tokio::test]
    async fn test_inquiry_success() {
        let mut server = mockito::Server::new();
        
        // Mock auth token request
        let auth_mock = server.mock("POST", "/snap/v1.0/access-token/b2b")
            .with_status(200)
            .with_body(r#"{"access_token":"test_token","token_type":"Bearer","expires_in":"899"}"#)
            .create();

        // Mock inquiry request
        let inquiry_mock = server.mock("POST", "/v2.0/transfer-bifast/inquiry-bifast")
            .match_header("Authorization", "Bearer test_token")
            .with_status(200)
            .with_body(r#"{
                "responseCode": "2008100",
                "responseMessage": "Successful",
                "referenceNo": "20220127BRINIDJA61050000018",
                "externalId": "53296848727",
                "registrationId": "0001230075",
                "receiverName": "KR DUMMY ACCOUNT TBXXXXXXXXXXXXXXXXXXXXX",
                "beneficiaryAccountNo": "99999",
                "beneficiaryBankCode": "CENAIDJA",
                "beneficiaryAccountType": "SVGS",
                "accountNumber": "020601000988301",
                "receiverType": "02",
                "receiverResidentStatus": "01",
                "receiverIdentityNumber": "1216728398362819",
                "receiverTownName": "0300",
                "currency": "IDR"
            }"#)
            .create();

        let auth_client = AuthClient::new(
            server.url(),
            "test_client".to_string(),
            include_str!("../../../tests/test_private_key.pem"),
        ).unwrap();

        let client = BifastV2Client::new(
            server.url(),
            auth_client,
            "test_secret".to_string(),
            "test_partner".to_string(),
            "12345".to_string(),
            "test_external".to_string(),
        );

        let request = InquiryRequest {
            beneficiary_bank_code: "CENAIDJA".to_string(),
            beneficiary_account_no: "99999".to_string(),
        };

        let result = client.inquiry(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.response_code, "2008100");
        assert_eq!(response.receiver_name, "KR DUMMY ACCOUNT TBXXXXXXXXXXXXXXXXXXXXX");

        auth_mock.assert();
        inquiry_mock.assert();
    }

    #[tokio::test]
    async fn test_transfer_success() {
        let mut server = mockito::Server::new();

        // Mock auth token request
        let auth_mock = server.mock("POST", "/snap/v1.0/access-token/b2b")
            .with_status(200)
            .with_body(r#"{"access_token":"test_token","token_type":"Bearer","expires_in":"899"}"#)
            .create();

        // Mock transfer request
        let transfer_mock = server.mock("POST", "/v2.0/transfer-bifast/payment-bifast")
            .match_header("Authorization", "Bearer test_token")
            .with_status(200)
            .with_body(r#"{
                "responseCode": "2008000",
                "responseMessage": "Successful",
                "customerReference": "54321",
                "sourceAccountNo": "001901000378301",
                "beneficiaryAccountNo": "12345678900",
                "beneficiaryBankCode": "CENAIDJA",
                "referenceNo": "20220128CENAIDJA1698678312816763100",
                "externalId": "038455145464",
                "journalSequence": "8292049",
                "originalReferenceNo": "20220128BRINIDJA794224862497256",
                "amount": {
                    "value": "120000.00",
                    "currency": "IDR"
                },
                "additionalInfo": {
                    "deviceId": "12345679237",
                    "channel": "mobilephone",
                    "isRdn": "True"
                }
            }"#)
            .create();

        let auth_client = AuthClient::new(
            server.url(),
            "test_client".to_string(),
            include_str!("../../../tests/test_private_key.pem"),
        ).unwrap();

        let client = BifastV2Client::new(
            server.url(),
            auth_client,
            "test_secret".to_string(),
            "test_partner".to_string(),
            "12345".to_string(),
            "test_external".to_string(),
        );

        let request = TransferRequest {
            customer_reference: "0000004".to_string(),
            sender_identity_number: "3515085211930002".to_string(),
            source_account_no: "001901000613303".to_string(),
            amount: Amount {
                value: "23000.00".to_string(),
                currency: "IDR".to_string(),
            },
            beneficiary_bank_code: "CENAIDJA".to_string(),
            beneficiary_account_no: "2571075557".to_string(),
            reference_no: "20220929BRINIDJA51050005403".to_string(),
            external_id: "54445790453".to_string(),
            transaction_date: "2022-11-30T22:25:24+07:00".to_string(),
            payment_info: Some("345345".to_string()),
            sender_type: "01".to_string(),
            sender_resident_status: "01".to_string(),
            sender_town_name: "0300".to_string(),
            additional_info: Some(AdditionalInfo {
                device_id: "12345679237".to_string(),
                channel: "mobilephone".to_string(),
                is_rdn: "true".to_string(),
            }),
        };

        let result = client.transfer(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.response_code, "2008000");
        assert_eq!(response.customer_reference, Some("54321".to_string()));

        auth_mock.assert();
        transfer_mock.assert();
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InquiryRequest {
    pub beneficiary_bank_code: String,
    pub beneficiary_account_no: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InquiryResponse {
    pub response_code: String,
    pub response_message: String,
    pub reference_no: String,
    pub external_id: String,
    pub registration_id: String,
    pub receiver_name: String,
    pub beneficiary_account_no: String,
    pub beneficiary_bank_code: String,
    pub beneficiary_account_type: String,
    pub account_number: String,
    pub receiver_type: String,
    pub receiver_resident_status: String,
    pub receiver_identity_number: String,
    pub receiver_town_name: String,
    pub currency: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub customer_reference: String,
    pub sender_identity_number: String,
    pub source_account_no: String,
    pub amount: Amount,
    pub beneficiary_bank_code: String,
    pub beneficiary_account_no: String,
    pub reference_no: String,
    pub external_id: String,
    pub transaction_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_info: Option<String>,
    pub sender_type: String,
    pub sender_resident_status: String,
    pub sender_town_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
    pub device_id: String,
    pub channel: String,
    #[serde(rename = "isRdn")]
    pub is_rdn: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponse {
    pub response_code: String,
    pub response_message: String,
    pub original_reference_no: Option<String>,
    pub journal_sequence: Option<String>,
    pub reference_no: Option<String>,
    pub external_id: Option<String>,
    pub customer_reference: Option<String>,
    pub beneficiary_account_no: Option<String>,
    pub beneficiary_bank_code: Option<String>,
    pub amount: Option<Amount>,
    pub source_account_no: Option<String>,
    pub additional_info: Option<AdditionalInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusRequest {
    pub original_partner_reference_no: String,
    pub service_code: String,
    pub transaction_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusResponse {
    pub response_code: String,
    pub response_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_partner_reference_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_reference_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_account_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_transaction_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default)]
    pub additional_info: HashMap<String, String>,
}


