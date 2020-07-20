use crate::models;
use actix_web::{post, web, Error as AWError, HttpResponse, Responder};
use domain::traits::UserRepo;
use domain::user::UserPermission;
use std::ops::Deref;

// error type
// #[derive(thiserror::Error, Debug)]
// enum Error {
//     #[error("No basic auth provided")]
//     NoBasicAuth,

//     #[error("UsersDomain error: {0}")]
//     UsersDomain(#[from] UsersError),

//     #[error("Server error: {0}")]
//     Message(String),
// }

// structure
#[derive(Debug)]
pub struct Auth<T> {
    auth: String,
    repo: T,
}

impl<T> Auth<T> {
    pub fn new(author: &str, repo: T) -> Auth<T> {
        Auth {
            auth: author.to_owned(),
            repo,
        }
    }
}

// deref for class Auth
impl<T> Deref for Auth<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}

pub fn routes<T: 'static>(author: &str, user_repo: T) -> impl FnOnce(&mut web::ServiceConfig) {
    // move env paramter in closure, need to copy one own paramter
    let author = author.to_owned();
    move |config: &mut web::ServiceConfig| {
        config
            .data(Auth::new(&author, user_repo))
            .service(web::scope("auth").service(user_info).service(update_user));
    }
}

// api
#[post("/userInfo")]
async fn user_info(
    auth: web::Data<Auth<UserPermission>>,
    request: web::Json<models::RequestUserInfo>,
) -> impl Responder {
    log::info!("request: {:?}", request);
    auth.get_user_full_name(&request.email).map_or_else(
        |_| HttpResponse::InternalServerError().finish(),
        |user| HttpResponse::Ok().json(user),
    )

    // for other way:
    // Result<HttpResponse, AWError>
    // auth.get_user_full_name("neil")
    // .map(|user| Ok(HttpResponse::Ok().json(user)))
    // .map_err(|e| HttpResponse::InternalServerError().finish())?

    /*---Option<> ---*/
    // let email = if let Some(v) = requset.email {
    //     v
    // } else {
    //     "".to_owned()
    // };
}

#[post("/updateUser")]
async fn update_user(
    auth: web::Data<Auth<UserPermission>>,
    request: web::Json<models::RequestUpdateUser>,
) -> impl Responder {
    log::info!("request: {:?}", request);
    auth.upate_user_name(&request.email, &request.firstname, &request.lastname)
        .map_or_else(
            |_| HttpResponse::InternalServerError().finish(),
            |user| HttpResponse::Ok().json(user),
        )
}
