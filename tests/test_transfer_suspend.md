# Test Case: Suspend Transaction

## Description
Testing behavior when transaction is suspended

## Test Type
Negative

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input sourceAccountNo: 1234567890

## Request Body
```json
{
    "customerReference": "123456789",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "1234567890",
    "amount": {
        "value": "10000.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "99999",
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
4038016

### Response Body
```json
{
    "responseCode": "4038016",
    "responseMessage": "Suspend Transaction"
}
```
