use crate::PgPool;
use ansi_term::Colour;
use chrono::Utc;
use diesel::prelude::*;
use std::io::Write;

pub fn init_logger(pattern: &str) {
    let mut builder = env_logger::Builder::new();
    if let Ok(lvl) = std::env::var("RUST_LOG") {
        builder.parse_filters(&lvl);
    }
    builder.parse_filters(pattern);

    builder.format(move |buf, record| {
        let time_now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let output = format!("{} {}", Colour::Black.bold().paint(time_now), record.args(),);
        writeln!(buf, "{}", output)
    });

    if builder.try_init().is_err() {
        log::info!("Global logger already initialized.  Skipping");
    }
}

pub trait ResultLogger {
    fn debug_log(self) -> Self;
}

use std::fmt::Debug;
impl<T: Debug, E: Debug> ResultLogger for Result<T, E> {
    fn debug_log(self) -> Self {
        log::debug!("{:?}", self);
        self
    }
}

pub fn create_pool(db_url: &str) -> PgPool {
    use diesel::r2d2::ConnectionManager;

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Postgres connection pooling failed")
}
