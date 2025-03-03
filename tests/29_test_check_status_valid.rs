use bri_api::{
    auth::AuthClient,
    bifast::v2::client::{BifastV2Client, CheckStatusRequest},
};

#[tokio::test]
async fn test_check_status_valid() -> Result<(), Box<dyn std::error::Error>> {
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

    // Create check status request matching the test specification
    let request = CheckStatusRequest {
        original_partner_reference_no: "54321".to_string(),
        service_code: "80".to_string(),
        transaction_date: "22-02-2022".to_string(),
    };

    // Print the request body
    println!("------------REQUEST------------");
    println!("{}", serde_json::to_string_pretty(&request)?);

    // Send the check status request
    let response = bifast_client.check_status(request).await?;

    // Print the response body
    println!("------------RESPONSE-----------");
    println!("{}", serde_json::to_string_pretty(&response)?);

    // Verify the response matches expected values from the test spec
    assert_eq!(response.response_code, "2008200");
    assert_eq!(response.response_message, "Successful");
    assert_eq!(response.original_partner_reference_no, Some("54321".to_string()));
    assert_eq!(response.original_reference_no.is_some(), true);
    assert_eq!(response.service_code, Some("80".to_string()));
    assert_eq!(response.latest_transaction_status, Some("00".to_string()));
    assert_eq!(response.transaction_status_desc, Some("Payment success".to_string()));
    
    if let Some(amount) = response.amount {
        assert_eq!(amount.value, "120000.00");
        assert_eq!(amount.currency, "IDR");
    }

    Ok(())
}
