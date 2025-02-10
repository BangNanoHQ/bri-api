use bri_api::auth::AuthClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.dev").ok();

    // Load and validate environment variables
    let base_url = std::env::var("BRI_API_URL")
        .unwrap_or_else(|_| "https://sandbox.partner.api.bri.co.id".to_string());
    let client_id = std::env::var("BRI_CLIENT_ID").expect("BRI_CLIENT_ID not set");
    let client_secret = std::env::var("BRI_CLIENT_SECRET").expect("BRI_CLIENT_SECRET not set");
    let private_key = std::env::var("BRI_PRIVATE_KEY").expect("BRI_PRIVATE_KEY not set");

    println!("Using base URL: {}", base_url);
    println!("Client ID: {}", client_id);
    println!("Private key length: {}", private_key.len());

    // Create the client
    let client = AuthClient::new(base_url, client_id, &private_key)?;

    // Get access token with detailed error handling
    match client.get_access_token().await {
        Ok(token) => {
            println!("Successfully obtained access token:");
            println!("Access Token: {}", token.access_token);
            println!("Token Type: {}", token.token_type);
            println!("Expires In: {}", token.expires_in);
        }
        Err(e) => {
            eprintln!("Authentication failed:");
            eprintln!("Error details: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
