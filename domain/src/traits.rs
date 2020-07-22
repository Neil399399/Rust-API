// interface
use crate::user::{Error, Users};
use diesel::prelude::*;
use uuid::Uuid;

pub trait UserRepo {
    fn get_user_full_name(&self, conn: &PgConnection, email: &str) -> Result<Users, Error>;
    fn upate_user_name(
        &self,
        conn: &PgConnection,
        email: &str,
        firstname: &str,
        lastname: &str,
    ) -> Result<Uuid, Error>;
}
