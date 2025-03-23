use jwt_api::{
    configuration,
    telemetry,
    application::Application
};
use tokio::task::JoinError;
use std::fmt::{Debug, Display};
//use sqlx::sqlite::SqlitePoolOptions;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // log
    let subscriber = telemetry::get_subscriber("info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // configure
    let config = configuration::get_configuration()
        .expect("Failed to read configuration.");

    // build and run
    let application = Application::build(config).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select!{
        outcome = application_task => exit_report("API", outcome),
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
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed.",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete.",
                task_name
            )
        }
    }
}
