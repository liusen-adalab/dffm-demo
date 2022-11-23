pub mod users;

use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use once_cell::sync::OnceCell;

pub type DBConnPool = Pool<AsyncPgConnection>;
pub type DBConn = Object<AsyncPgConnection>;
static DB_POOL: OnceCell<DBConnPool> = OnceCell::new();

pub fn pool_init() {
    DB_POOL.get_or_init(|| {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable expected.");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();
        pool
    });
}

pub async fn db_conn() -> DBConn {
    pool_init();
    DB_POOL.get().unwrap().get().await.unwrap()
}
