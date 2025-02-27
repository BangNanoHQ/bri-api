use bri_api::{
    auth::AuthClient,
    bifast::v2::client::{BifastV2Client, InquiryRequest},
};

#[tokio::test]
async fn test_inquiry_valid_proxy_email() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.dev").ok();

    let base_url = std::env::var("BRI_API_URL")
        .unwrap_or_else(|_| "https://sandbox.partner.api.bri.co.id".to_string());
    let client_id = std::env::var("BRI_CLIENT_ID").expect("BRI_CLIENT_ID not set");
    let private_key = std::env::var("BRI_PRIVATE_KEY").expect("BRI_PRIVATE_KEY not set");
    let client_secret = std::env::var("BRI_CLIENT_SECRET").expect("BRI_CLIENT_SECRET not set");

    // Initialize auth client
    let auth_client = AuthClient::new(base_url.clone(), client_id, &private_key)?;

    // Initialize bifast client
    let bifast_client = BifastV2Client::new(
        base_url,
        auth_client,
        client_secret,
        "sandbox_partner_id".to_string(),
        "12345".to_string(),
        "sandbox_external_id".to_string(),
    );

    // Create inquiry request matching the test specification
    let request = InquiryRequest {
        beneficiary_bank_code: "".to_string(), // Empty for proxy
        beneficiary_account_no: "@testing@gmail.com".to_string(),
    };

    // Send the inquiry request
    let response = bifast_client.inquiry(request).await?;

    // Verify the response matches expected values from the test spec
    assert_eq!(response.response_code, "2008100");
    assert_eq!(response.response_message, "Successful");
    assert!(!response.reference_no.is_empty());
    assert!(!response.external_id.is_empty());
    assert_eq!(response.registration_id, "0001230075");
    assert_eq!(
        response.receiver_name,
        "YAYASAN DHARMA BAKTI INDONESIA(Pemelik Rekening Kosong)"
    );
    assert_eq!(response.beneficiary_account_no, "@testing@gmail.com");
    assert_eq!(response.beneficiary_bank_code, "BRINIDJA");
    assert_eq!(response.beneficiary_account_type, "SVGS");
    assert_eq!(response.account_number, "020601000988301");
    assert_eq!(response.receiver_type, "01");
    assert_eq!(response.receiver_resident_status, "01");
    assert_eq!(response.receiver_identity_number, "1216728398362819");
    assert_eq!(response.receiver_town_name, "0300");
    assert_eq!(response.currency, "IDR");

    Ok(())
}
