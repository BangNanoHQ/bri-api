# Test Case: Exceeds Transaction Amount Limit

## Description
Testing behavior when transaction amount exceeds limit

## Test Type
Negative

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input sourceAccountNo: 99999

## Request Body
```json
{
    "customerReference": "54321",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "99999",
    "amount": {
        "value": "120000.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "12345678900",
    "referenceNo": "20220127BRINIDJA61050000018",
    "externalId": "53394951711",
    "transactionDate": "2022-02-22T13:07:24Z",
    "paymentInfo": "testing bifast",
    "senderType": "01",
    "senderResidentStatus": "01",
    "senderTownName": "0300",
    "additionalInfo": {
        "isRdn":"false",
        "deviceId": "12345679237",
        "channel": "mobilephone"
    }
}
```

## Expected Response
### Response Code
4038002

### Response Body
```json
{
    "responseCode": "4038002",
    "responseMessage": "Exceeds Transaction Amount Limit"
}
```
