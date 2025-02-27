# Test Case: Internal Server Error

## Description
Testing behavior when internal server error occurs

## Test Type
Negative

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input customerReference: 22222

## Request Body
```json
{
    "customerReference": "22222",
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
    "senderTownName": "0300"
}
```

## Expected Response
### Response Code
5008001

### Response Body
```json
{
    "responseCode": "5008001",
    "responseMessage": "Internal Server Error"
}
```
