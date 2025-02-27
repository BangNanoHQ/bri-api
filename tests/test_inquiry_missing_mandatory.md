# Test Case: Missing Mandatory Field

## Description
Testing behavior when mandatory field is missing

## Test Type
Negative

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Leave mandatory field empty

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": ""
}
```

## Expected Response
### Response Code
4008102

### Response Body
```json
{
    "responseCode": "4008102",
    "responseMessage": "Invalid Mandatory Field beneficiaryAccountNo"
}
```
