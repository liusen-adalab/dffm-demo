use crate::schema::users;
use diesel::{
    expression_methods::ExpressionMethods, insert_into, Insertable, QueryDsl, QueryResult,
    Queryable,
};
use diesel_async::RunQueryDsl;
use returns::*;
use serde::Serialize;

use super::DBConn;

mod returns {
    use crate::schema::users::dsl::*;

    pub static USER: (id, name, tel_number) = (id, name, tel_number);
}

#[derive(Queryable, Serialize)]
#[diesel(table_name=users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub tel_number: String,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct UserInsert {
    pub name: String,
    pub tel_number: String,
}

pub async fn create(db: &mut DBConn, item: &UserInsert) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    insert_into(users)
        .values(item)
        .returning(USER)
        .get_result::<User>(db)
        .await
}

pub async fn read_by_id(db: &mut DBConn, item_id: i32) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(item_id))
        .select(USER)
        .first::<User>(db)
        .await
}
