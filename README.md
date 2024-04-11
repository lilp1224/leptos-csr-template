```shell
cargo generate --git https://github.com/noobmastercn/leptos-csr-template
```

```shell
brew install trunk
npm install -D tailwindcss
npx tailwindcss init
```

```shell
trunk serve
```

```shell
trunk build --release --no-default-features
```


```shell
server {
    listen 80;
    server_name yourdomain.com;

    root /var/www/html;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html; # 当Nginx无法直接匹配到请求的文件或目录时，重定向请求到/index.html，这时前端路由器可以接管并根据路由逻辑显示相应的页面，包括自定义的404页面。
    }
}
```
