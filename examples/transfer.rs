use bri_api::auth::AuthClient;
use bri_api::bifast::v2::client::{BifastV2Client, TransferRequest, Amount, AdditionalInfo};
use chrono::{DateTime, FixedOffset, Utc};
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

    let timezone_offset = FixedOffset::east_opt(7 * 3600).unwrap(); // Jakarta Timezone (+07:00)
    let now_utc: DateTime<Utc> = Utc::now();
    let now_jakarta: DateTime<FixedOffset> = now_utc.with_timezone(&timezone_offset);

    let transaction_date = now_jakarta.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    println!("Transaction Date: {}", transaction_date);

    // Example Transfer Request
    let transfer_request = TransferRequest {
        customer_reference: "54321".to_string(),
        sender_identity_number: "3515085211930002".to_string(),
        source_account_no: "001901000378301".to_string(),
        amount: Amount {
            value: "23000.00".to_string(),
            currency: "IDR".to_string(),
        },
        beneficiary_bank_code: "CENAIDJA".to_string(),
        beneficiary_account_no: "12345678900".to_string(),
        reference_no: "20220127BRINIDJA61050000018".to_string(),
        external_id: "53394951711".to_string(),
        // transaction_date: transaction_date, // Ensure the timezone is included and correctly formattede timezone is included and correctly formatted
        transaction_date: "2022-02-22T13:07:24Z".to_string(),
        payment_info: Some("testing bifast".to_string()),
        sender_type: "01".to_string(),
        sender_resident_status: "01".to_string(),
        sender_town_name: "0300".to_string(),
        additional_info: Some(AdditionalInfo {
            device_id: "12345679237".to_string(),
            channel: "mobilephone".to_string(),
            is_rdn: "false".to_string(),
        })
        // additional_info: None,
    };


    println!("Performing Transfer...");
    let result = bifast_client.transfer(transfer_request).await;

    match result {
        Ok(transfer_response) => {
            println!("Transfer Response: {:?}", transfer_response);
        }
        Err(err) => {
            eprintln!("Error during Transfer: {:?}", err);
        }
    }

    Ok(())
}
