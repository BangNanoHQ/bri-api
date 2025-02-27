# Test Case: Transaction Not Found

## Description
Testing behavior when transaction is not found

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input originalPartnerReferenceNo: 17010

## Request Body
```json
{
    "originalPartnerReferenceNo":"17010",
    "serviceCode": "80",
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
4048200

### Response Body
```json
{
    "responseCode": "4048200",
    "responseMessage": "Invalid Transaction Status"
}
```
