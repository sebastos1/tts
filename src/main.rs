use std::str;
use std::fs::File;
use std::io::Write;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, COOKIE};

fn main() {
    let content = read_file("test.txt");

    tts(
        "fecef076f71b6d2c3ffa3c85a1b74b2a",
        "en_male_pirate",
        &content
    )
}

fn tts(session_id: &str, voice: &str, text: &str) {
    let text = text.replace("+", "plus").replace(" ", "+").replace("&", "and").replace("\n", "+");
    let mut headers = HeaderMap::new();
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        headers.insert(COOKIE, format!("sessionid={}", session_id).parse().unwrap());

    if text.len() < 200 {
        File::create("output.mp3").unwrap()
            .write_all(&request(headers, voice, &text)).unwrap()
    } else {
        let mut collection = Vec::<String>::new();
        split_string(text, &mut collection);

        let mut bytes = Vec::<u8>::new();
        for splonk in collection {
            let headers = headers.clone();
            let mut vector = request(headers, voice, &splonk);
            bytes.append(&mut vector)
        }
        File::create("output.mp3").unwrap()
            .write_all(&bytes).unwrap()
    }
}

fn read_file(filename: &str) -> String {
    return std::fs::read_to_string(filename).unwrap()
}

fn split_string(mut string: String, collection: &mut Vec<String>) {
    let mut point: usize = 0;
    if string.len() > 200 {
        for (i, c) in string.chars().enumerate() {
            if c == '+' && i < 199 {
                point = i
            }
        }
        collection.push(string[0..point].to_string());
        string = string[point..].to_string();
        split_string(string, collection)
    } else {
        collection.push(string)
    }
}

fn request(headers: HeaderMap, voice: &str, text: &str) -> Vec<u8> {
    let url = format!("https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?text_speaker={voice}&req_text={text}&speaker_map_type=0&aid=1233");
    let res = reqwest::blocking::Client::new().post(url)
        .headers(headers)
        .send().unwrap()
        .text().unwrap();
    let map: HashMap<String, Value> = serde_json::from_str(&res).unwrap();
    return base64::decode(map["data"]["v_str"]
        .to_string()
        .replace("String(", "")
        .replace(&[')', '"'], ""))
        .unwrap()
}