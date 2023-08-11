use std::fmt::Error;

// use chrono::Utc;
use diesel::prelude::*;
use diesel::QueryDsl;

use crate::models::tweet::Tweet;
use crate::repository::schema::tweets::dsl::*;

use super::database_connection_pool;

pub struct TweetsRepository {
    pool: database_connection_pool::DBPool,
}

impl TweetsRepository {
    pub fn new(pool: database_connection_pool::DBPool) -> Self {
        TweetsRepository { pool }
    }

    pub fn get_tweets(&self) -> Vec<Tweet> {
        tweets
            .load::<Tweet>(&mut self.pool.get().unwrap())
            .expect("error loading all tweets")
    }

    pub fn create_tweet(&self, tweet_content: String) -> Result<Tweet, Error> {
        let tweet = Tweet::new(tweet_content);

        diesel::insert_into(tweets)
            .values(&tweet)
            .execute(&mut self.pool.get().unwrap())
            .expect("error creating new tweet");

        Ok(tweet)
    }

    pub fn get_tweet_by_id(&self, tweet_id: String) -> Option<Tweet> {
        let tweet = tweets
            .find(tweet_id)
            .get_result::<Tweet>(&mut self.pool.get().unwrap())
            .expect("error loading tweet by id");

        Some(tweet)
    }

    pub fn delete_tweet_by_id(&self, tweet_id: String) -> Option<usize> {
        // let count = diesel::delete(tweets.find(tweet_id))
        //     .execute(&mut self.pool.get().unwrap())
        //     .expect("error deleting tweet by id");

        let count = diesel::delete(tweets.find(tweet_id)).execute(&mut self.pool.get().unwrap());

        // ! Prevent to send error to the client when the tweet have likes and can't be deleted
        match count {
            Ok(deleted_tweets) => Some(deleted_tweets),
            Err(_) => None,
        }
    }

    /*
    pub fn update_tweet_by_id(&self, tweet_id: String, mut tweet: Tweet) -> Option<Tweet> {
        tweet.created_at = Utc::now().naive_utc();
        let tweet = diesel::update(tweets.find(tweet_id))
            .set(&tweet)
            .get_result::<Tweet>(&mut self.pool.get().unwrap())
            .expect("error updating tweet by id");
        Some(tweet)
    }
    */
}
