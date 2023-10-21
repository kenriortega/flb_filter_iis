# Fluent-bit Plugin to process IIS logs


Doc: [https://docs.fluentbit.io/manual/development/wasm-filter-plugins](https://docs.fluentbit.io/manual/development/wasm-filter-plugins)


This filter takes the Internet Information Services (IIS) w3c logs (with some custom modifications) and transforms the raw string into a standard Fluent Bit JSON structured record.


> Compile to WASM

```console
$ cargo build --target wasm32-unknown-unknown --release
$ ls target/wasm32-unknown-unknown/release/*.wasm
target/wasm32-unknown-unknown/release/filter_rust.wasm
```


## How to configure 


```ini
[INPUT]
    Name             dummy
    Dummy            {"log": "2023-08-11 19:56:44 W3SVC1 WIN-PC1 ::1 GET / - 80 ::1 Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/115.0.0.0+Safari/537.36+Edg/115.0.1901.200 - - localhost 304 142 756 1078 -"}
    Tag              iis.*

[FILTER]
    Name             wasm
    match            iis.*
    WASM_Path        /plugins/flb_filter_iis_wasm.wasm
    Function_Name    flb_filter_log_iis_w3c_custom
    accessible_paths .

[OUTPUT]
    name             stdout
    match            iis.*
```

The incoming raw strings from an IIS log are composed of the following fields:
date time s-sitename s-computername s-ip cs-method cs-uri-stem cs-uri-query s-port c-ip cs(User-Agent) cs(Cookie) cs(Referer) cs-host sc-status sc-bytes cs-bytes time-taken c-authorization-header
The output after the filter logic will be:

```console

#Software: Microsoft Internet Information Services 10.0
#Version: 1.0
#Date: 2023-07-20 17:17:36
#Fields: date time s-sitename s-computername s-ip cs-method cs-uri-stem cs-uri-query s-port c-ip cs(User-Agent) cs(Cookie) cs(Referer) cs-host sc-status sc-bytes cs-bytes time-taken c-authorization-header
2023-07-20 17:18:54 W3SVC279 WIN-PC1 192.168.1.104 GET /api/Site/site-data qName=quww 13334 10.0.0.0 Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/114.0.0.0+Safari/537.36+Edg/114.0.1823.82 _ga=GA1.1.499592451.1685996504;+_gid=GA1.1.1209215542.1689808850;+_ga_PCBRLY5C8Y=GS1.1.1689811012.8.0.1689811012.0.0.0 http://192.168.1.104:13334/swagger/index.html 192.168.1.104:13334 200 456 1082 3131 Bearer+token
```

### Output

```console
[0] iis.*: [[1692131925.559486675, {}], {"c_authorization_header"=>"-", "c_ip"=>"::1", "cs_bytes"=>756, "cs_cookie"=>"-", "cs_host"=>"localhost", "cs_method"=>"GET", "cs_referer"=>"-", "cs_uri_query"=>"-", "cs_uri_stem"=>"/", "cs_user_agent"=>"Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/115.0.0.0+Safari/537.36+Edg/115.0.1901.200", "date"=>"2023-08-11 19:56:44", "s_computername"=>"WIN-PC1", "s_ip"=>"::1", "s_port"=>"80", "s_sitename"=>"W3SVC1", "sc_bytes"=>142, "sc_status"=>"304", "source"=>"LogEntryIIS", "tag"=>"iis.*", "time"=>"2023-08-15T20:38:45.559486675 +0000", "time_taken"=>1078}]
```


