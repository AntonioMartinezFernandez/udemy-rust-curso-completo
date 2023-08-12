use gloo_net::{http::Request, Error};
use serde::Deserialize;

pub async fn youtube_search(search_text: String) -> Result<VideoItem, Error> {
    // Log the text to search in the browser console
    let text_to_be_logged = search_text.clone();
    web_sys::console::log_1(&text_to_be_logged.into());

    let youtube_api_key = crate::env::YOUTUBE_API_KEY;

    let yt_api_query_url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}&key={}",
        search_text, youtube_api_key
    );

    // let mut auth = String::from("Bearer ");
    // auth.push_str(crate::env::YOUTUBE_API_KEY);

    let response_from_youtube_api = Request::get(&yt_api_query_url)
        // .header("Authorization", &auth)
        .send()
        .await?;

    let search_result = response_from_youtube_api.json::<SearchResult>().await?;

    let first_video_result = search_result.items.first();

    let empty_video = build_empty_video();

    let video = match first_video_result {
        Some(first_video) => first_video,
        None => &empty_video,
    };

    Ok(video.clone())
}

// Auxiliar methods
fn build_empty_video() -> VideoItem {
    VideoItem {
        id: VideoItemId {
            kind: String::from(""),
            video_id: String::from(""),
        },
        snippet: VideoSnippet {
            title: String::from(""),
            description: String::from(""),
        },
    }
}

// API response struct
#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub region_code: String,
    pub items: Vec<VideoItem>,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoSnippet,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoItemId {
    pub kind: String,
    pub video_id: String,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VideoSnippet {
    pub title: String,
    pub description: String,
}
