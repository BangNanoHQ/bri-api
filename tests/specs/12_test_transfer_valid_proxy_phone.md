# Test Case: Transfer with Proxy Phone Number

## Description
Valid scenario for transfer using proxy phone number (using referenceNo and externalId from Inquiry Proxy Account Phone)

## Test Type
Positive

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input all fields with valid data
3. Use transactionDate: "2022-02-22T13:07:24Z"

## Request Body
```json
{
    "customerReference": "77777",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "001901000378301",
    "amount": {
        "value": "15000.00",
        "currency": "IDR"
    },
    "beneficiaryBankCode": "",
    "beneficiaryAccountNo": "@6285733347342",
    "referenceNo": "20220127BRINIDJA61010000027",
    "externalId": "13297730559",
    "transactionDate": "2022-02-22T13:07:24Z",
    "paymentInfo": "testing bifast",
    "senderType": "01",
    "senderResidentStatus": "01",
    "senderTownName": "0300",
    "additionalInfo": {
        "isRdn":"true",
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
    "customerReference": "77777",
    "sourceAccountNo": "001901000378301",
    "beneficiaryAccountNo": "@6285733347342",
    "referenceNo": "20220128021710978140260165817498863",
    "externalId": "875014615127",
    "journalSequence": "0869256",
    "originalReferenceNo": "20220128BRINIDJA514631900565000",
    "amount": {
        "value": "12000.00",
        "currency": "IDR"
    }
}
```
