# Test Case: Unauthorized Client in Check Status

## Description
Testing behavior when client is unauthorized

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input originalPartnerReferenceNo: 66666

## Request Body
```json
{
    "originalPartnerReferenceNo": "66666",
    "serviceCode": "80",
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
4018200

### Response Body
```json
{
    "responseCode": "4018200",
    "responseMessage": "Unauthorized. Client"
}
```
