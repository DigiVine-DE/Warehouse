use diesel_ext::db_backend::PoolBackend;
use digivine_api::{config, init_log, start_server};
#[macro_use]
extern crate log;
extern crate simplelog;

#[tokio::main]
async fn main() {
    let conf = config::Settings::new().expect("could not build config");
    init_log(&conf);
    match start_server::<PoolBackend>(&conf, None).await {
        Ok(()) => {}
        Err(e) => error!("{}", e),
    };
}
