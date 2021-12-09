# certbot

A quick and dirty interface to `certbot`. Expects to be `root` on an Ubuntu machine. Uses `apt install -y certbot` to make sure `certbot` is installed.

```rust
certbot::get_cert_paths("my@email.com", "thismachine.domain.com").unwrap();
```

```
CertPaths {
    cert: "/etc/letsencrypt/live/thismachine.domain.com/cert.pem",
    chain: "/etc/letsencrypt/live/thismachine.domain.com/chain.pem",
    fullchain: "/etc/letsencrypt/live/thismachine.domain.com/fullchain.pem",
    privkey: "/etc/letsencrypt/live/thismachine.domain.com/privkey.pem",
}
```
