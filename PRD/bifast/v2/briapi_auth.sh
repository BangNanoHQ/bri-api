#!/bin/bash

# THIS ONE WORKS for getting the acces token and the inquiry-bifast

# Load Environment Variables
PRIVATE_KEY=$PRIVATE_KEY_PATH
CLIENT_KEY=$CLIENT_KEY
CLIENT_SECRET=$CLIENT_SECRET


# Set Arbitrary Sandbox Values
PARTNER_ID="sandbox_partner_id"
CHANNEL_ID="12345"
EXTERNAL_ID="sandbox_external_id"

# Generate Correct Timestamp
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S").$(printf "%03d" $(( $(date +%N) / 1000000 )))Z

# Obtain Access Token
ACCESS_TOKEN=$(curl -s -X POST 'https://sandbox.partner.api.bri.co.id/snap/v1.0/access-token/b2b' \
  -H "Content-Type: application/json" \
  -H "X-CLIENT-KEY: $CLIENT_KEY" \
  -H "X-TIMESTAMP: $TIMESTAMP" \
  -H "X-SIGNATURE: $(echo -n "$CLIENT_KEY|$TIMESTAMP" | openssl dgst -sha256 -sign "$PRIVATE_KEY" | base64 | tr -d '\n')" \
  -d '{"grantType": "client_credentials"}' | jq -r '.accessToken')

# Validate Access Token
if [[ -z "$ACCESS_TOKEN" || "$ACCESS_TOKEN" == "null" ]]; then
  echo "Error: Failed to retrieve access token"
  exit 1
fi

# Define API Request Details
HTTP_METHOD="POST"
API_PATH="/v2.0/transfer-bifast/inquiry-bifast"
REQUEST_BODY='{"beneficiaryBankCode":"CENAIDJA","beneficiaryAccountNo":"99999"}'

# Generate Correct Timestamp
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S").$(printf "%03d" $(( $(date +%N) / 1000000 )))Z


# 🔹 **Step 1: Minify and Hash the Request Body**
BODY_HASH=$(echo -n "$REQUEST_BODY" | openssl dgst -sha256 | awk '{print $2}' | tr '[:upper:]' '[:lower:]')

# 🔹 **Step 2: Format String to Sign**
STRING_TO_SIGN="${HTTP_METHOD}:${API_PATH}:${ACCESS_TOKEN}:${BODY_HASH}:${TIMESTAMP}"

# 🔹 **Step 3: Generate HMAC-SHA512 Signature**
SIGNATURE=$(echo -n "$STRING_TO_SIGN" | openssl dgst -sha512 -hmac "$CLIENT_SECRET" | awk '{print $2}')

# 🔹 **Debugging: Print Corrected String to Sign**
echo "STRING_TO_SIGN: $STRING_TO_SIGN"
echo "BODY_HASH: $BODY_HASH"
echo "SIGNATURE: $SIGNATURE"

# 🔹 **Step 4: Perform Account Inquiry With Correct Signature**
curl -X POST "https://sandbox.partner.api.bri.co.id$API_PATH" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -H "X-TIMESTAMP: $TIMESTAMP" \
  -H "X-SIGNATURE: $SIGNATURE" \
  -H "X-PARTNER-ID: $PARTNER_ID" \
  -H "CHANNEL-ID: $CHANNEL_ID" \
  -H "X-EXTERNAL-ID: $EXTERNAL_ID" \
  -d "$REQUEST_BODY"