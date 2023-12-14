# Cross-Site Scripting

By default the service is available at `localhost:8001`

The service supports formatting text by using HTML. This is a useful feature, hoverer one can also embed JavaScript.

## Exploit

Sending malicious message:

```bash
curl 'http://127.0.0.1:5000/' -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br' -H 'DNT: 1' -H 'Connection: keep-alive' -H 'Upgrade-Insecure-Requests: 1' -H 'Sec-Fetch-Dest: document' -H 'Sec-Fetch-Mode: navigate' -H 'Sec-Fetch-Site: same-origin' -H 'Sec-GPC: 1'
```

All the clients will execute JavaScript code snippet when refreshing the page.
For e.g. one may insert JS that sends collects cookies and sends them to
malicious server

## Fixes

- filter JS tags
- limit HTML features, since svg, img tags also could call js
- hide server behind filtering firewall, as with ssrf, nginx has cool ones

## References

- [Definition at owasp.org](https://owasp.org/www-community/attacks/xss)
