# Rust full stack project(Rust全栈项目)

This project is for personal practice of rust and is for reference only.

该项目为个人练习rust所用，仅供参考。

## Install

- `Rust`: quick install
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- 检查安装：
```sh
cargo -v
```
- [NodeJS](https://nodejs.org/en/download/package-manager)
- [PostgreSQL](https://www.postgresql.org/download/)
- [VScode](https://code.visualstudio.com/Download)
    - 拓展工具：`rust-analyzer`
- [DataBase](https://www.jetbrains.com/datagrip/)

## Related Background
[Rust 入门秘籍](https://rust-book.junmajinlong.com/about.html)

[Rust Course-语言圣经](https://course.rs/about-book.html)

[Rust By Practice-Rust练习实践](https://practice-zh.course.rs/why-exercise.html)

## Catalog Introduction

### DB - demo

基于`sqlx`和`actix`的使用，连接数据库服务

### S1 [workspace] - demo
包含四个模块 `http`, `httpserver`, `tcpclient`, `tcpserver`

- `http:`  自定义实现http结构，解析http请求、和响应结构，附带相关测试用例

- `httpserver:` 自定义实现http server结构，自定义服务路由及相关资源管理

- `tcpclient:` 创建tcp client demo

- `tcpserver:` 创建 tpc server demo

### WS(web server) - developer
目前包含`wasm-client`, `webapp`, `webservice` 三个目录

- `wasm-client:` WebAssembly 应用（拓展和维护成本较高，后续考虑替换为现代web框架应用，如`vue、react、svelte、redwood`。此出用作后台管理
    - 优点：
        - `快速、高效、可移植性`
            - 通过利用常见的硬件能力，`WebAssembly`代码在不同平台上能够以接近本地速度运行
        - `可读、可调试`
            - `WebAssembly` 是一门低阶语言，但是它有一种人类可读的文本格式，允许通过手工来编写、查看以及调试代码。
        - `保持安全`
            - `WebAssembly`被限制运行在一个安全的沙箱执行环境中，遵循浏览器的同源策略和授权策略。
        - `不破坏网络`
            - `WebAssembly`的设计原则是与其他网络技术和谐共处并保持向后兼容。
- `webapp:` rust前台项目，服务端渲染，考虑后续转移到web框架应用（同上）。
- `webservice:` web 后端服务，主要连接数据库以及创建api等

## Project Step
### Download Code
```sh
git clone https://github.com/hiiiiiiixiaohui/rs-full-stack-project.git
```
### Go to file directory
```sh
cd /file-path
```
### Compile and run for development/test
```sh
cargo run
```
## Release
### Compile release (e.g. :ws directory)
```sh
cd ./ws
```
### Build service release
```sh
cargo build --bin teacher-service --release
```
### Build webapp release
```sh
cargo build --bin webapp --release
```
### Then go to target file directory, find .exe file, and run it.(two .exe file)

### Go to wasm-client directory, run
```sh
cd ./wasm-client && wasm-pack build --relesae
```
### Go to web-server directory(use NodeJs v16)
```sh
npm i && npm run build
```

### Install local http server and run start local web
```sh
npm install -g http-server && http-server ./dist -p 8081
```

### Address
[frontEnd web server -> http://localhost:8080](http://localhost:8080)

[manegement sys server -> http://localhost:8081](http://localhost:8081)

[backEnd Api server -> http://localhost:3030](http://localhost:3030)
