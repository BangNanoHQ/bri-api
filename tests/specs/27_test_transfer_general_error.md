# Test Case: Transfer General Error

## Description
Testing behavior when general error occurs

## Test Type
Negative

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input customerReference: 33333

## Request Body
```json
{
    "customerReference": "33333",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "001901000378301",
    "amount": {
        "value": "10000.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "12345678900",
    "referenceNo": "20220127BRINIDJA61050000018",
    "externalId": "53394951711",
    "transactionDate": "2022-01-28T13:30:24Z",
    "paymentInfo": "testing bifast",
    "senderType": "01",
    "senderResidentStatus": "01",
    "senderTownName": "0300",
    "additionalInfo": {
        "isRdn": "false",
        "deviceId": "12345679237",
        "channel": "mobilephone"
    }
}
```

## Expected Response
### Response Code
5008000

### Response Body
```json
{
    "responseCode": "5008000",
    "responseMessage": "General Error"
}
```
