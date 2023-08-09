use regex::Regex;
use serde::{Deserialize, Serialize};
// Import pure and fast JSON library written in Rust
use chrono::{TimeZone, Utc};
use serde_json::json;
use serde_json::Value;
use std::io::Write;
use std::os::raw::c_char;
use std::slice;
use std::str;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntryIIS {
    date_time: String,
    s_sitename: String,
    s_computername: String,
    s_ip: String,
    cs_method: String,
    cs_uri_stem: String,
    cs_uri_query: String,
    s_port: String,
    c_ip: String,
    cs_user_agent: String,
    cs_cookie: String,
    cs_referer: String,
    cs_host: String,
    sc_status: String,
    sc_bytes: String,
    cs_bytes: String,
    time_taken: String,
    c_authorization_header: String,
}

impl LogEntryIIS {
    pub fn parse_log_iis(input: &str) -> Option<Self> {
        let re = Regex::new(r#"^(\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}:\d{2})\s(\S+)\s(\S+)\s(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)\s(\S+)"#).unwrap();

        if let Some(captures) = re.captures(input) {
            let date_time = captures.get(1).map(|m| m.as_str()).unwrap().to_string();
            let s_sitename = captures.get(2).map(|m| m.as_str()).unwrap().to_string();
            let s_computername = captures.get(3).map(|m| m.as_str()).unwrap().to_string();
            let s_ip = captures.get(4).map(|m| m.as_str()).unwrap().to_string();
            let cs_method = captures.get(5).map(|m| m.as_str()).unwrap().to_string();
            let cs_uri_stem = captures.get(6).map(|m| m.as_str()).unwrap().to_string();
            let cs_uri_query = captures.get(7).map(|m| m.as_str()).unwrap().to_string();
            let s_port = captures.get(8).map(|m| m.as_str()).unwrap().to_string();
            let c_ip = captures.get(9).map(|m| m.as_str()).unwrap().to_string();
            let cs_user_agent = captures.get(10).map(|m| m.as_str()).unwrap().to_string();
            let cs_cookie = captures.get(11).map(|m| m.as_str()).unwrap().to_string();
            let cs_referer = captures.get(12).map(|m| m.as_str()).unwrap().to_string();
            let cs_host = captures.get(13).map(|m| m.as_str()).unwrap().to_string();
            let sc_status = captures.get(14).map(|m| m.as_str()).unwrap().to_string();
            let sc_bytes = captures.get(15).map(|m| m.as_str()).unwrap().to_string();
            let cs_bytes = captures.get(16).map(|m| m.as_str()).unwrap().to_string();
            let time_taken = captures.get(17).map(|m| m.as_str()).unwrap().to_string();
            let c_authorization_header = captures.get(18).map(|m| m.as_str()).unwrap().to_string();

            Some(LogEntryIIS {
                date_time,
                s_sitename,
                s_computername,
                s_ip,
                cs_method,
                cs_uri_stem,
                cs_uri_query,
                s_port,
                c_ip,
                cs_user_agent,
                cs_cookie,
                cs_referer,
                cs_host,
                sc_status,
                sc_bytes,
                cs_bytes,
                time_taken,
                c_authorization_header,
            })
        } else {
            None
        }
    }
}



#[no_mangle]
pub extern "C" fn flb_filter_log_iis(
    tag: *const c_char,
    tag_len: u32,
    time_sec: u32,
    time_nsec: u32,
    record: *const c_char,
    record_len: u32,
) -> *const u8 {
    let slice_tag: &[u8] = unsafe { slice::from_raw_parts(tag as *const u8, tag_len as usize) };
    let slice_record: &[u8] =
        unsafe { slice::from_raw_parts(record as *const u8, record_len as usize) };
    let mut vt: Vec<u8> = Vec::new();
    vt.write(slice_tag).expect("Unable to write");
    let vtag = str::from_utf8(&vt).unwrap();
    let v: Value = serde_json::from_slice(slice_record).unwrap();
    let dt = Utc.timestamp(time_sec as i64, time_nsec);
    let time = dt.format("%Y-%m-%dT%H:%M:%S.%9f %z").to_string();

    let input = v["log"].as_str().unwrap();
    let data = LogEntryIIS::parse_log_iis(input).unwrap();
    let message = json!({
        "message": serde_json::to_string(&data).unwrap(),
        "time": format!("{}", time),
        "tag": vtag,
        "source": "LogEntryIIS",
    });
    let buf: String = message.to_string();
    buf.as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_iis_date_time() {
        let input = "2023-07-20 17:18:54 W3SVC279 WIN-PC1 192.168.1.104 GET /api/Site/site-data qName=quww 13334 10.0.0.0 Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/114.0.0.0+Safari/537.36+Edg/114.0.1823.82 _ga=GA2.3.499592451.1685996504;+_gid=GA2.3.1209215542.1689808850;+_ga_PC23235C8Y=GS2.3.1689811012.8.0.1689811012.0.0.0 http://192.168.1.104:13334/swagger/index.html 192.168.1.104:13334 200 456 1082 3131 Bearer+token";

        match LogEntryIIS::parse_log_iis(input) {
            Some(data) => {
                assert_eq!(data.date_time, "2023-07-20 17:18:54".to_owned());
                assert_eq!(data.s_computername, "WIN-PC1".to_owned());
                assert_eq!(data.s_sitename, "W3SVC279".to_owned());
                assert_eq!(data.s_ip, "192.168.1.104".to_owned());
                assert_eq!(data.cs_method, "GET".to_owned());
                assert_eq!(data.cs_uri_stem, "/api/Site/site-data".to_owned());
                assert_eq!(data.cs_uri_query, "qName=quww".to_owned());
                assert_eq!(data.s_port, "13334".to_owned());
                assert_eq!(data.c_ip, "10.0.0.0".to_owned());
                assert_eq!(data.cs_user_agent,"Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/114.0.0.0+Safari/537.36+Edg/114.0.1823.82".to_owned());
                assert_eq!(data.cs_cookie,"_ga=GA2.3.499592451.1685996504;+_gid=GA2.3.1209215542.1689808850;+_ga_PC23235C8Y=GS2.3.1689811012.8.0.1689811012.0.0.0".to_owned());
                assert_eq!(
                    data.cs_referer,
                    "http://192.168.1.104:13334/swagger/index.html".to_owned()
                );
                assert_eq!(data.cs_host, "192.168.1.104:13334".to_owned());
                assert_eq!(data.sc_status, "200".to_owned());
                assert_eq!(data.sc_bytes, "456".to_owned());
                assert_eq!(data.cs_bytes, "1082".to_owned());
                assert_eq!(data.time_taken, "3131".to_owned());
                assert_eq!(data.c_authorization_header, "Bearer+token".to_owned());
            }
            None => println!("None"),
        }
    }
}
