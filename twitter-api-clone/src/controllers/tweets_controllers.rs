use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::constants;

use crate::controllers::tweet_dto;

use crate::repository::tweets_repository::TweetsRepository;

// GET tweets handler (using macro to create a service)
#[get("/tweets")]
pub async fn get_tweets_handler(tweets_repository: Data<TweetsRepository>) -> impl Responder {
    // Get tweets from DB
    let tweets = tweets_repository.get_tweets();

    // Response
    HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweets)
}

// POST tweet handler (using macro to create a service)
#[post("/tweet")]
pub async fn post_tweet_handler(
    req_body: web::Json<tweet_dto::TweetDto>,
    tweets_repository: Data<TweetsRepository>,
) -> impl Responder {
    // Obtain tweet message from Tweet DTO
    let tweet_msg = &req_body.message;

    // Save tweet in DB
    let saved_tweet = tweets_repository.create_tweet(tweet_msg.clone());

    // Response
    match saved_tweet {
        Ok(tweet) => HttpResponse::Ok().json(tweet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// GET tweet handler (using macro to create a service)
#[get("/tweet/{id}")]
pub async fn get_tweet_handler(
    path: Path<(String,)>,
    tweets_repository: Data<TweetsRepository>,
) -> impl Responder {
    // Obtain tweet ID from path parameter
    let tweet_id = format!("{}", path.0);

    // Get tweet from DB
    let tweet = tweets_repository.get_tweet_by_id(tweet_id);

    return HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(tweet);
}

// DELETE tweet handler (using macro to create a service)
#[delete("/tweet/{id}")]
pub async fn delete_tweet_handler(
    path: Path<(String,)>,
    tweets_repository: Data<TweetsRepository>,
) -> impl Responder {
    // Obtain tweet ID from path parameter
    let tweet_id = format!("{}", path.0);

    // Delete tweet from DB
    let deleted_tweets = tweets_repository.delete_tweet_by_id(tweet_id);

    return HttpResponse::Accepted()
        .content_type(constants::APPLICATION_JSON)
        .json(deleted_tweets);
}
