# Test Case: Transfer with Real Account

## Description
Valid scenario for transfer using real account (using referenceNo and externalId from Inquiry Real Account)

## Test Type
Positive

## Steps
1. Send transfer request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/payment-bifast
2. Input all fields with valid data
3. Use transactionDate: "2022-02-22T13:07:24Z"

## Request Body
```json
{
    "customerReference": "54321",
    "senderIdentityNumber": "3515085211930002",
    "sourceAccountNo": "001901000378301",
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
    "customerReference": "54321",
    "sourceAccountNo": "001901000378301",
    "beneficiaryAccountNo": "12345678900",
    "beneficiaryBankCode": "CENAIDJA",
    "referenceNo": "20220128CENAIDJA1698678312816763100",
    "externalId": "038455145464",
    "journalSequence": "8292049",
    "originalReferenceNo": "20220128BRINIDJA794224862497256",
    "amount": {
        "value": "120000.00",
        "currency": "IDR"
    }
}
```
