# Product Requirement Document (PRD)  
## BI-Fast Check Status API  

### 1. Overview  
Endpoint ini digunakan untuk mengetahui status transaksi pemindahbukuan (transfer) antar rekening real BRI yang dilakukan sebelumnya berdasarkan info dari rekening koran apakah sukses dilakukan pemindahbukuan atau tidak.  
Selain itu, endpoint ini juga dapat digunakan untuk:  
- Melakukan cek status RTGS apakah status terakhirnya sudah sukses diterima di Bank Lain atau belum.  
- Melakukan cek status transaksi transfer via **BI-Fast** apakah sudah sukses di rekening koran atau gagal.  

**Catatan:**  
Cek status transfer **BI-Fast** hanya bisa dilakukan maksimal **48 jam** setelah transaksi. Jika lebih dari itu, maka status informasi yang muncul bisa dianggap **invalid**.  

---

### 2. General Information  
| Parameter     | Value                          |
|--------------|--------------------------------|
| **HTTP Method** | `POST`                         |
| **Path**       | `/v2.0/transfer-bifast/check-status-bifast` |
| **Tipe Format** | `JSON`                         |
| **Authentication** | `OAuth 2.0 with Access Token` |

---

### 3. Header Structure & Sample  

| Key            | Value               | Format       | Mandatory | Length | Deskripsi                  |
|---------------|---------------------|-------------|----------|--------|----------------------------|
| Authorization | Bearer {Token}      | String      | M        | -      | Bearer {Token}             |
| X-TIMESTAMP   | timestamp           | Datetime    | M        | -      | Format Timestamp ISO8601   |
| X-SIGNATURE   | signature           | String      | M        | -      | SHA256-HMAC                |
| Content-type  | application/json    | -           | M        | -      | application/json           |
| X-PARTNER-ID  |                     |Alphanumeric        | M           | 36            | |
| CHANNEL-ID    |                     |Alphanumeric        | M           | 5             | |
| X-EXTERNAL-ID |                     |Numeric             | M           | 36            | |

### Request Structure & Sample

| Field                      | Data Type | Mandatory | Length | Description                                                                                                                                         | Example                        |
|----------------------------|-----------|----------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------|
| originalPartnerReferenceNo | String    | M        | 28     | Nomor referensi transaksi unique ketika dilakukan request internal fund transfer                                                                   | -                              |
| serviceCode                | String    | M        | 2      | Kode service jenis transaksi, meliputi: <br> - 17 : internal fund transfer <br> - 18 : transfer online **belum bisa diakomodir** <br> - 22 : RTGS transfer <br> - 23 : SKN transfer <br> - 33 : internal transfer to VA **belum bisa diakomodir** <br> - 80 : BI-Fast transfer | -                              |
| transactionDate            | String    | M        | 25     | Tanggal transaksi : ISO 8601                                                                                                                         | 2019-07-03T12:08:56+07:00      |
| additionalInfo             | Object    | O        | -      | Info tambahan yang diinputkan ketika request meliputi: <br> - deviceId <br> - channel                                                              | -                              |


### Response Structure & Sample

| Field                     | Data Type | Mandatory | Length | Description                                                                                                                                      | Example                          |
|---------------------------|-----------|----------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------|
| responseCode             | String    | M        | 4      | Response code                                                                                                                                   | -                                |
| responseMessage          | String    | M        | 50     | Deskripsi *response message*                                                                                                                     | -                                |
| originalPartnerReferenceNo | String  | O        | -      | Nomor referensi transaksi unique ketika dilakukan request internal fund transfer                                                                | -                                |
| originalReferenceNo      | String    | O        | -      | Journal sequence pembukuan                                                                                                                      | -                                |
| serviceCode              | String    | O        | 2      | Kode service jenis transaksi, meliputi: <br> - 17 : internal fund transfer <br> - 18 : transfer online **belum bisa diakomodir** <br> - 22 : RTGS transfer <br> - 23 : SKN transfer <br> - 33 : internal transfer to VA **belum bisa diakomodir** <br> - 80 : BI-Fast transfer | -                                |
| remark                   | String    | O        | 40     | Deskripsi transaksi                                                                                                                             | -                                |
| transactionDate          | String    | O        | -      | Tanggal transaksi : ISO8601                                                                                                                      | 2019-07-03T12:08:56+07:00        |
| sourceAccountNo         | String    | O        | 15     | Nomor rekening asal yang didebet                                                                                                                | -                                |
| beneficiaryAccountNo     | String    | O        | -      | Nomor rekening tujuan yang dikredit                                                                                                             | -                                |
| referenceNumber         | String    | O        | -      | Nomor referensi unik dengan format pembentuk : noreff dari SP + journals eq dari SP                                                            | -                                |
| latestTransactionStatus  | String    | O        | 4      | Status transaksi terakhir dari rekening koran, meliputi: <br> - 00 : sukses pembukuan/sukses ke Bank Lain <br> - 03 : pending dan perlu recheck <br> - 04 : transaksi dikembalikan/retur <br> - 06 : gagal pembukuan/gagal ke Bank Lain | -                                |
| transactionStatusDesc   | String    | O        | -      | Deskripsi transaksi status dari rekening koran atau dari Bank Indonesia                                                                         | -                                |
| amount                  | Object    | M        | 15     | Info amount beserta currency-nya : <br> - **value** : Jumlah nominal transaksi menggunakan delimiter titik "." untuk nilai decimal. <br> - **currency** : Mata uang transaksi. | value : 1000.00 <br> currency : IDR untuk mata uang Rupiah Indonesia |
| additionalInfo          | Object    | M        | -      | Info tambahan yang diinputkan ketika request meliputi: <br> - **deviceId** <br> - **channel**                                                  | -                                |


### Request & Response Payload Sample

#### Request:
```json
{
  "originalPartnerReferenceNo": "54321",
  "serviceCode": "80",
  "transactionDate": "2019-07-03T12:08:56+07:00"
}
```
#### Normal Response:
```json
{
  "responseCode": "2008200",
  "responseMessage": "Successful",
  "originalPartnerReferenceNo": "54321",
  "originalReferenceNo": "20220128BRINIDJA902359667275093",
  "serviceCode": "80",
  "transactionDate": "2019-07-03T12:08:56+07:00",
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

#### Error Response:
```json
{
  "responseCode": "4048200",
  "responseMessage": "Invalid Transaction Status"
}
```


### List of Error/Response Code

| HTTP Status | Service Code | Code     | Status                | Response Message         | Deskripsi                                      |
|------------|-------------|----------|----------------------|-------------------------|------------------------------------------------|
| 200        | 82          | 2008200  | Success              | Successful              | -                                              |
| 400        | 82          | 4008201  | Failed               | Invalid Field Format    | -                                              |
| 400        | 82          | 4008202  | Failed               | Invalid Mandatory Field | -                                              |
| 404        | 82          | 4048200  | Failed               | Invalid Transaction Status | -                                           |
| 404        | 82          | 4048201  | Failed               | Transaction Not Found   | -                                              |
| 500        | 82          | 5008201  | Suspend/Need To Check | Internal Server Error   | -                                              |
| 500        | 82          | 5008200  | Suspend/Need To Check | General Error           | Muncul ketika ada response yg belum terhandle |
| 504        | 82          | 5048200  | Suspend/Need To Check | Timeout                 | -                                              |
