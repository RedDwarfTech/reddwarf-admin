#### 分页查询频道



```bash
curl 'http://127.0.0.1:8000/manage/app/cruise/channel/v1/page' \
  -H 'Accept: */*' \
  -H 'Accept-Language: en,zh-CN;q=0.9,zh;q=0.8,zh-TW;q=0.7,fr;q=0.6' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: text/plain;charset=UTF-8' \
  -H 'DNT: 1' \
  -H 'Origin: http://127.0.0.1:8083' \
  -H 'Referer: http://127.0.0.1:8083/app/cruise/channel' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36' \
  -H 'sec-ch-ua: " Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "macOS"' \
  -H 'x-access-token: xxxxxxxxxxxxxxx' \
  -H 'x-request-id: 31e8c0fa-be6b-4d06-9fa5-54549fe79967' \
-H 'app-id: jgjguyyg' \
-H 'x-app-id: jgjguyyg' \
-H 'user-id: 1' \
  --data-raw '{"current":1,"pageSize":20,"pageNum":1,"editorPick":null,"minimalReputation":0}' \
  --compressed
```









#### 更新频道标签



```bash
curl 'http://127.0.0.1:8000/manage/app/cruise/channel/v1/update' \
  -X 'PUT' \
  -H 'Accept: */*' \
  -H 'Accept-Language: en,zh-CN;q=0.9,zh;q=0.8,zh-TW;q=0.7,fr;q=0.6' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: text/plain;charset=UTF-8' \
  -H 'DNT: 1' \
  -H 'Origin: http://127.0.0.1:8083' \
  -H 'Referer: http://127.0.0.1:8083/app/cruise/channel' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36' \
  -H 'sec-ch-ua: " Not A;Brand";v="99", "Chromium";v="100", "Google Chrome";v="100"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "macOS"' \
  -H 'x-access-token: eyJhbGciOiJIUzUxMiJ9.eyJ1c2VySWQiOjEsImRldmljZUlkIjoieHh4eHgiLCJleHAiOjE2NTE0MDM4ODB9.D5pxWhzgaE22GK4zazBiEvDeJh8h07YNqaH2XDHrs5AGb2XT8ucT1NJYSGYlPEpWltxFfEuOWaj0N6eebE0eIA' \
  -H 'x-request-id: b0359bf6-010f-4b2a-9475-c166ff97b3c9' \
  -H 'app-id: b0359bf6-010f-4b2a-9475-c166ff97b3c9' \
  -H 'user-id: b0359bf6-010f-4b2a-9475-c166ff97b3c9' \
  --data-raw '{"channelId":4767,"tags":"[\"DEVELOPER\"]"}' \
  --compressed
```








