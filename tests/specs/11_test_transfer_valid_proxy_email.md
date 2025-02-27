# Test Case: Transfer with Proxy Email Account

## Description
Valid scenario for transfer using proxy email (using referenceNo and externalId from Inquiry Proxy Account Email)

## Test Type
Positive

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input all fields with valid data
3. Use transactionDate: "2022-02-22T13:07:24Z"

## Request Body
```json
{
    "customerReference": "88888",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "001901000378301",
    "amount": {
        "value": "12000.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "@testing@gmail.com",
    "referenceNo": "20220127BRINIDJA61010000026",
    "externalId": "13297657619",
    "transactionDate": "2022-02-22T13:07:24Z",
    "paymentInfo": "testing bifast",
    "senderType": "01",
    "senderResidentStatus": "01",
    "senderTownName": "0300",
    "additionalInfo": {
        "isRdn": "true",
        "deviceId": "12345679237",
        "channel": "mobilephone"
    }
}
```

## Expected Response
### Response Code
2008000

### Response Body
```json
{
    "responseCode": "2008000",
    "responseMessage": "Successful",
    "customerReference": "88888",
    "sourceAccountNo": "001901000378301",
    "beneficiaryAccountNo": "@testing@gmail.com",
    "beneficiaryBankCode": "CENAIDJA",
    "referenceNo": "20220128996328462262856287056344901",
    "externalId": "532101278586",
    "journalSequence": "4340807",
    "originalReferenceNo": "20220128BRINIDJA211953675017267",
    "amount": {
        "value": "12000.00",
        "currency": "IDR"
    }
}
```
