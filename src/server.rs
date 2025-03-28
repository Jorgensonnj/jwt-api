use super::{
    configuration::Settings,
    app::{
        app_service_config::full_stack_service_config,
        app_auth::jwt_extract
    }
};
use std::{io, net::TcpListener};
use actix_web::{App, HttpServer, dev::Server};
use actix_web_grants::GrantsMiddleware;
use tracing_actix_web::TracingLogger;
//use sqlx::{
//    Pool,
//    sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions}
//};

pub struct AppServer {
    port: u16,
    server: Server,
}

impl AppServer {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        // setup the port


        let address = format!(
            "{}:{}",
            configuration.application.host,
            configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();

        //// db connect if db exists
        //let connection_options = configuration.database
        //    .as_ref()
        //    .map_or_else(
        //        || SqliteConnectOptions::new_without_pgpass(),
        //        |database| database.connect_options()
        //    );

        //let database_pool = SqlitePoolOptions::new().connect_lazy_with(connection_options);

        // Wrap into data
        //let data_database_pool = Data::new(database_pool);

        // build server
        let server = HttpServer::new(move || {

            let jwt_extractor = GrantsMiddleware::with_extractor(jwt_extract);

            App::new()
                .wrap(TracingLogger::default())
                .wrap(jwt_extractor)
                .configure(full_stack_service_config)
                //.app_data(data_database_pool.clone())
            }
        )
        .listen(listener)?
        .run();

        Ok(Self {port, server})
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), io::Error> {
        self.server.await
    }
}

