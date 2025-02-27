use bri_api::{
    auth::AuthClient,
    bifast::v2::client::{BifastV2Client, TransferRequest, Amount, AdditionalInfo},
};

#[tokio::test]
async fn test_transfer_valid_proxy_phone() -> Result<(), Box<dyn std::error::Error>> {
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

    // Create transfer request matching the test specification
    let request = TransferRequest {
        customer_reference: "77777".to_string(),
        sender_identity_number: "3515085211930002".to_string(),
        source_account_no: "001901000378301".to_string(),
        amount: Amount {
            value: "15000.00".to_string(),
            currency: "IDR".to_string(),
        },
        beneficiary_bank_code: "CENAIDJA".to_string(),
        beneficiary_account_no: "+628123456789".to_string(),
        reference_no: "20220127BRINIDJA61020000033".to_string(),
        external_id: "24681357902".to_string(),
        transaction_date: "2022-02-22T13:07:24Z".to_string(),
        payment_info: Some("testing bifast".to_string()),
        sender_type: "01".to_string(),
        sender_resident_status: "01".to_string(),
        sender_town_name: "0300".to_string(),
        additional_info: Some(AdditionalInfo {
            is_rdn: "true".to_string(),
            device_id: "12345679237".to_string(),
            channel: "mobilephone".to_string(),
        }),
    };

    // print the request body
    println!("------------REQUEST------------");
    println!("{}", serde_json::to_string_pretty(&request)?);

    // Send the transfer request
    let response = bifast_client.transfer(request).await?;

    // print the response body
    println!("------------RESPONSE-----------");
    println!("{}", serde_json::to_string_pretty(&response)?);

    // Verify the response matches expected values from the test spec
    assert_eq!(response.response_code, "2008000");
    assert_eq!(response.response_message, "Successful");
    assert_eq!(response.customer_reference, Some("77777".to_string()));
    assert_eq!(response.source_account_no, Some("001901000378301".to_string()));
    assert_eq!(response.beneficiary_account_no, Some("+628123456789".to_string()));
    assert_eq!(response.beneficiary_bank_code, Some("CENAIDJA".to_string()));
    assert!(!response.reference_no.is_none());
    assert!(!response.external_id.is_none());
    assert!(!response.journal_sequence.is_none());
    assert!(!response.original_reference_no.is_none());
    assert_eq!(response.amount.clone().unwrap().value, "15000.00");
    assert_eq!(response.amount.clone().unwrap().currency, "IDR");

    Ok(())
}
