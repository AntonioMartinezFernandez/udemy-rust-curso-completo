extern crate dotenv;

use dotenv::dotenv;
use std::env;

use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;

mod controllers;
mod models;
mod repository;

mod constants;

// use 'actix-web' macro for main (similar to 'tokio' macro for main)
#[actix_web::main]
async fn main() {
    /*******************************************
     *
     *          TWITTER API CLONE
     *      USING ACTIX WEB FRAMEWORK
     *
     *******************************************/

    // Enable DEBUG logging with 'env_logger'
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("not found DATABASE_URL as env variable");

    // Create database connection pool
    let db_pool = repository::database_connection_pool::new(database_url);

    // Create Repositories
    let tweets_repository = repository::tweets_repository::TweetsRepository::new(db_pool.clone());
    let likes_repository = repository::likes_repository::LikesRepository::new(db_pool.clone());

    // Data extractors
    let tweets_data = web::Data::new(tweets_repository);
    let likes_data = web::Data::new(likes_repository);

    // Create server
    let server = HttpServer::new(move || {
        App::new()
            // Inject repositories
            .app_data(tweets_data.clone())
            .app_data(likes_data.clone())
            // Define routes/services
            .route("/", web::get().to(get_index_handler))
            .service(controllers::tweets_controllers::get_tweets_handler)
            .service(controllers::tweets_controllers::get_tweet_handler)
            .service(controllers::tweets_controllers::post_tweet_handler)
            .service(controllers::tweets_controllers::delete_tweet_handler)
            .service(controllers::likes_controllers::get_tweet_likes_handler)
            .service(controllers::likes_controllers::post_tweet_like_handler)
            .service(controllers::likes_controllers::delete_tweet_like_handler)
    })
    .bind(("127.0.0.1", 8000))
    .unwrap();

    // Start server
    server.run().await.unwrap();
}

// GET index handler (using direct route)
async fn get_index_handler() -> impl Responder {
    return HttpResponse::Ok().body("Hi from Actix Web!");
}
