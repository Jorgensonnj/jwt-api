use super::{
    configuration::Settings,
    app::{
        app_service_config::full_stack_service_config,
        app_auth::extract_jwt_permissions,
        shared::models::response_models::{AppResponse, AppResponseStatus}
    }
};
use std::{io, net::TcpListener};
use actix_web::{
    dev::{Server, ServiceResponse},
    error::ErrorInternalServerError,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web::Data, App, HttpResponse, HttpServer, Result
};
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
    pub async fn build(config: Settings) -> Result<Self, io::Error> {
        // setup the port
        let address = format!(
            "{}:{}",
            config.application.host,
            config.application.port
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

        let data_config = Data::new(config);

        // build server
        let server = HttpServer::new(move || {

            let auth_handler = GrantsMiddleware::with_extractor(extract_jwt_permissions);
            let error_handlers = ErrorHandlers::new().default_handler(default_error_handler);

            App::new()
                .wrap(auth_handler)
                .wrap(error_handlers)
                .wrap(TracingLogger::default())
                .configure(full_stack_service_config)
                .app_data(data_config.clone())
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

fn default_error_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {

    // break apart service response
    let (req, res) = res.into_parts();

    let status_code = res.status();
    let error = res.error()
        .unwrap_or(&ErrorInternalServerError("Unable to get response error."))
        .to_string();

    // buil app's response
    let error_res = AppResponse {
        status: AppResponseStatus::Error,
        status_code: status_code.as_u16(),
        message: error
    };

    let new_res = HttpResponse::build(status_code).json(error_res);

    let res = ServiceResponse::new(req, new_res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
