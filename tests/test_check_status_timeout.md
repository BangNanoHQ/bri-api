# Test Case: Check Status Timeout

## Description
Testing behavior when check status request times out

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input originalPartnerReferenceNo: 44444

## Request Body
```json
{
    "originalPartnerReferenceNo": "44444",
    "serviceCode": "80",
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
5048200

### Response Body
```json
{
    "responseCode": "5048200",
    "responseMessage": "Timeout"
}
```
