main page is 
<url>/gallery


upload photo

```bash
curl 'http://127.0.0.1:8000/api/photo_upload' -X POST -H 'Accept: image/avif,image/webp,*/*' -H 'Accept-Encoding: gzip, deflate, br' -H 'Referer: http://127.0.0.1:8000/api/photo_upload' -H 'DNT: 1' -H 'Connection: keep-alive' -H 'Sec-Fetch-Dest: image' -H 'Sec-Fetch-Mode: no-cors' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-GPC: 1' -H 'Content-Type: application/json' -H 'Origin: http://127.0.0.1:8000' -H 'Pragma: no-cache' -H 'Cache-Control: no-cache' --data-raw '{"url":"https://foundation.rust-lang.org/img/cargo.png"}'
```

server does not handle any mime type or anthing. Ask it to do request something from local network, and it would do that. Moreover downloaded file will be available via link below.
So, at leas it could be used to broke intra network isolation, as a proxy for dark deals (limitied with get requests)

get concrete file:
```http://127.0.0.1:8000/content/fb2f15c8-d92f-43c4-b752-773d82f39c56.jpg```

How to fix:
- add validation for response mimetype
- make strict policy for deny any access to internal network (since service does not require one)
- place server behind filtering proxy that drop all packages with suspecious requests (nginx has plenty of configs for it)
- clear get paramenters (since this is significant attack surface)