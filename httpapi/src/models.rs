use serde::Deserialize;

//Requests
#[derive(Debug, Deserialize)]
pub struct RequestUserInfo {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestUpdateUser {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}
