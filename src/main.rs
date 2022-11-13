use std::fs::File;
use std::io::Write;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, COOKIE};
// use std::env;
use std::str;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let text = args[1..args.len()].into_iter().collect();

    tts(
        "fecef076f71b6d2c3ffa3c85a1b74b2a",
        "en_us_rocket",
        "Paragraphs are the building blocks of papers. Many students define paragraphs in terms of length: a paragraph is a group of at least five sentences, a paragraph is half a page long, etc. In reality, though, the unity and coherence of ideas among sentences is what constitutes a paragraph. A paragraph is defined as “a group of sentences or a single sentence that forms a unit” (Lunsford and Connors 116). Length and appearance do not determine whether a section in a paper is a paragraph. For instance, in some styles of writing, particularly journalistic styles, a paragraph can be just one sentence long. Ultimately, a paragraph is a sentence or group of sentences that support one main idea. In this handout, we will refer to this as the “controlling idea,” because it controls what happens in the rest of the paragraph."
    )
}

fn tts(session_id: &str, voice: &str, text: &str) {
    let text = text.replace("+", "plus").replace(" ", "+").replace("&", "and");
    let mut headers = HeaderMap::new();
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        headers.insert(COOKIE, format!("sessionid={}", session_id).parse().unwrap());

    if text.len() < 200 {
        File::create("output.mp3").expect("Oh no! #3")
            .write_all(&request(headers, voice, &text)).expect("Oh no! #4");
    } else {
        let mut collection = Vec::<String>::new();
        split_string(text, &mut collection);

        let mut bytes = Vec::<u8>::new();
        for splonk in collection {
            let headers = headers.clone();
            let mut vector = request(headers, voice, &splonk);
            bytes.append(&mut vector)
        }
        
        File::create("output.mp3").expect("Oh no! #3")
            .write_all(&bytes).expect("Oh no! #4");
    }
}


fn split_string(mut string: String, collection: &mut Vec<String>) {
    let mut point: usize = 0;
    if string.len() > 200 {
        for (i, c) in string.chars().enumerate() {
            if c == '+' && i < 199 {
                point = i;
            }
        }
        collection.push(string[0..point].to_string());
        string = string[point..].to_string();
        split_string(string, collection);
    } else {
        collection.push(string);
    }
}


fn request(headers: HeaderMap, voice: &str, text: &str) -> Vec<u8> {
    let url = format!("https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?text_speaker={voice}&req_text={text}&speaker_map_type=0&aid=1233");
    let res = reqwest::blocking::Client::new().post(url)
        .headers(headers)
        .send().expect("Oh no! #1")
        .text().expect("Oh no! #2");

    let map: HashMap<String, Value> = serde_json::from_str(&res).unwrap();

    let v_str: String = map["data"]["v_str"].to_string()
        .replace("String(", "")
        .replace(")", "")
        .replace('"', "");

    return base64::decode(v_str).unwrap()
}