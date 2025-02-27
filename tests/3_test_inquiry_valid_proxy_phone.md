# Test Case: Inquiry Proxy Account (Phone Number)

## Description
Valid scenario for inquiring using proxy phone number

## Test Type
Positive

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input all fields with valid data

## Request Body
```json
{
    "beneficiaryBankCode": "",
    "beneficiaryAccountNo": "@6285733347342"
}
```

## Expected Response
### Response Code
2008100

### Response Body
```json
{
    "responseCode": "2008100",
    "responseMessage": "Successful",
    "referenceNo": "20220127BRINIDJA61010000027",
    "externalId": "13297730559",
    "registrationId": "0001230075",
    "receiverName": "YAYASAN DHARMA BAKTI INDONESIA(Pemelik Rekening Kosong)",
    "beneficiaryAccountNo": "@6285733347342",
    "beneficiaryBankCode": "BRINIDJA",
    "beneficiaryAccountType": "SVGS",
    "accountNumber": "020601000988301",
    "receiverType": "01",
    "receiverResidentStatus": "01",
    "receiverIdentityNumber": "1216728398362819",
    "receiverTownName": "0300",
    "currency": "IDR"
}
```
