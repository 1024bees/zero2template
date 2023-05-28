use once_cell::sync::Lazy;
{% if sqlx -%}
use sqlx::SqlitePool;
{% endif -%}


use {{ crate_name }}::configuration::get_configuration;
use {{ crate_name }}::startup::Application;
use {{ crate_name }}::telemetry::{get_subscriber, init_subscriber};


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
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    Lazy::force(&TRACING);
    // Randomise configuration to ensure test isolation

    
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case

        // Use a random OS port
        c.application.port = 0;
        {% if sqlx -%}

        c.application.db_name = ":memory:".into();
        {% endif -%}


        c
    };
    // Create and migrate the database
    

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    // Get the port before spawning the application
    let port = application.port();
    {% if sqlx -%}
    run_migrations(application.pool()).await;
    {% endif -%}
    let address = format!("http://127.0.0.1:{}", port);
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        port
    }
}

{% if sqlx -%}
async fn run_migrations(pool : &SqlitePool)  {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to migrate the database");
   
}
{% endif -%}
