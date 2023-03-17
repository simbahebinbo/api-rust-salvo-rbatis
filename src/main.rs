#[macro_use]
extern crate rbatis;
extern crate rbdc_mysql;

use salvo::logging::Logger;
use salvo::prelude::{Router, Server, TcpListener};

mod database;
mod routes;


#[tokio::main]
async fn main() {
    //database
    database::connect_db();

    // main server configuration
    // tracing_subscriber::fmt().init();
    let router = Router::new()
        .push(Router::new().hoop(Logger).handle(routes::blogs));

    let server_host = "127.0.0.1";
    let server_port = "9991";

    tracing::info!("Listening {}:{}", server_host, server_port);
    Server::new(TcpListener::bind(&format!("{}:{}", server_host, server_port)))
        .serve(router)
        .await;
}

