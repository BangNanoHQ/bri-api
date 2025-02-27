# Test Case: Internal Server Error

## Description
Testing behavior when internal server error occurs

## Test Type
Negative

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input beneficiaryAccountNo: 22222

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "22222"
}
```

## Expected Response
### Response Code
5008101

### Response Body
```json
{
    "responseCode": "5008101",
    "responseMessage": "Internal Server Error"
}
```
