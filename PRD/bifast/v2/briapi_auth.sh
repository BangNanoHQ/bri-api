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

# Function to generate signature
generate_signature() {
  local http_method="$1"
  local api_path="$2"
  local access_token="$3"
  local body_hash="$4"
  local timestamp="$5"
  local client_secret="$6"

  local string_to_sign="${http_method}:${api_path}:${access_token}:${body_hash}:${timestamp}"
  echo -n "$string_to_sign" | openssl dgst -sha512 -hmac "$client_secret" | awk '{print $2}'
}

# Obtain Access Token
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S").$(printf "%03d" $(( $(date +%N) / 1000000 )))Z
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

# Account Inquiry
echo "Performing Account Inquiry..."
API_PATH="/v2.0/transfer-bifast/inquiry-bifast"
REQUEST_BODY='{"beneficiaryBankCode":"CENAIDJA","beneficiaryAccountNo":"99999"}'
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S").$(printf "%03d" $(( $(date +%N) / 1000000 )))Z
BODY_HASH=$(echo -n "$REQUEST_BODY" | openssl dgst -sha256 | awk '{print $2}' | tr '[:upper:]' '[:lower:]')
SIGNATURE=$(generate_signature "POST" "$API_PATH" "$ACCESS_TOKEN" "$BODY_HASH" "$TIMESTAMP" "$CLIENT_SECRET")

INQUIRY_RESPONSE=$(curl -s -X POST "https://sandbox.partner.api.bri.co.id$API_PATH" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -H "X-TIMESTAMP: $TIMESTAMP" \
  -H "X-SIGNATURE: $SIGNATURE" \
  -H "X-PARTNER-ID: $PARTNER_ID" \
  -H "CHANNEL-ID: $CHANNEL_ID" \
  -H "X-EXTERNAL-ID: $EXTERNAL_ID" \
  -d "$REQUEST_BODY")

echo "Inquiry Response: $INQUIRY_RESPONSE"

# Fund Transfer
echo "Performing Fund Transfer..."
API_PATH="/v2.0/transfer-bifast/payment-bifast"

# Generate timestamp for the transfer request
TRANSFER_TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S").$(printf "%03d" $(( $(date +%N) / 1000000 )))Z

TRANSFER_REQUEST_BODY='{
    "customerReference": "54321",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "001901000378301",
    "amount": {
        "value": "120000.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "12345678900",
    "referenceNo": "20220127BRINIDJA61050000018",
    "externalId": "53394951711",
    "transactionDate": "2022-02-22T13:07:24Z",
    "paymentInfo": "testing bifast",
    "senderType": "01",
    "senderResidentStatus": "01",
    "senderTownName": "0300",
    "additionalInfo": {
        "deviceId": "12345679237",
        "channel": "mobilephone"
    }
}'
echo "Transfer Request Body: $TRANSFER_REQUEST_BODY"

TRANSFER_REQUEST_BODY_CLEANED=$(echo -n "$TRANSFER_REQUEST_BODY" | tr -d '\n' | jq -c .)

BODY_HASH=$(echo -n "$TRANSFER_REQUEST_BODY_CLEANED" | openssl dgst -sha256 | awk '{print $2}' | tr '[:upper:]' '[:lower:]')
SIGNATURE=$(generate_signature "POST" "$API_PATH" "$ACCESS_TOKEN" "$BODY_HASH" "$TRANSFER_TIMESTAMP" "$CLIENT_SECRET")

TRANSFER_RESPONSE=$(curl -s -X POST "https://sandbox.partner.api.bri.co.id$API_PATH" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -H "X-TIMESTAMP: $TRANSFER_TIMESTAMP" \
  -H "X-SIGNATURE: $SIGNATURE" \
  -H "X-PARTNER-ID: $PARTNER_ID" \
  -H "CHANNEL-ID: $CHANNEL_ID" \
  -H "X-EXTERNAL-ID: $EXTERNAL_ID" \
  -d "$TRANSFER_REQUEST_BODY")

echo "Transfer Response: $TRANSFER_RESPONSE"