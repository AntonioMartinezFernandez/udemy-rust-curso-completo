use chrono::NaiveDateTime; // ISO 8601, without TimeZone
use chrono::Utc;

use uuid::Uuid;

use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

/************************
 *  Create Tweet struct
************************/

// Implement necessary Diesel methods using macros
#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
// Define table name from Diesel schema
#[diesel(table_name = crate::repository::schema::tweets)]
pub struct Tweet {
    #[serde(default)]
    id: String,
    pub created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    pub fn new(msg: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().naive_utc(),
            message: msg,
        }
    }
}
