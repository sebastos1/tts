use reqwest::header::*;


fn main() {
    let client = reqwest::Client::new();
    let session_id = "fecef076f71b6d2c3ffa3c85a1b74b2a".to_string();
    let voice = "en_us_rocket".to_string();
    let text ="testing string".to_string();

    // build url
    // "https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?text_speaker={text_speaker}&req_text={req_text}&speaker_map_type=0&aid=1233"
    // ?text_speaker={text_speaker}
    // &req_text={req_text}

    let mut url =
        "https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?text_speaker="
        .push_str(voice).push_str("&req_text=").push_str(text)
        .push_str("&speaker_map_type=0&aid=1233");

    let res = client
        .post(url)
        .header("session_id".to_string(), session_id)
        .send()
        .await?;
}