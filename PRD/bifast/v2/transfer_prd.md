# BI-Fast Transfer - Product Requirements Document (PRD)

## General Information

**Endpoint:** Digunakan untuk melakukan instruksi pemindahbukuan (transfer) dari rekening BRI ke rekening bank lain melalui switching Bank Indonesia (BI-Fast). Terdapat limit transaksi dan limit transaksi harian yang sudah diatur sebelumnya. Untuk transaksi ini, sumber dananya tidak dapat menggunakan rekening VA, melainkan hanya bisa menggunakan rekening real BRI saja.

### HTTP Method & Path
- **Method:** POST
- **Path:** `/v2.0/transfer-bifast/payment-bifast`
- **Format:** JSON
- **Authentication:** OAuth 2.0 dengan Access Token

## Header Structure

| Field          | Data Type    | Mandatory | Length | Description |
|---------------|-------------|-----------|--------|-------------|
| Authorization | String      | M         | 36     | Bearer {Token} |
| X-TIMESTAMP  | Datetime    | M         | 5      | Format Timestamp ISO8601 |
| X-SIGNATURE  | String      | M         | 36     | SHA256-HMAC |
| Content-type  | String      | M         | -      | application/json |
| X-PARTNER-ID  | Alphanumeric | M        | -      | ID mitra |
| CHANNEL-ID    | Alphanumeric | M        | -      | ID channel |
| X-EXTERNAL-ID | Numeric     | M         | -      | ID eksternal |

## Request Structure & Sample

| Field                | Data Type | Mandatory | Length | Description | Example |
|----------------------|-----------|-----------|--------|-------------|---------|
| customerReference   | String    | M         | 28     | Nomor referensi transaksi unique dari institusi. *Hanya allow karakter spasi, underscore (_) dan dash (-).* | - |
| beneficiaryBankCode | String    | C         | 8      | Kode bank tujuan. *Wajib diisi jika beneficiaryAccountNo bukan menggunakan proxy resolution.* | CENAIDJA untuk Bank BCA |
| sourceAccountNo     | String    | M         | 15     | Sumber rekening asal bank BRI | - |
| beneficiaryAccountNo | String   | M         | 34     | Rekening bank lain tujuan transaksi *(real account/proxy resolution)* | @bankbri@corp.bri.co.id (proxy resolution email) |
| referenceNo         | String    | M         | 35     | Nomor referensi unik dari hasil *Inquiry BI-Fast Account Information* | - |
| externalId          | String    | M         | 12     | ID eksternal BRI dari hasil *Inquiry BI-Fast Account Information* | - |
| amount             | Object    | M         | Max 15 | Info jumlah transaksi dan mata uang | {"value": "1000.00", "currency": "IDR"} |
| senderIdentityNumber | String  | M         | 100    | Nomor KTP pengirim transaksi | - |
| paymentInfo        | String    | O         | 140    | Keterangan transaksi | - |
| senderType         | String    | M         | 2      | Tipe pengirim transaksi (01=Individual, 02=Corporate, 99=Others, 04=Remittance, 03=Government) | 01 |
| senderResidentStatus | String  | M         | 2      | Status kependudukan pengirim transaksi (01=Resident, 02=NonResident) | 01 |
| senderTownName     | String    | M         | 100    | Kota asal pengirim transaksi | 0300 untuk kota Jakarta |
| transactionDate    | String    | M         | 19     | Tanggal dan jam transaksi dalam format: yyyy-MM-ddTHH:mm:ssZ | - |
| additionalInfo     | Object    | O         | -      | Info tambahan (deviceId, channel, isRdn) | {"deviceId": "12345679237", "channel": "mobilephone", "isRdn": "true"} |

## Response Structure & Sample

| Field                | Data Type | Mandatory | Length | Description | Example |
|----------------------|-----------|-----------|--------|-------------|---------|
| responseCode        | String    | M         | 4      | Response Code | - |
| responseMessage    | String    | M         | 250    | Deskripsi response message | - |
| originalReferenceNo | String    | C         | 40     | Nomor referensi original unik dari transaksi transfer yang dilakukan dan akan muncul di remark TLBD2 di rekening koran | - |
| journalSequence     | String    | C         | 7      | Journal sequence pembukuan transaksi | - |
| referenceNo         | String    | C         | 35     | Nomor referensi unik hasil dari transaksi transfer yang dilakukan | - |
| externalId          | String    | C         | 12     | ID eksternal BRI dari transaksi transfer yang dilakukan | - |
| customerReference   | String    | O         | 28     | Nomor referensi transaksi unique dari institusi | - |
| beneficiaryAccountNo | String   | O         | 34     | Nomor rekening tujuan transaksi | - |
| beneficiaryBankCode | String    | O         | 3      | Kode bank tujuan transaksi | CENAIDJA untuk Bank BCA |
| amount             | Object    | C         | Max 15 | Info jumlah transaksi dan mata uang | {"value": "1000.00", "currency": "IDR"} |
| sourceAccountNo    | String    | O         | 15     | Sumber rekening asal bank BRI | - |
| additionalInfo     | Object    | C         | -      | Informasi tambahan (deviceId, channel, isRdn) | - |


## Request & Response Payload Sample
### Request:
#### Transfer Real Account

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
        "deviceId": "12345679237",
        "channel": "mobilephone"
    }
}
```

#### Transfer Proxy Account (Email)

```json
{
  "customerReference": "0000004",
  "senderIdentityNumber": "3515085211930002",
  "sourceAccountNo": "001901000613303",
  "amount": {
    "value": "23000.00",
    "currency": "IDR"
  },
  "beneficiaryBankCode": "",
  "beneficiaryAccountNo": "@testing@gmail.com",
  "referenceNo": "20220929BRINIDJA51050005403",
  "externalId": "54445790453",
  "transactionDate": "2022-11-30T22:25:24+07:00",
  "paymentInfo": "345345",
  "senderType": "01",
  "senderResidentStatus": "01",
  "senderTownName": "0300",
  "additionalInfo": {
    "deviceId": "12345679237",
    "channel": "mobilephone",
    "isrdn": "true"
  }
}
```

#### Transfer Proxy Account (No Hp)

```json
{
  "customerReference": "0000004",
  "senderIdentityNumber": "3515085211930002",
  "sourceAccountNo": "001901000613303",
  "amount": {
    "value": "23000.00",
    "currency": "IDR"
  },
  "beneficiaryBankCode": "",
  "beneficiaryAccountNo": "@6285733347342",
  "referenceNo": "20220929BRINIDJA51050005403",
  "externalId": "54445790453",
  "transactionDate": "2022-11-30T22:25:24+07:00",
  "paymentInfo": "345345",
  "senderType": "01",
  "senderResidentStatus": "01",
  "senderTownName": "0300",
  "additionalInfo": {
    "deviceId": "12345679237",
    "channel": "mobilephone",
    "isrdn": "true"
  }
}
```

### Normal Response

#### Transfer Real Account

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
  },
  "additionalInfo": {
    "deviceId": "12345679237",
    "channel": "mobilephone",
    "isrdn": "True"
  }
}
```


#### Transfer Proxy Account (Email)

```json
{
  "responseCode": "2008000",
  "responseMessage": "Successful",
  "customerReference": "88888",
  "sourceAccountNo": "001901000378301",
  "beneficiaryAccountNo": "@testing@gmail.com",
  "referenceNo": "20220128996328462262856287056344901",
  "externalId": "532101278586",
  "journalSequence": "4340807",
  "originalReferenceNo": "20220128BRINIDJA211953675017267",
  "amount": {
    "value": "12000.00",
    "currency": "IDR"
  },
  "additionalInfo": {
    "deviceId": "12345679237",
    "channel": "mobilephone",
    "isRdn": "true"
  }
}
```

#### Transfer Proxy Account (No Hp)

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
  },
  "additionalInfo": {
    "deviceId": "12345679237",
    "channel": "mobilephone",
    "isRdn": "true"
  }
}
```


### Error Response

```json
{
  "responseCode": "4038002",
  "responseMessage": "Exceeds Transaction Amount Limit"
}


## List of Error/Response Code

| HTTP Status | Service Code | Code    | Status Response        | Message                         | Deskripsi |
|------------|--------------|---------|------------------------|---------------------------------|-----------|
| 200        | 80           | 2008000 | Success                | Successful                      | -         |
| 202        | 80           | 2028000 | Suspend / Need To Check | Request In Progress             | Mohon untuk melakukan pengecekan rekening koran untuk melihat status transfer, atau menggunakan endpoint check status |
| 400        | 80           | 4008001 | Failed                 | Invalid Field Format {fieldName} | -         |
| 400        | 80           | 4008002 | Failed                 | Invalid Mandatory Field {fieldName} | -         |
| 401        | 80           | 4018000 | Failed                 | Unauthorized. Partner-id not allow | -         |
| 401        | 80           | 4018000 | Failed                 | Unauthorized. [Reason]          | -         |
| 403        | 80           | 4038002 | Failed                 | Exceeds Transaction Amount Limit | -         |
| 403        | 80           | 4038009 | Failed                 | Dormant Account                 | -         |
| 403        | 80           | 4038014 | Failed                 | Insufficient Funds              | -         |
| 403        | 80           | 4038015 | Failed                 | Transaction Not Permitted. [Reason] | -         |
| 403        | 80           | 4038016 | Suspend / Need To Check | Suspend Transaction             | Mohon untuk melakukan pengecekan rekening koran untuk melihat status transfer, atau menggunakan endpoint check status |
| 403        | 80           | 4038018 | Failed                 | Inactive Card/Account/Customer  | -         |
| 404        | 80           | 4048000 | Failed                 | Invalid Transaction Status      | -         |
| 404        | 80           | 4048011 | Failed                 | Invalid Card/Account/Customer [info]/Virtual Account | - |
| 404        | 80           | 4048013 | Failed                 | Invalid Amount                  | -         |
| 409        | 80           | 4098001 | Gagal                  | Duplicate customerReference     | -         |
| 500        | 80           | 5008001 | Suspend / Need To Check | Internal Server Error           | -         |
| 500        | 80           | 5008000 | Suspend / Need To Check | General Error                   | Muncul ketika ada response yg belum terhandle |
| 504        | 80           | 5048000 | Suspend / Need To Check | Timeout                         | -         |
