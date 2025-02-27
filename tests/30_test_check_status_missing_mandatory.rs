use bri_api::{
    auth::AuthClient,
    bifast::v2::client::{BifastV2Client, CheckStatusRequest},
};

#[tokio::test]
async fn test_check_status_missing_mandatory() -> Result<(), Box<dyn std::error::Error>> {
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

    // Create check status request with missing mandatory field
    let request = CheckStatusRequest {
        original_partner_reference_no: "".to_string(), // Empty mandatory field
        service_code: "80".to_string(),
        transaction_date: "16-09-2020".to_string(),
    };

    // Print the request body
    println!("------------REQUEST------------");
    println!("{}", serde_json::to_string_pretty(&request)?);

    // Send the check status request and expect it to fail
    let result = bifast_client.check_status(request).await;

    println!("------------RESPONSE-----------");
    if let Err(err) = &result {
        println!("{}", err);
    }
    
    // Convert the error to a string and verify it contains expected message
    let err_string = result.unwrap_err().to_string();
    assert!(err_string.contains("4008202"));
    assert!(err_string.contains("Invalid Mandatory Field originalPartnerReferenceNo"));

    Ok(())
}
