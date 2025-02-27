# Test Case: Valid Check Status

## Description
Valid scenario for checking transaction status

## Test Type
Positive

## Steps
1. Send check status request to endpoint: https://sandbox.partner.api.bri.co.id/v2.0/transfer-bifast/check-status-bifast
2. Input all fields with valid data
3. Use transactionDate: "22-02-2022"

## Request Body
```json
{
    "originalPartnerReferenceNo": "54321",
    "serviceCode": "80",
    "transactionDate": "22-02-2022"
}
```

## Expected Response
### Response Code
2008200

### Response Body
```json
{
    "responseCode": "2008200",
    "responseMessage": "Successful",
    "originalPartnerReferenceNo": "54321",
    "originalReferenceNo": "20220128BRINIDJA369512651746837",
    "serviceCode": "80",
    "transactionDate": "2022-02-22T13:07:24Z",
    "amount": {
        "value": "120000.00",
        "currency": "IDR"
    },
    "referenceNumber": "ID2131925142835ST17241796",
    "sourceAccountNo": "55555",
    "beneficiaryAccountNo": "12345678900",
    "remark": "2131925142835ST1",
    "latestTransactionStatus": "00",
    "transactionStatusDesc": "Payment success",
    "additionalInfo": {}
}
```
