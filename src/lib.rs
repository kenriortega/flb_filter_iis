use serde::{Deserialize, Serialize};
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
    ///
    /// parse_log_iis
    /// recive this input `date time s-sitename s-computername s-ip cs-method cs-uri-stem cs-uri-query s-port c-ip cs(User-Agent) cs(Cookie) cs(Referer) cs-host sc-status sc-bytes cs-bytes time-taken c-authorization-header`
    /// return LogEntryIIS
    /// 
    pub fn parse_log_iis_w3c_custom(input: &str) -> Option<Self> {
        let elements: Vec<&str> = input.split(" ").collect();
            Some(LogEntryIIS {
                date_time: format!("{} {}",elements[0],elements[1]),
                s_sitename: elements[2].to_string(),
                s_computername: elements[3].to_string(),
                s_ip:elements[4].to_string(),
                cs_method: elements[5].to_string(),
                cs_uri_stem: elements[6].to_string(),
                cs_uri_query: elements[7].to_string(),
                s_port: elements[8].to_string(),
                c_ip: elements[9].to_string(),
                cs_user_agent: elements[10].to_string(),
                cs_cookie: elements[11].to_string(),
                cs_referer: elements[12].to_string(),
                cs_host: elements[13].to_string(),
                sc_status: elements[14].to_string(),
                sc_bytes: elements[15].to_string(),
                cs_bytes: elements[16].to_string(),
                time_taken: elements[17].to_string(),
                c_authorization_header: elements[18].to_string(),
            })
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
    let dt = Utc.timestamp_opt(time_sec as i64, time_nsec).unwrap();
    let time = dt.format("%Y-%m-%dT%H:%M:%S.%9f %z").to_string();

    let input_logs = v["log"].as_str().unwrap();
    let el: LogEntryIIS = LogEntryIIS::parse_log_iis_w3c_custom(input_logs).unwrap();
    // let elements: Vec<&str> = input_logs.split(" ").collect();


    let message = json!({
        "date": el.date_time,
        "s_sitename": el.s_sitename,
        "s_computername": el.s_computername,
        "s_ip": el.s_ip,
        "cs_method": el.cs_method,
        "cs_uri_stem": el.cs_uri_stem,
        "cs_uri_query": el.cs_uri_query,
        "s_port": el.s_port,
        "c_ip": el.c_ip,
        "cs_user_agent": el.cs_user_agent,
        "cs_cookie": el.cs_cookie,
        "cs_referer": el.cs_referer,
        "cs_host": el.cs_host,
        "sc_status": el.sc_status,
        "sc_bytes": el.sc_bytes,
        "cs_bytes": el.cs_bytes,
        "time_taken": el.time_taken,
        "c_authorization_header": el.c_authorization_header,
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
    fn test_parse_log_iis_w3c_custom() {
        let input = "2023-07-20 17:18:54 W3SVC279 WIN-PC1 192.168.1.104 GET /api/Site/site-data qName=quww 13334 10.0.0.0 Mozilla/5.0+(Windows+NT+10.0;+Win64;+x64)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/114.0.0.0+Safari/537.36+Edg/114.0.1823.82 _ga=GA2.3.499592451.1685996504;+_gid=GA2.3.1209215542.1689808850;+_ga_PC23235C8Y=GS2.3.1689811012.8.0.1689811012.0.0.0 http://192.168.1.104:13334/swagger/index.html 192.168.1.104:13334 200 456 1082 3131 Bearer+token";

        match LogEntryIIS::parse_log_iis_w3c_custom(input) {
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
