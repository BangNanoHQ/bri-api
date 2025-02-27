# Test Case: Check Status General Error

## Description
Testing behavior when general error occurs

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input originalPartnerReferenceNo: 33333

## Request Body
```json
{
    "originalPartnerReferenceNo": "33333",
    "serviceCode": "80",
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
5008200

### Response Body
```json
{
    "responseCode": "5008200",
    "responseMessage": "General Error"
}
```
