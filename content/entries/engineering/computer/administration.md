# administration

below are some sites i use to check on whether a server is being configured correctly

## domain / general

- [Internet NL](https://internet.nl/site/metasyn.pw/)
  - tests for IPV6 compatibility, signed domain name, HTTPS, security options, DNSSEC, route authorization
  - complaints:
    - ipv6 connectivity
    - dnssec is not supported by my VPS provider
- [Mozilla Observatory](https://observatory.mozilla.org/analyze/metasyn.pw) - B
  - has HTTP, TLS, and SSH tests - including HSTS, various headers, CORS, etc
  - suggests I set `Content-Security-Policy`

## security

- [Security Headers](https://securityheaders.com/?followRedirects=on&hide=on&q=metasyn.pw) - A
  - I still need to add a `Content-Security-Policy`
- [Immuniweb](https://www.immuniweb.com/ssl/metasyn.pw/fvUEuDm8/) - A
- [CryptCheck](https://tls.imirhil.fr/https/metasyn.pw) - B
  - They suggest improving key exchange related things

## ssh

- [SSH Audit](https://www.sshaudit.com/) - tests server configurations related to SSH - encryption ciphers, key exchanges, host key types, message authentication codes
- [fail2ban setup tutorial](https://www.linode.com/docs/guides/using-fail2ban-to-secure-your-server-a-tutorial/) - add fail2ban

## performance

- https://tools.pingdom.com/#6104c94ab2000000m - simple page optimizations, page load speed/size
- https://gtmetrix.com/reports/metasyn.pw/Mbto37Gk/ - dom loading, css, overall performance
- https://pagespeed.web.dev/report?url=https%3A%2F%2Fmetasyn.pw%2 - diagnose performance issues, also has notes about accessibility
