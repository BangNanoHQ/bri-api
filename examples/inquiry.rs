use bri_api::{
    auth::AuthClient,
    bifast::v2::client::{BifastV2Client, InquiryRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.dev").ok();

    let base_url = std::env::var("BRI_API_URL")
        .unwrap_or_else(|_| "https://sandbox.partner.api.bri.co.id".to_string());
    let client_id = std::env::var("BRI_CLIENT_ID").expect("BRI_CLIENT_ID not set");
    let private_key = std::env::var("BRI_PRIVATE_KEY").expect("BRI_PRIVATE_KEY not set");
    let client_secret = std::env::var("BRI_CLIENT_SECRET").expect("BRI_CLIENT_SECRET not set");

    println!("Creating auth client...");
    let auth_client = AuthClient::new(base_url.clone(), client_id, &private_key)?;

    println!("Creating bifast client...");
    let bifast_client = BifastV2Client::new(
        base_url,
        auth_client,
        client_secret,
        "sandbox_partner_id".to_string(), // You might want to move these to env vars
        "12345".to_string(),
        "sandbox_external_id".to_string(),
    );

    // Create inquiry request
    let request = InquiryRequest {
        beneficiary_bank_code: "CENAIDJA".to_string(),
        beneficiary_account_no: "99999".to_string(),
    };

    println!("Sending inquiry request...");
    match bifast_client.inquiry(request).await {
        Ok(response) => {
            println!("\nInquiry successful!");
            println!("Response code: {}", response.response_code);
            println!("Response message: {}", response.response_message);
            println!("Reference No: {}", response.reference_no);
            println!("External ID: {}", response.external_id);
            println!("Registration ID: {}", response.registration_id);
            println!("Receiver Name: {}", response.receiver_name);
            println!("Bank code: {}", response.beneficiary_bank_code);
            println!("Account number: {}", response.beneficiary_account_no);
            println!("Account type: {}", response.beneficiary_account_type);
            println!("Account number: {}", response.account_number);
            println!("Receiver type: {}", response.receiver_type);
            println!("Resident status: {}", response.receiver_resident_status);
            println!("Identity number: {}", response.receiver_identity_number);
            println!("Town name: {}", response.receiver_town_name);
            println!("Currency: {}", response.currency);
        }
        Err(e) => {
            eprintln!("Inquiry failed:");
            eprintln!("Error details: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}
