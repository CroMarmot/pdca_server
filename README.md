# README

## Dependence

### DB

mongodb

## start

```
cargo run
```

## Client

[pdca_web](https://github.com/CroMarmot/pdca_web)

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
db["pdca_daily"].find()
```


## BUG

后台关掉没有 stdout时会炸

临时方案 `./server 1>stdout.log 2>stderr.log &`
