# Test Case: Missing Mandatory Field in Check Status

## Description
Testing behavior when mandatory field is missing

## Test Type
Negative

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Leave originalPartnerReferenceNo empty

## Request Body
```json
{
    "originalPartnerReferenceNo": "",
    "serviceCode": "80",
    "transactionDate": "16-09-2020"
}
```

## Expected Response
### Response Code
4008202

### Response Body
```json
{
    "responseCode": "4008202",
    "responseMessage": "Invalid Mandatory Field originalPartnerReferenceNo"
}
```
