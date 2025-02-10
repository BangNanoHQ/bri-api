use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Signature generation failed: {0}")]
    SignatureError(String),
    
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
}
