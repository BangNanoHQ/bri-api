# Test Case: Invalid Field Format in Check Status

## Description
Testing behavior when field format is invalid

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input value field with incorrect data type or exceeding allowed character length

## Request Body
```json
{
    "originalPartnerReferenceNo": "645432439999999999999",
    "serviceCode": "80",
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
4008201

### Response Body
```json
{
    "responseCode": "4008201",
    "responseMessage": "Invalid Field Format originalPartnerReferenceNo"
}
```
