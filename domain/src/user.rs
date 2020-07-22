use crate::postgres::user as userDB;
use crate::traits::UserRepo;
use diesel::prelude::*;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("UserNotFound error: {0}")]
    UserNotFound(String),

    #[error("Server error: {0}")]
    Message(String),
}

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

impl Users {
    pub fn new_users_row(email: &str, firstname: &str, lastname: &str) -> Self {
        Users {
            id: Uuid::new_v4(),
            email: email.to_owned(),
            firstname: firstname.to_owned(),
            lastname: lastname.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserPermission {
    mock_users: Vec<Users>,
}

impl UserPermission {
    pub fn new() -> Self {
        UserPermission {
            mock_users: mock_user(),
        }
    }
}

impl UserRepo for UserPermission {
    fn get_user_full_name(&self, conn: &PgConnection, email: &str) -> Result<Users, Error> {
        userDB::select_user_by_email(conn, email).map_err(|_| Error::UserNotFound(email.to_owned()))
    }

    fn upate_user_name(
        &self,
        conn: &PgConnection,
        email: &str,
        firstname: &str,
        lastname: &str,
    ) -> Result<Uuid, Error> {
        userDB::update_user_data_by_email(conn, email, firstname, lastname)
            .map_err(|_| Error::UserNotFound(email.to_owned()))
    }
}

// for mock
fn mock_user() -> Vec<Users> {
    let user_ids = vec![
        "41379e1f-5979-448e-b693-6cf5102e1cb4",
        "3b646b9f-a9a6-42c9-ae71-ff60583441a3",
        "3632875b-95c7-4ea6-bf79-3e3c8af41e8d",
    ];
    let names = vec!["neil", "doppelganger_1", "Doppelganger_2"];
    let email_subfix = "@aetheras.io";
    let lastname = "void";

    names
        .into_iter()
        .enumerate()
        .map(|(idx, name)| {
            let mut email = name.to_owned();
            email.push_str(email_subfix);
            Users {
                id: Uuid::parse_str(user_ids[idx]).unwrap(),
                email,
                firstname: name.to_owned(),
                lastname: lastname.to_owned(),
            }
        })
        .collect()
}
