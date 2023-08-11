use serde::Deserialize;

#[derive(Deserialize)]
pub struct TweetDto {
    pub message: String,
}
