use jwt_api::{
    configuration,
    telemetry,
    server::AppServer
};
use tokio::task::JoinError;
use std::fmt::{Debug, Display};
//use sqlx::sqlite::SqlitePoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // log
    let subscriber = telemetry::get_subscriber("info", std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // configure
    let config = configuration::get_configuration()
        .expect("Failed to read configuration.");

    // build and run
    let server = AppServer::build(config).await?;
    let server_task = tokio::spawn(server.run_until_stopped());

    tokio::select!{
        outcome = server_task => exit_report("API", outcome)
    };

    Ok(())
}

fn exit_report(
    task_name: &str,
    outcome: Result<Result<(), impl Debug + Display>, JoinError>)
{
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited.", task_name)
        }
        Ok(Err(err)) => {
            tracing::error!(
                error.cause_chain = ?err,
                error.message = %err,
                "{} failed.",
                task_name
            )
        }
        Err(err) => {
            tracing::error!(
                error.cause_chain = ?err,
                error.message = %err,
                "{}' task failed to complete.",
                task_name
            )
        }
    }
}
