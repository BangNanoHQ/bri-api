# Test Case: Invalid Amount

## Description
Testing behavior when amount is invalid (less than minimum allowed)

## Test Type
Negative

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input amount less than 10000.00

## Request Body
```json
{
    "customerReference": "88888",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "13560",
    "amount": {
        "value": "23.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "55555",
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
4048013

### Response Body
```json
{
    "responseCode": "4048013",
    "responseMessage": "Invalid Amount"
}
```
