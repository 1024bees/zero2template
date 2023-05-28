use once_cell::sync::Lazy;
use sqlx::{sqlite::SqlitePoolOptions, Connection, Executor, SqlitePool};

use wiremock::MockServer;
use zero2template::configuration::get_configuration;
use zero2template::startup::Application;
use zero2template::telemetry::{get_subscriber, init_subscriber};

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the value
    // of `TEST_LOG` because the sink is part of the type returned by `get_subscriber`,
    // therefore they are not the same type. We could work around it, but this is the
    // most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: SqlitePool,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    Lazy::force(&TRACING);
    // Randomise configuration to ensure test isolation

    let email_server = MockServer::start().await;
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case

        // Use a random OS port
        c.application.port = 0;
        c.application.db_name = ":memory:".into();

        c
    };
    // Create and migrate the database
    let db_pool = create_db().await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    // Get the port before spawning the application
    let port = application.port();
    let address = format!("http://127.0.0.1:{}", port);
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        port,
        db_pool,
    }
}
async fn create_db() -> SqlitePool {
    let connection_pool = SqlitePoolOptions::default()
        .connect(":memory:")
        .await
        .expect("Failed to connect to sqlitedb.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
