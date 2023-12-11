upload photo

```bash
curl 'http://127.0.0.1:8000/api/photo_upload' -X POST -H 'Accept: image/avif,image/webp,*/*' -H 'Accept-Encoding: gzip, deflate, br' -H 'Referer: http://127.0.0.1:8000/api/photo_upload' -H 'DNT: 1' -H 'Connection: keep-alive' -H 'Sec-Fetch-Dest: image' -H 'Sec-Fetch-Mode: no-cors' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-GPC: 1' -H 'Content-Type: application/json' -H 'Origin: http://127.0.0.1:8000' -H 'Pragma: no-cache' -H 'Cache-Control: no-cache' --data-raw '{"url":"https://foundation.rust-lang.org/img/cargo.png"}'
```

get concrete file:
```http://127.0.0.1:8000/content/fb2f15c8-d92f-43c4-b752-773d82f39c56.jpg```