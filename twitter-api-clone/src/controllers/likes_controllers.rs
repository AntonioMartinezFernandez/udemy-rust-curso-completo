use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::constants;
use crate::repository::likes_repository::LikesRepository;

// GET tweet likes handler (using macro to create a service)
#[get("/tweet/{id}/likes")]
pub async fn get_tweet_likes_handler(
    path: Path<(String,)>,
    likes_repository: Data<LikesRepository>,
) -> impl Responder {
    let tweet_id = format!("{}", path.0);
    let likes = likes_repository.get_likes_by_tweet_id(tweet_id);

    return HttpResponse::Ok()
        .content_type(constants::APPLICATION_JSON)
        .json(likes);
}

// POST tweet like handler (using macro to create a service)
#[post("/tweet/{id}/like")]
pub async fn post_tweet_like_handler(
    path: Path<(String,)>,
    likes_repository: Data<LikesRepository>,
) -> impl Responder {
    let tweet_id = format!("{}", path.0);
    let new_like = likes_repository.create_like(tweet_id);

    match new_like {
        Ok(like) => HttpResponse::Ok().json(like),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// DELETE tweet like handler (using macro to create a service)
#[delete("/tweet/{id}/like")]
pub async fn delete_tweet_like_handler(
    path: Path<(String,)>,
    likes_repository: Data<LikesRepository>,
) -> impl Responder {
    // Obtain like ID from path parameter
    let like_id = format!("{}", path.0);

    // Delete like from DB
    let deleted_likes = likes_repository.delete_like_by_id(like_id);

    return HttpResponse::Accepted()
        .content_type(constants::APPLICATION_JSON)
        .json(deleted_likes);
}
