use bri_api::{
    auth::AuthClient,
    bifast::v2::client::{BifastV2Client, InquiryRequest},
};

#[tokio::test]
async fn test_inquiry_internal_error() -> Result<(), Box<dyn std::error::Error>> {
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
        beneficiary_bank_code: "CENAIDJA".to_string(),
        beneficiary_account_no: "22222".to_string(),
    };

    // print the request body
    println!("------------REQUEST------------");
    println!("{}", serde_json::to_string_pretty(&request)?);

    // Send the inquiry request and expect it to fail
    let result = bifast_client.inquiry(request).await;
    
    // Verify we got an error
    assert!(result.is_err());

    // print the response body
    println!("------------RESPONSE-----------");
    if let Err(err) = &result {
        println!("{}", err);
    }
    
    // Convert the error to a string and verify it contains expected message
    let err_string = result.unwrap_err().to_string();
    assert!(err_string.contains("5008101"));
    assert!(err_string.contains("Internal Server Error"));

    Ok(())
}
