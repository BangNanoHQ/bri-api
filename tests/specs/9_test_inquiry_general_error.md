# Test Case: General Error

## Description
Testing behavior when general error occurs

## Test Type
Negative

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input beneficiaryAccountNo: 33333

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "33333"
}
```

## Expected Response
### Response Code
5008100

### Response Body
```json
{
    "responseCode": "5008100",
    "responseMessage": "General Error"
}
```
