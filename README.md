# README

## Dependence

### DB

mongodb

## start

```
cargo run
```


## Base framework

[actix-web](https://actix.rs/)

## nginx config example

```
location /api/ {
    proxy_pass http://127.0.0.1:8088; // this server
}
location / {
    proxy_pass http://127.0.0.1:3000; // the nuxt client
}
```

## mongodb example

```bash
mongo
use pdca_v1
db["some-coll"].find()
```