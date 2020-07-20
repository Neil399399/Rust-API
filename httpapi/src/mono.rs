use crate::auth;
use crate::utils;
use actix_cors::Cors;
use actix_rt::System; //thread
use actix_web::{middleware, web, App, HttpServer};
use domain::user::UserPermission;

pub fn mono_server() -> std::io::Result<()> {
    // init logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    utils::init_logger("debug");

    System::new("server").block_on(async {
        // use port 80 will require sudo to run
        let bind_address = "127.0.0.1:3031";
        let user_repo = UserPermission::new();
        // run future
        HttpServer::new(move || {
            App::new()
                // enable logger
                .wrap(middleware::Logger::default())
                // cors
                .wrap(
                    Cors::new()
                        // #NOTE for test
                        .allowed_origin("http://localhost")
                        .allowed_origin("http://localhost:3000")
                        .allowed_methods(vec!["GET", "POST", "HEAD", "PUT", "PATCH", "DELETE"])
                        .supports_credentials()
                        .finish(),
                )
                // heathlz
                .service(web::resource("/heathlz").route(web::get().to(|| async { "healthz" })))
                .service(web::scope("/v1").configure(auth::routes("neil", user_repo.clone())))
        })
        .bind(bind_address)
        .unwrap_or_else(|thing| panic!("Could not bind server to address {:?}", thing))
        .run()
        .await
    })
}
