# Test Case: Internal Server Error in Check Status

## Description
Testing behavior when internal server error occurs

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input originalPartnerReferenceNo with value "22222" to trigger internal server error

## Request Body
```json
{
    "originalPartnerReferenceNo": "22222",
    "serviceCode": "80", 
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
5008201

### Response Body
```json
{
    "responseCode": "5008201",
    "responseMessage": "Internal Server Error"
}
```
