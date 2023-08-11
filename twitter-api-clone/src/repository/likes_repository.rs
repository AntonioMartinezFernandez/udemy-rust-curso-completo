use std::fmt::Error;

use diesel::prelude::*;
use diesel::QueryDsl;

use crate::models::like::Like;
use crate::repository::schema::likes::dsl::*;

use super::database_connection_pool;

pub struct LikesRepository {
    pool: database_connection_pool::DBPool,
}

impl LikesRepository {
    pub fn new(pool: database_connection_pool::DBPool) -> Self {
        LikesRepository { pool }
    }

    pub fn create_like(&self, t_id: String) -> Result<Like, Error> {
        let like = Like::new(t_id);

        diesel::insert_into(likes)
            .values(&like)
            .execute(&mut self.pool.get().unwrap())
            .expect("error creating new like");

        Ok(like)
    }

    pub fn get_likes_by_tweet_id(&self, t_id: String) -> Option<Vec<Like>> {
        let like = likes
            .filter(tweet_id.eq(t_id))
            .load::<Like>(&mut self.pool.get().unwrap())
            .expect("error loading tweet likes");

        Some(like)
    }

    pub fn delete_like_by_id(&self, like_id: String) -> Option<usize> {
        let count = diesel::delete(likes.find(like_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("error deleting like by id");

        Some(count)
    }
}
