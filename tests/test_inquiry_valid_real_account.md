# Test Case: Inquiry Real Account

## Description
Valid scenario for inquiring a real account

## Test Type
Positive

## Steps
1. Send inquiry request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/inquiry-bifast
2. Input all fields with valid data

## Request Body
```json
{
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountNo": "99999"
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
    "referenceNo": "20220127BRINIDJA61050000018",
    "externalId": "53296848727",
    "registrationId": "0001230075", 
    "receiverName": "KR DUMMY ACCOUNT TBXXXXXXXXXXXXXXXXXXXXX",
    "beneficiaryAccountNo": "99999",
    "beneficiaryBankCode": "CENAIDJA",
    "beneficiaryAccountType": "SVGS",
    "accountNumber": "020601000988301",
    "receiverType": "02",
    "receiverResidentStatus": "01",
    "receiverIdentityNumber": "1216728398362819",
    "receiverTownName": "0300",
    "currency": "IDR"
}
```
