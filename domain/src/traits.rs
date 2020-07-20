// interface
use crate::user::{Error, Users};
use uuid::Uuid;

pub trait UserRepo {
    fn get_user_full_name(&self, email: &str) -> Result<Users, Error>;
    fn upate_user_name(&self, email: &str, firstname: &str, lastname: &str) -> Result<Uuid, Error>;
}
