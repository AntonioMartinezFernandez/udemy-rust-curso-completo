use chrono::NaiveDateTime; // ISO 8601, without TimeZone
use chrono::Utc;

use uuid::Uuid;

use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

/************************
 *  Create Likes struct
************************/

// Implement necessary Diesel methods using macros
#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
// Define table name from Diesel schema
#[diesel(table_name = crate::repository::schema::likes)]
pub struct Like {
    #[serde(default)]
    id: String,
    created_at: NaiveDateTime,
    tweet_id: String,
}

impl Like {
    pub fn new(t_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().naive_utc(),
            tweet_id: t_id,
        }
    }
}
