# Fluent-bit Plugin to process IIS logs



> Tags to process `date time s-sitename s-computername s-ip cs-method cs-uri-stem cs-uri-query s-port c-ip cs(User-Agent) cs(Cookie) cs(Referer) cs-host sc-status sc-bytes cs-bytes time-taken c-authorization-header`

```console

#Software: Microsoft Internet Information Services 10.0
#Version: 1.0
#Date: 2023-07-20 17:17:36
#Fields: date time s-sitename s-computername s-ip cs-method cs-uri-stem cs-uri-query s-port c-ip cs(User-Agent) cs(Cookie) cs(Referer) cs-host sc-status sc-bytes cs-bytes time-taken c-authorization-header
2023-07-20 17:18:54 W3SVC279 WIN-PC1 192.168.1.104 GET /api/Site/site-data qName=quww 13334 10.0.0.0 Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/114.0.0.0+Safari/537.36+Edg/114.0.1823.82 _ga=GA1.1.499592451.1685996504;+_gid=GA1.1.1209215542.1689808850;+_ga_PCBRLY5C8Y=GS1.1.1689811012.8.0.1689811012.0.0.0 http://192.168.1.104:13334/swagger/index.html 192.168.1.104:13334 200 456 1082 3131 Bearer+token
```


> Compile to WASM

```console
$ cargo build --target wasm32-unknown-unknown --release
$ ls target/wasm32-unknown-unknown/release/*.wasm
target/wasm32-unknown-unknown/release/filter_rust.wasm
```


## How to configure 


```ini
[SERVICE]
    Flush        1
    Daemon       Off
    Log_Level    info
    HTTP_Server  Off
    HTTP_Listen  0.0.0.0
    HTTP_Port    2020

[INPUT]
    Name dummy
    Tag  iis.local

[FILTER]
    Name   wasm
    match  iis.*
    WASM_Path C:\Users\kenri\github.com\otel_project\flb_filter_iis_wasm\target\wasm32-unknown-unknown\release\flb_filter_iis_wasm.wasm
    Function_Name flb_filter_log_iis
    accessible_paths C:\Users\kenri\github.com\otel_project\fluent-bit-2.1.8-win64

[OUTPUT]
    Name  stdout
    Match *
```