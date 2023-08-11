use diesel::r2d2;
use diesel::PgConnection;

pub type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn new(database_url: String) -> DBPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool: DBPool = r2d2::Pool::builder()
        .build(manager)
        .expect("error creating DB pool");

    pool
}
