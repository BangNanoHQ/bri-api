# Product Requirements Document (PRD)
## Rust-based API Client for BRIAPI BI-Fast V2

## **1. Introduction**
This document outlines the requirements for building a **Rust-based API client** for **Bank Rakyat Indonesia (BRI) BI-Fast V2**, focusing on secure authentication, fund transfers, transaction inquiries, and status checks. The client must include robust **unit tests** to ensure the correctness and reliability of API integrations.

---

## **2. Objectives**
- Implement a **secure** Rust-based API client for **BRIAPI BI-Fast V2**.
- Use **OAuth 2.0 with Digital Signature authentication** for secure access.
- Provide methods for:
  - **Account Inquiry** (`/v2.0/transfer-bifast/inquiry-bifast`)
  - **Fund Transfer** (`/v2.0/transfer-bifast/payment-bifast`)
  - **Transaction Status Check** (`/v2.0/transfer-bifast/check-status-bifast`)
- Implement **unit tests** for each API function to validate responses and error handling.

---

## **3. API Overview**
### **3.1 Authentication (OAuth 2.0 with Digital Signature)**

#### **Step 1: Obtain Access Token**
- **Method:** `POST`
- **Endpoint:** `/snap/v1.0/access-token/b2b`
- **Headers:**
  - `X-SIGNATURE`: Generated using the client's private key.
  - `X-CLIENT-KEY`: The client's unique identifier.
  - `X-TIMESTAMP`: The current timestamp in `yyyy-MM-ddTHH:mm:ss.SSSTZD` format (ISO8601).
  - `Content-Type`: `application/json`
- **Request Body:**
  ```json
  {
    "grantType": "client_credentials"
  }
  ```
- **String to Sign:**
  ```
  client_id + "|" + X-TIMESTAMP
  ```
- **Signature Generation:**
  - Use the RSA 2048-bit private key to sign the string using the `SHA256withRSA` algorithm.
- **Response:**
  ```json
  {
    "accessToken": "<TOKEN>",
    "tokenType": "BearerToken",
    "expiresIn": "899"
  }
  ```

#### **Step 2: Include in API Requests**
- **Headers:**
  - `Authorization`: `Bearer {access_token}`
  - `X-SIGNATURE`: The generated signature.
  - `X-TIMESTAMP`: The timestamp used in the signature.

---

## **4. Unit Testing Strategy**
### **4.1 Testing Framework**
- Use **Rust’s built-in test framework** (`#[test]` macro).
- Use **`mockito`** for simulating API responses.
- Use **`serde_json`** for JSON parsing and validation.

### **4.2 Unit Test Cases**
#### **4.2.1 Authentication Tests**
- **Test case 1:** Valid credentials return a valid access token.
- **Test case 2:** Invalid credentials return an authentication error.
- **Test case 3:** Expired token is rejected, requiring re-authentication.
- **Test case 4:** Signature generation produces the correct RSA-SHA256 output.
- **Test case 5:** Requests with missing/invalid signatures fail authentication.

#### **4.2.2 API Request Validation**
- **Test case 1:** `Authorization`, `X-SIGNATURE`, and `X-TIMESTAMP` headers are correctly included.
- **Test case 2:** Invalid/missing headers result in proper rejection.

#### **4.2.3 Account Inquiry Tests**
- **Test case 1:** Valid account details return expected information.
- **Test case 2:** Invalid account number returns an error.
- **Test case 3:** Empty request body results in validation error.

#### **4.2.4 Fund Transfer Tests**
- **Test case 1:** Valid transfer request completes successfully.
- **Test case 2:** Insufficient funds return a `4038014` error.
- **Test case 3:** Invalid bank code returns a `4008101` error.
- **Test case 4:** Exceeding transaction limit returns a `4038002` error.

#### **4.2.5 Transaction Status Check Tests**
- **Test case 1:** Valid reference number returns the correct status.
- **Test case 2:** Expired transaction reference returns `4048201`.
- **Test case 3:** Incorrect reference format returns `4008201`.

### **4.3 Mock Response Handling**
For each test, simulate API responses using `mockito`:
```rust
#[test]
fn test_authentication_success() {
    let mut server = mockito::Server::new();
    let mock = server.mock("POST", "/snap/v1.0/access-token/b2b")
        .with_status(200)
        .with_body(r#"{"accessToken": "valid_token", "expiresIn": "899"}"#)
        .create();
    
    let result = authenticate(); // Call authentication function
    assert_eq!(result, Ok("valid_token".to_string()));
    mock.assert();
}
```

---

## **5. Security Considerations**
- **Store API credentials securely** using environment variables.
- **Use Rust’s `rsa` and `sha2` crates** for signing API requests.
- **Implement retry logic** for transient API errors.

---

## **6. Conclusion**
This PRD defines the roadmap for building a **Rust-based API client** for **BRIAPI BI-Fast V2** with a robust testing strategy to ensure reliability and security.

