# multi-x509

> Run a command against each x509 certificate in the input stream

## Installation

``` console
$ cargo install
```

## Examples

Print the subject and issuer for certificates presented by google.com:

``` console
$ openssl s_client -showcerts -host google.com -port 443 </dev/null 2>/dev/null \
  | multi-x509 openssl x509 -noout -subject -issuer
subject= /C=US/ST=California/L=Mountain View/O=Google Inc/CN=*.google.com
issuer= /C=US/O=Google Inc/CN=Google Internet Authority G2
subject= /C=US/O=Google Inc/CN=Google Internet Authority G2
issuer= /C=US/O=GeoTrust Inc./CN=GeoTrust Global CA
subject= /C=US/O=GeoTrust Inc./CN=GeoTrust Global CA
issuer= /C=US/O=Equifax/OU=Equifax Secure Certificate Authority
```

