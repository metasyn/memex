# administration

below are some sites i use to check on whether a server is being configured correctly

## domain / general

- https://internet.nl/site/metasyn.pw/ - tests for IPV6 compatibility, signed domain name, HTTPS, security options, DNSSEC, route authorization
- https://observatory.mozilla.org/analyze/metasyn.pw - has HTTP, TLS, and SSH tests - including HSTS, various headers, CORS, etc

## security

- https://securityheaders.com/?followRedirects=on&hide=on&q=metasyn.pw
- https://www.immuniweb.com/ssl/metasyn.pw/fvUEuDm8/
- https://tls.imirhil.fr/https/metasyn.pw

## ssh

- https://www.sshaudit.com/ - tests server configurations related to SSH - encryption ciphers, key exchanges, host key types, message authentication codes
- https://www.linode.com/docs/guides/using-fail2ban-to-secure-your-server-a-tutorial/ - add fail2ban

## performance

- https://tools.pingdom.com/#6104c94ab2000000m - simple page optimizations, page load speed/size
- https://gtmetrix.com/reports/metasyn.pw/Mbto37Gk/ - dom loading, css, overall performance
- https://pagespeed.web.dev/report?url=https%3A%2F%2Fmetasyn.pw%2 - diagnose performance issues, also has notes about accessibility
