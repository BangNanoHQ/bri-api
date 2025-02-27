# Test Case: Invalid Account

## Description
Testing behavior with invalid account number

## Test Type
Negative

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input beneficiaryAccountNo: 11111

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "11111"
}
```

## Expected Response
### Response Code
4048111

### Response Body
```json
{
    "responseCode": "4048111",
    "responseMessage": "Invalid Card/Account/Customer [info]/Virtual Account"
}
```
