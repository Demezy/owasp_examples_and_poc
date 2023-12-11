access 0.0.0.0:8001 (if deploy in docker compose)

service support annotation text as html. Mostly good feature for self expression.
but JS is allowed.

send malicios message
```bash
curl 'http://127.0.0.1:5000/' -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br' -H 'DNT: 1' -H 'Connection: keep-alive' -H 'Upgrade-Insecure-Requests: 1' -H 'Sec-Fetch-Dest: document' -H 'Sec-Fetch-Mode: navigate' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-GPC: 1'
```

all clients when refresh page invoke js

possible attack examples:
send cookies to bad guy server

how to fix:
- filter JS tags
- limit HTML features, since svg, img tags also could call js
- hide server behind filtering firewall, as with ssrf, nginx has cool ones