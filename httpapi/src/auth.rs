use crate::models;
use crate::PgPool;
use actix_web::{post, web, HttpResponse, Responder};
use domain::traits::UserRepo;
use domain::user::{Error, UserPermission, Users};
use std::ops::Deref;

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
    pg_pool: web::Data<PgPool>,
) -> impl Responder {
    log::info!("request: {:?}", request);
    web::block(move || {
        let conn = &pg_pool.get().map_err(|e| Error::Message(e.to_string()))?;
        let user = auth.get_user_full_name(conn, &request.email)?;
        Ok::<Users, Error>(user)
    })
    .await
    .map_or_else(
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
    pg_pool: web::Data<PgPool>,
) -> impl Responder {
    log::info!("request: {:?}", request);
    web::block(move || {
        let conn = &pg_pool.get().map_err(|e| Error::Message(e.to_string()))?;
        auth.upate_user_name(conn, &request.email, &request.firstname, &request.lastname)
    })
    .await
    .map_or_else(
        |_| HttpResponse::InternalServerError().finish(),
        |user_uuid| HttpResponse::Ok().json(user_uuid),
    )
}
