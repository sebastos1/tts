use std::fs::File;
use std::io::Write;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, COOKIE};

fn main() {
    tts(
        "fecef076f71b6d2c3ffa3c85a1b74b2a",
        "en_male_m03_lobby",
        "hello world"
    )
}

fn tts(session_id: &str, voice: &str, text: &str) {
    let text = text.replace("+", "plus").replace(" ", "+").replace("&", "and");
    let url = format!("https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?text_speaker={voice}&req_text={text}&speaker_map_type=0&aid=1233");
    let mut headers = HeaderMap::new();
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
    let cookie = format!("sessionid={}", session_id);
        headers.insert(COOKIE, cookie.parse().unwrap());

    let res = reqwest::blocking::Client::new().post(url)
        .headers(headers)
        .send().expect("Oh no! #1")
        .text().expect("Oh no! #2");

    let map: HashMap<String, Value> = serde_json::from_str(&res).unwrap();

    let test: String = map["data"]["v_str"].to_string()
        .replace("String(", "")
        .replace(")", "")
        .replace('"', "");

    let vstr = base64::decode(test).unwrap();

    let mut file = File::create("output.mp3").expect("Oh no! #3");
    file.write_all(&vstr).expect("Oh no! #4");
}