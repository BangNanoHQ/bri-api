# Test Case: Invalid Field Format

## Description
Testing behavior when field format is invalid

## Test Type
Negative

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input value field with incorrect data type or exceeding allowed character length

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJAO",
    "beneficiaryAccountNo": "1234567890"
}
```

## Expected Response
### Response Code
4008101

### Response Body
```json
{
    "responseCode": "4008101",
    "responseMessage": "Invalid Field Format beneficiaryBankCode"
}
```
