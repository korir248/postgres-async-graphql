use async_graphql::{Object, Result};
use diesel::RunQueryDsl;

use crate::{create_pool, models::User};

#[derive(Default)]
pub struct GetAllUsers;

#[Object]
impl GetAllUsers {
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        use crate::schema::users::dsl::users;

        let mut conn = create_pool().get()?;

        let res: Vec<User> = users.load(&mut conn)?;

        Ok(res)
    }
}
