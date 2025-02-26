use bri_api::auth::AuthClient;
use bri_api::bifast::v2::client::{BifastV2Client, CheckStatusRequest};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    // Create check status request
    let check_status_request = CheckStatusRequest {
        original_partner_reference_no: "54321".to_string(),
        service_code: "80".to_string(),
        transaction_date: "22-02-2022".to_string(),
    };

    println!("Checking transaction status...");
    let result = bifast_client.check_status(check_status_request).await;

    match result {
        Ok(response) => {
            println!("Check Status Response: {:?}", response);
            println!("\nCheck Status Successful!");
            println!("Response code: {}", response.response_code);
            println!("Response message: {}", response.response_message);
            
            if let Some(status) = response.latest_transaction_status {
                println!("Latest Transaction Status: {}", status);
                match status.as_str() {
                    "00" => println!("Status: Success - Booking successful/success to other bank"),
                    "03" => println!("Status: Pending - Need to recheck"),
                    "04" => println!("Status: Returned transaction"),
                    "06" => println!("Status: Failed - Booking failed/failed to other bank"),
                    _ => println!("Status: Unknown status code"),
                }
            }
            
            if let Some(desc) = response.transaction_status_desc {
                println!("Transaction Status Description: {}", desc);
            }
            
            if let Some(amount) = response.amount {
                println!("Amount: {} {}", amount.value, amount.currency);
            }
        }
        Err(err) => {
            eprintln!("Check Status failed:");
            eprintln!("Error details: {:?}", err);
        }
    }

    Ok(())
}
