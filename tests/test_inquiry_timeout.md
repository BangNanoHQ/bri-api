# Test Case: Timeout

## Description
Testing behavior when request times out

## Test Type
Negative

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input beneficiaryAccountNo: 44444

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "44444"
}
```

## Expected Response
### Response Code
5048100

### Response Body
```json
{
    "responseCode": "5048100",
    "responseMessage": "Timeout"
}
```
