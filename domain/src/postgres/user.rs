use crate::schema::example_table::dsl::*;
use crate::user::Users;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn select_user_by_email(conn: &PgConnection, target_email: &str) -> Result<Users, Error> {
    example_table
        .filter(email.eq(&target_email))
        .first::<Users>(conn)
}

pub fn update_user_data_by_email(
    conn: &PgConnection,
    user_email: &str,
    first_name: &str,
    last_name: &str,
) -> Result<Uuid, Error> {
    diesel::update(example_table.filter(email.eq(user_email)))
        .set((firstname.eq(first_name), lastname.eq(last_name)))
        // do execute(conn) and return object
        .get_result::<Users>(conn)
        .map(|res| res.id)
}
